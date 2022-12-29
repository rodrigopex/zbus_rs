# SPDX-License-Identifier: Apache-2.0

target_sources(app PRIVATE
        "${PROJECT_SOURCE_DIR}/rust/src/producer_bridge.c"
        # If needed, add your C source files here
)

target_include_directories(app PRIVATE "${PROJECT_SOURCE_DIR}/src")

# Build and bring in the rust package as an "external project".
include(ExternalProject)

set(RUST_SRC_DIR "${PROJECT_SOURCE_DIR}/rust")

#set(RUST_TARGET "riscv32imac-unknown-none-elf")
set(RUST_TARGET "riscv32imc-unknown-none-elf")
#set(RUST_TARGET "thumbv7em-none-eabihf")

set(RUST_TARGET_DIR "${PROJECT_SOURCE_DIR}/build")

option(RUST_MODE_RELEASE "This enables release as the Rust code portion mode" ON)
if(RUST_MODE_RELEASE)
    set(RUST_LIB_DESTINATION "${RUST_TARGET_DIR}/${RUST_TARGET}/release/librust.a")
    set(RUST_RELEASE_FLAG "--release")
else()
    set(RUST_LIB_DESTINATION "${RUST_TARGET_DIR}/${RUST_TARGET}/debug/librust.a")
    set(RUST_RELEASE_FLAG "--debug")
endif()

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
        "--target-dir" ${RUST_TARGET_DIR}
        ${RUST_RELEASE_FLAG}
        INSTALL_COMMAND ""
        BUILD_BYPRODUCTS
        ${RUST_LIB_DESTINATION}
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
        ${RUST_LIB_DESTINATION})

# Add our application, along with -lgcc
target_link_libraries(app PUBLIC rust_lib gcc)

add_subdirectory("${PROJECT_SOURCE_DIR}/rust/src/zephyr")
