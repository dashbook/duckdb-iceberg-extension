#define DUCKDB_EXTENSION_MAIN

#inlcude "iceberg.hpp"
#include "iceberg_extension.hpp"

namespace duckdb {

void IcebergExtension::Load(DuckDB &db) {
	iceberg_init_rust(*db.instance);
}
std::string IcebergExtension::Name() {
	return "iceberg";
}

} // namespace duckdb

extern "C" {

DUCKDB_EXTENSION_API void iceberg_init(duckdb::DatabaseInstance &db) {
	iceberg_init_rust(db);
}

DUCKDB_EXTENSION_API const char *iceberg_version() {
	return duckdb::DuckDB::LibraryVersion();
}
}

#ifndef DUCKDB_EXTENSION_MAIN
#error DUCKDB_EXTENSION_MAIN not defined
#endif