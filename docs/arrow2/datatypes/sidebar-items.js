initSidebarItems({"enum":[["DataType","The set of supported logical types. Each variant uniquely identifies a logical type, which define specific semantics to the data (e.g. how it should be represented). Each variant has a corresponding [`PhysicalType`], obtained via [`DataType::to_physical_type`], which declares the in-memory representation of data. The [`DataType::Extension`] is special in that it augments a [`DataType`] with metadata to support custom types. Use `to_logical_type` to desugar such type and return its correspoding logical type."],["IntegerType","the set of valid indices types of a dictionary-encoded Array. Each type corresponds to a variant of [`crate::array::DictionaryArray`]."],["IntervalUnit","Interval units defined in Arrow"],["PhysicalType","The set of physical types: unique in-memory representations of an Arrow array. A physical type has a one-to-many relationship with a [`crate::datatypes::DataType`] and a one-to-one mapping to each struct in this crate that implements [`crate::array::Array`]."],["PrimitiveType","The set of all (physical) primitive types. Each type corresponds to a variant of [`crate::array::PrimitiveArray`]."],["TimeUnit","The time units defined in Arrow."],["UnionMode","Mode of [`DataType::Union`]"]],"fn":[["get_extension","support get extension for metadata"]],"struct":[["Field","A logical [`DataType`] and its associated metadata per Arrow specification"],["Schema","An ordered sequence of [`Field`] with optional metadata."]],"type":[["Extension","typedef fpr [Option<(String, Option)>]."],["Metadata","typedef for [Option<BTreeMap<String, String>>]."],["SchemaRef","typedef for [`Arc<Schema>`]."]]});