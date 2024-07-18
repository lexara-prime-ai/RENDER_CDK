cmake_minimum_required(VERSION 3.10)
project(librender_cdk)

# Define source files for the C++ library
file(GLOB_RECURSE LIB_SOURCES "*.cpp")

# Add the C++ library target
add_library(librender_cdk STATIC ${LIB_SOURCES})

# Include directories for the C++ library
target_include_directories(librender_cdk PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/common
)