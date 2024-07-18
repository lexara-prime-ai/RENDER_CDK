cmake_minimum_required(VERSION 3.10)
project(render_cloud_cdk)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Clean up followed by fresh install.
execute_process(
    COMMAND sudo apt purge -y libjsoncpp-dev
    COMMAND sudo apt purge -y libcurl4-openssl-dev
)

# Find packages and handle missing ones
find_package(jsoncpp QUIET)
if(NOT jsoncpp_FOUND)
    message(STATUS "JsonCpp not found. Attempting to install...")
    execute_process(
        COMMAND sudo apt-get update
        COMMAND sudo apt-get install -y libjsoncpp-dev
        COMMAND_ERROR_IS_FATAL ANY
    )
    find_package(jsoncpp REQUIRED)
endif()

find_package(CURL QUIET)
if(NOT CURL_FOUND)
    message(STATUS "cURL not found. Attempting to install...")
    execute_process(
        COMMAND sudo apt-get update
        COMMAND sudo apt-get install -y libcurl4-openssl-dev
        COMMAND_ERROR_IS_FATAL ANY
    )
    find_package(CURL REQUIRED)
endif()

# Define source files
file(GLOB_RECURSE LIB_SOURCES "librender_cdk/*.cpp")

# Add a static library target for the C++ code
add_library(librender_cdk STATIC ${LIB_SOURCES})

# Include directories for dependencies
include_directories(${JSONCPP_INCLUDE_DIRS})
include_directories(${CURL_INCLUDE_DIRS})

# Link dependencies to the C++ library
target_link_libraries(librender_cdk ${CURL_LIBRARIES} ${JSONCPP_LIBRARIES})

# Add a custom target to build the Rust project
add_custom_target(render_cdk ALL
    COMMAND cargo build --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/render_cdk/Cargo.toml
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/render_cdk
    COMMENT "Running [cargo build] on specified manifest path.."
)

# Ensure the C++ library is built before the Rust project
add_dependencies(render_cdk librender_cdk)

# Define a custom target to build both C++ and Rust projects
add_custom_target(build_all
    DEPENDS librender_cdk render_cdk
)
