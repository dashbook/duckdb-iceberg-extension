#define DUCKDB_EXTENSION_MAIN

#include "iceberg.hpp"

const char* iceberg_version_rust(void);
void iceberg_init_rust(duckdb::DatabaseInstance* db);

namespace duckdb {

void IcebergExtension::Load(DuckDB &db) {
	iceberg_init_rust(db.instance.get());
}
std::string IcebergExtension::Name() {
	return "iceberg";
}

} // namespace duckdb

extern "C" void duckdb_web_iceberg_init(duckdb::DuckDB* db) { db->LoadExtension<duckdb::IcebergExtension>(); }

extern "C" {

DUCKDB_EXTENSION_API void iceberg_init(duckdb::DatabaseInstance* db) {
	iceberg_init_rust(db);
}

DUCKDB_EXTENSION_API const char *iceberg_version() {
	return duckdb::DuckDB::LibraryVersion();
}
}

#ifndef DUCKDB_EXTENSION_MAIN
#error DUCKDB_EXTENSION_MAIN not defined
#endif