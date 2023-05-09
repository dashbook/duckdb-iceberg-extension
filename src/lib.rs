use std::ffi::c_void;

use duckdb::ffi::duckdb_database;

/// Init hook for DuckDB, registers all functionality provided by this extension
/// # Safety
/// .
#[no_mangle]
pub unsafe extern "C" fn deltatable_init_rust(db: *mut c_void) {
    let _db = db as duckdb_database;
}
