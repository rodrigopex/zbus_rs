# SPDX-License-Identifier: Apache-2.0
target_include_directories(app PRIVATE
        "${PROJECT_SOURCE_DIR}/rust/src/zephyr/")

target_sources(app PRIVATE
        "${PROJECT_SOURCE_DIR}/rust/src/zephyr/bridge.c")
