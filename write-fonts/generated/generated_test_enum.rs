// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

pub use read_fonts::codegen_test::enums::{MyEnum1, MyEnum2};

impl FontWrite for MyEnum1 {
    fn write_into(&self, writer: &mut TableWriter) {
        let val = *self as u16;
        writer.write_slice(&val.to_be_bytes())
    }
}

impl FontWrite for MyEnum2 {
    fn write_into(&self, writer: &mut TableWriter) {
        let val = *self as u16;
        writer.write_slice(&val.to_be_bytes())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MyRecord {
    pub my_enum1: MyEnum1,
    pub my_enum2: MyEnum2,
}

impl MyRecord {
    /// Construct a new `MyRecord`
    pub fn new(my_enum1: MyEnum1, my_enum2: MyEnum2) -> Self {
        Self { my_enum1, my_enum2 }
    }
}

impl FontWrite for MyRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.my_enum1.write_into(writer);
        self.my_enum2.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("MyRecord")
    }
}

impl Validate for MyRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::codegen_test::enums::MyRecord> for MyRecord {
    fn from_obj_ref(obj: &read_fonts::codegen_test::enums::MyRecord, _: FontData) -> Self {
        MyRecord {
            my_enum1: obj.my_enum1(),
            my_enum2: obj.my_enum2(),
        }
    }
}
