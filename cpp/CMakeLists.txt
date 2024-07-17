# Minimum CMake version required
cmake_minimum_required(VERSION 3.10)

# Project name
project(ListServices)

# Set C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Specify the executable target
add_executable(list_services main.cpp)

# Find and include the JsonCpp library
find_package(jsoncpp REQUIRED)
include_directories(${JSONCPP_INCLUDE_DIRS})

# Find and include the cURL library
find_package(CURL REQUIRED)
include_directories(${CURL_INCLUDE_DIRS})

# Link the libraries to the executable
target_link_libraries(list_services ${CURL_LIBRARIES} ${JSONCPP_LIBRARIES})



# Explanation:
# cmake_minimum_required: Specifies the minimum version of CMake required.
# project: Defines the project name.
# set(CMAKE_CXX_STANDARD 11): Sets the C++ standard to C++11.
# add_executable: Defines the executable target (list_services).
# find_package(jsoncpp REQUIRED): Finds the JsonCpp package.
# include_directories: Includes the directories for JsonCpp and cURL headers.
# find_package(CURL REQUIRED): Finds the cURL package.
# target_link_libraries: Links the found libraries (JsonCpp and cURL) to the executable.