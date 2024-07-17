cmake_minimum_required(VERSION 3.10)

project(ListServices)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

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

add_executable(list_services main.cpp)

include_directories(${JSONCPP_INCLUDE_DIRS})

include_directories(${CURL_INCLUDE_DIRS})

# Link the libraries.
target_link_libraries(list_services ${CURL_LIBRARIES} ${JSONCPP_LIBRARIES})

##############################################
# Build docs:
#
# cmake_minimum_required: Specifies the minimum version of CMake required.
# project: <project_name>.
# set(CMAKE_CXX_STANDARD 11): Set C++ standard to C++11.
# add_executable: Define executable target (list_services).
# find_package(jsoncpp REQUIRED): Find JsonCpp package.
# include_directories: Includes directories for JsonCpp and cURL headers.
# find_package(CURL REQUIRED): Finds cURL package.
# target_link_libraries: Links found libraries (JsonCpp and cURL) to the executable.
