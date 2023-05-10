cmake_minimum_required(VERSION 3.22)

set(TARGET_NAME iceberg)

set(EXTENSION_NAME ${TARGET_NAME}_extension)
set(RUST_LIB duckdb-iceberg-extension)

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
    GIT_TAG v0.3 # Optionally specify a commit hash, version tag or branch here
)

if(EMSCRIPTEN)
    set(Rust_CARGO_TARGET wasm32-unknown-unknown)
endif()  

FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml)

#
# Build extension
#

set(EXTENSION_SOURCES src/iceberg.hpp src/iceberg.cpp)

build_loadable_extension(${TARGET_NAME} ${EXTENSION_SOURCES})

set(LIB_NAME ${TARGET_NAME}_loadable_extension)

set_target_properties(${LIB_NAME} PROPERTIES LINKER_LANGUAGE CXX)
target_link_libraries(${LIB_NAME} ${RUST_LIB})
