use std::{
    error::Error,
    ffi::{c_char, CString},
};

use duckdb::vtab::{
    BindInfo, DataChunk, Free, FunctionInfo, InitInfo, LogicalType, LogicalTypeId, VTab,
};

#[repr(C)]
pub struct IcebergBindDataStruct {
    filename: *mut c_char,
}
impl Free for IcebergBindDataStruct {
    fn free(&mut self) {
        unsafe {
            drop(CString::from_raw(self.filename.cast()));
        }
    }
}

#[repr(C)]
pub struct IcebergInitDataStruct {
    done: bool, // TODO: support more than *vector size* rows
}
impl Free for IcebergInitDataStruct {}

pub struct IcebergFunction {}
impl VTab for IcebergFunction {
    type InitData = IcebergInitDataStruct;
    type BindData = IcebergBindDataStruct;
    fn bind(bind: &BindInfo, data: *mut Self::BindData) -> duckdb::Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn init(_init: &InitInfo, _data: *mut Self::InitData) -> duckdb::Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn func(func: &FunctionInfo, output: &mut DataChunk) -> duckdb::Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn parameters() -> Option<Vec<LogicalType>> {
        Some(vec![LogicalType::new(LogicalTypeId::Varchar)])
    }
}
