# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})
project(hello_world)

target_sources(app PRIVATE src/consumer.c)

add_subdirectory(rust)
#add_subdirectory("${PROJECT_SOURCE_DIR}/src/led_service")
