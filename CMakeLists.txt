cmake_minimum_required(VERSION 3.22)

set(TARGET_NAME iceberg)

set(EXTENSION_NAME ${TARGET_NAME}_extension)
project(
  ${TARGET_NAME}
  VERSION 0.1.0
  LANGUAGES CXX)
  
#
# CORROSION
#        
        
include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG dc1e4e5ae9dc97a58d0c9e2fadc621f03c36861a # Optionally specify a commit hash, version tag or branch here
)
# Set any global configuration variables such as `Rust_TOOLCHAIN` before this line!
FetchContent_MakeAvailable(Corrosion)
  
corrosion_import_crate(MANIFEST_PATH ${CMAKE_SOURCE_DIR}/Cargo.toml)

#
# Build extension
#

set(CMAKE_CXX_STANDARD 20)

set(EXTENSION_SOURCES src/iceberg.hpp src/iceberg.cpp)

build_loadable_extension(${TARGET_NAME} ${EXTENSION_SOURCES})

corrosion_experimental_cbindgen(
        TARGET ${TARGET_NAME}
        HEADER_NAME ${EXTENSION_NAME}.hpp
        FLAGS --config ${CMAKE_SOURCE_DIR}/cbindgen.toml
)

#
# Link rust library
#

set(LIB_NAME ${TARGET_NAME}_loadable_extension)

set_target_properties(${LIB_NAME} PROPERTIES LINKER_LANGUAGE CXX)
target_link_libraries(${LIB_NAME} PUBLIC duckdb-iceberg-extension)

install(
  TARGETS ${EXTENSION_NAME}
  EXPORT "${DUCKDB_EXPORT_SET}"
  LIBRARY DESTINATION "${INSTALL_LIB_DIR}"
  ARCHIVE DESTINATION "${INSTALL_LIB_DIR}")