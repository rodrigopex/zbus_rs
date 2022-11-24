# SPDX-License-Identifier: Apache-2.0

target_sources(app PRIVATE
        "${PROJECT_SOURCE_DIR}/rust/src/producer.c"
        "${PROJECT_SOURCE_DIR}/rust/src/zephyr/bridge.c")

target_include_directories(app PRIVATE "${PROJECT_SOURCE_DIR}/src")

# Build and bring in the rust package as an "external project".
include(ExternalProject)

set(rust_src_dir "${PROJECT_SOURCE_DIR}/rust")
set(rust_target "riscv32imac-unknown-none-elf")
ExternalProject_Add(
        rust
        PREFIX ${standlone_build_dir}
        SOURCE_DIR ${rust_src_dir}
        BINARY_DIR ${rust_src_dir}
        CONFIGURE_COMMAND ""
        # For the build to always be run, so that changes in the Rust
        # code are detected.
        BUILD_ALWAYS TRUE
        BUILD_COMMAND
        "cargo"
        "+nightly"
        "build"
        "--target" ${rust_target}
        "--release"
        INSTALL_COMMAND ""
        BUILD_BYPRODUCTS
        "${rust_src_dir}/target/${rust_target}/release/librust.a"
)

add_library(rust_lib STATIC IMPORTED GLOBAL)

add_dependencies(
        rust_lib
        rust
)

set_target_properties(rust_lib PROPERTIES IMPORTED_LOCATION
        "${rust_src_dir}/target/${rust_target}/release/librust.a")

# Add our application, along with -lgcc
target_link_libraries(app PUBLIC rust_lib gcc)
