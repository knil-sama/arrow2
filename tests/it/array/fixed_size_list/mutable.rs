use arrow2::array::*;
use arrow2::datatypes::{DataType, Field};

#[test]
fn primitive() {
    let data = vec![
        Some(vec![Some(1i32), Some(2), Some(3)]),
        Some(vec![None, None, None]),
        Some(vec![Some(4), None, Some(6)]),
    ];

    let mut list = MutableFixedSizeListArray::new(MutablePrimitiveArray::<i32>::new(), 3);
    list.try_extend(data).unwrap();
    let list: FixedSizeListArray = list.into();

    let a = list.value(0);
    let a = a.as_any().downcast_ref::<Int32Array>().unwrap();

    let expected = Int32Array::from(vec![Some(1i32), Some(2), Some(3)]);
    assert_eq!(a, &expected);

    let a = list.value(1);
    let a = a.as_any().downcast_ref::<Int32Array>().unwrap();

    let expected = Int32Array::from(vec![None, None, None]);
    assert_eq!(a, &expected)
}

#[test]
fn new_with_field() {
    let data = vec![
        Some(vec![Some(1i32), Some(2), Some(3)]),
        Some(vec![None, None, None]),
        Some(vec![Some(4), None, Some(6)]),
    ];

    let mut list = MutableFixedSizeListArray::new_with_field(
        MutablePrimitiveArray::<i32>::new(),
        "custom_items",
        false,
        3,
    );
    list.try_extend(data).unwrap();
    let list: FixedSizeListArray = list.into();

    assert_eq!(
        list.data_type(),
        &DataType::FixedSizeList(
            Box::new(Field::new("custom_items", DataType::Int32, false)),
            3
        )
    );

    let a = list.value(0);
    let a = a.as_any().downcast_ref::<Int32Array>().unwrap();

    let expected = Int32Array::from(vec![Some(1i32), Some(2), Some(3)]);
    assert_eq!(a, &expected);

    let a = list.value(1);
    let a = a.as_any().downcast_ref::<Int32Array>().unwrap();

    let expected = Int32Array::from(vec![None, None, None]);
    assert_eq!(a, &expected)
}
