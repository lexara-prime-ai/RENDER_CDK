cmake_minimum_required(VERSION 3.10)

project(RenderCdk)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_subdirectory(librender_cdk)

include_directories(${CMAKE_SOURCE_DIR}/librender_cdk/librender_cdk)

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

add_executable(render_cdk librender_cdk/librender_cdk/main.cpp)

# Include JsonCpp and cURL header directories.
include_directories(${JSONCPP_INCLUDE_DIRS})
include_directories(${CURL_INCLUDE_DIRS})

# Link the libraries.
target_link_libraries(render_cdk ${CURL_LIBRARIES} ${JSONCPP_LIBRARIES})

##############################################
# Build docs:
#
# cmake_minimum_required: Specifies the minimum version of CMake required.
# project: <project_name>: Defines the project name.
# set(CMAKE_CXX_STANDARD 11): Sets the C++ standard to C++11.
# add_subdirectory: Adds the subdirectory for the library.
# include_directories: Adds include directories for library and packages.
# find_package(jsoncpp REQUIRED): Finds the JsonCpp package.
# find_package(CURL REQUIRED): Finds the cURL package.
# add_executable: Defines the executable target.
# target_link_libraries: Links the found libraries (JsonCpp and cURL) to the executable.
#
#  Building with the GNU compiler.
#  > g++ lib.cpp environment_manager.cpp -o <executable_name>
#
###############################################