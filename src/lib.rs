use std::ffi::c_char;

use duckdb::{
    ffi::{duckdb_database, duckdb_library_version},
    Connection,
};
use table_function::IcebergFunction;

mod table_function;

/// Init hook for DuckDB, registers all functionality provided by this extension
/// # Safety
/// .
#[no_mangle]
pub unsafe extern "C" fn iceberg_init_rust(db: duckdb_database) {
    let connection = Connection::open_from_raw(db).expect("Failed to open database connection.");
    connection
        .register_table_function::<IcebergFunction>("read_iceberg")
        .expect("Failed to register table function.");
}

/// Version hook for DuckDB, indicates which version of DuckDB this extension was compiled against
#[no_mangle]
pub extern "C" fn deltatable_version_rust() -> *const c_char {
    unsafe { duckdb_library_version() }
}
