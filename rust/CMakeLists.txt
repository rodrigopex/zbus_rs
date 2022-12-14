# SPDX-License-Identifier: Apache-2.0

target_sources(app PRIVATE
        "${PROJECT_SOURCE_DIR}/rust/src/producer_bridge.c"
        # If needed, add your C source files here
)

target_include_directories(app PRIVATE "${PROJECT_SOURCE_DIR}/src")

# Build and bring in the rust package as an "external project".
include(ExternalProject)

set(RUST_SRC_DIR "${PROJECT_SOURCE_DIR}/rust")

set(RUST_TARGET "riscv32imac-unknown-none-elf")
ExternalProject_Add(
        rust
        PREFIX ""
        SOURCE_DIR ${RUST_SRC_DIR}
        BINARY_DIR ${RUST_SRC_DIR}
        CONFIGURE_COMMAND ""
        # For the build to always be run, so that changes in the Rust
        # code are detected.
        BUILD_ALWAYS TRUE
        BUILD_COMMAND
        "cargo"
        "+nightly"
        "build"
        "--target" ${RUST_TARGET}
        "--release"
        INSTALL_COMMAND ""
        BUILD_BYPRODUCTS
        "${RUST_SRC_DIR}/target/${RUST_TARGET}/release/librust.a"
)

# By calling west build -t clean_rust, the target folder will be deleted
add_custom_target(clean_rust
        WORKING_DIRECTORY ${RUST_SRC_DIR}
        COMMAND
        "cargo"
        "+nightly"
        "clean"
        VERBATIM
)

add_library(rust_lib STATIC IMPORTED GLOBAL)

add_dependencies(
        rust_lib
        rust
)

set_target_properties(rust_lib PROPERTIES IMPORTED_LOCATION
        "${RUST_SRC_DIR}/target/${RUST_TARGET}/release/librust.a")

# Add our application, along with -lgcc
target_link_libraries(app PUBLIC rust_lib gcc)

add_subdirectory("${PROJECT_SOURCE_DIR}/rust/src/zephyr")
