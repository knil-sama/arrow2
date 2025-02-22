//! APIs to read from Parquet format.
mod binary;
mod boolean;
mod dictionary;
mod fixed_size_binary;
mod nested;
mod nested_utils;
mod null;
mod primitive;
mod simple;
mod struct_;
mod utils;

use parquet2::read::get_page_iterator as _get_page_iterator;
use parquet2::schema::types::PrimitiveType;

use crate::{
    array::{Array, DictionaryKey, FixedSizeListArray, ListArray},
    datatypes::{DataType, Field, IntervalUnit},
    error::Result,
};

use self::nested_utils::{InitNested, NestedArrayIter, NestedState};
use simple::page_iter_to_arrays;

use super::*;

/// Creates a new iterator of compressed pages.
pub fn get_page_iterator<R: Read + Seek>(
    column_metadata: &ColumnChunkMetaData,
    reader: R,
    pages_filter: Option<PageFilter>,
    buffer: Vec<u8>,
    max_header_size: usize,
) -> Result<PageReader<R>> {
    Ok(_get_page_iterator(
        column_metadata,
        reader,
        pages_filter,
        buffer,
        max_header_size,
    )?)
}

fn create_list(
    data_type: DataType,
    nested: &mut NestedState,
    values: Box<dyn Array>,
) -> Box<dyn Array> {
    let (mut offsets, validity) = nested.nested.pop().unwrap().inner();
    match data_type.to_logical_type() {
        DataType::List(_) => {
            offsets.push(values.len() as i64);

            let offsets = offsets.iter().map(|x| *x as i32).collect::<Vec<_>>();
            Box::new(ListArray::<i32>::new(
                data_type,
                offsets.into(),
                values,
                validity.and_then(|x| x.into()),
            ))
        }
        DataType::LargeList(_) => {
            offsets.push(values.len() as i64);

            Box::new(ListArray::<i64>::new(
                data_type,
                offsets.into(),
                values,
                validity.and_then(|x| x.into()),
            ))
        }
        DataType::FixedSizeList(_, _) => Box::new(FixedSizeListArray::new(
            data_type,
            values,
            validity.and_then(|x| x.into()),
        )),
        _ => unreachable!(),
    }
}

fn is_primitive(data_type: &DataType) -> bool {
    matches!(
        data_type.to_physical_type(),
        crate::datatypes::PhysicalType::Primitive(_)
            | crate::datatypes::PhysicalType::Null
            | crate::datatypes::PhysicalType::Boolean
            | crate::datatypes::PhysicalType::Utf8
            | crate::datatypes::PhysicalType::LargeUtf8
            | crate::datatypes::PhysicalType::Binary
            | crate::datatypes::PhysicalType::LargeBinary
            | crate::datatypes::PhysicalType::FixedSizeBinary
            | crate::datatypes::PhysicalType::Dictionary(_)
    )
}

fn columns_to_iter_recursive<'a, I: 'a>(
    mut columns: Vec<I>,
    mut types: Vec<&PrimitiveType>,
    field: Field,
    init: Vec<InitNested>,
    num_rows: usize,
    chunk_size: Option<usize>,
) -> Result<NestedArrayIter<'a>>
where
    I: Pages,
{
    if init.is_empty() && is_primitive(&field.data_type) {
        return Ok(Box::new(
            page_iter_to_arrays(
                columns.pop().unwrap(),
                types.pop().unwrap(),
                field.data_type,
                chunk_size,
                num_rows,
            )?
            .map(|x| Ok((NestedState::new(vec![]), x?))),
        ));
    }

    nested::columns_to_iter_recursive(columns, types, field, init, num_rows, chunk_size)
}

/// Returns the number of (parquet) columns that a [`DataType`] contains.
fn n_columns(data_type: &DataType) -> usize {
    use crate::datatypes::PhysicalType::*;
    match data_type.to_physical_type() {
        Null | Boolean | Primitive(_) | Binary | FixedSizeBinary | LargeBinary | Utf8
        | Dictionary(_) | LargeUtf8 => 1,
        List | FixedSizeList | LargeList => {
            let a = data_type.to_logical_type();
            if let DataType::List(inner) = a {
                n_columns(&inner.data_type)
            } else if let DataType::LargeList(inner) = a {
                n_columns(&inner.data_type)
            } else if let DataType::FixedSizeList(inner, _) = a {
                n_columns(&inner.data_type)
            } else {
                unreachable!()
            }
        }
        Struct => {
            if let DataType::Struct(fields) = data_type.to_logical_type() {
                fields.iter().map(|inner| n_columns(&inner.data_type)).sum()
            } else {
                unreachable!()
            }
        }
        _ => todo!(),
    }
}

/// An iterator adapter that maps multiple iterators of [`Pages`] into an iterator of [`Array`]s.
///
/// For a non-nested datatypes such as [`DataType::Int32`], this function requires a single element in `columns` and `types`.
/// For nested types, `columns` must be composed by all parquet columns with associated types `types`.
///
/// The arrays are guaranteed to be at most of size `chunk_size` and data type `field.data_type`.
pub fn column_iter_to_arrays<'a, I: 'a>(
    columns: Vec<I>,
    types: Vec<&PrimitiveType>,
    field: Field,
    chunk_size: Option<usize>,
    num_rows: usize,
) -> Result<ArrayIter<'a>>
where
    I: Pages,
{
    Ok(Box::new(
        columns_to_iter_recursive(columns, types, field, vec![], num_rows, chunk_size)?
            .map(|x| x.map(|x| x.1)),
    ))
}
