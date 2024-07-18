cmake_minimum_required(VERSION 3.10)
project(my_project)

# Set the C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Define source files for the C++ library
file(GLOB_RECURSE LIB_SOURCES "librender_cdk/*.cpp")

# Add the C++ library target
add_library(librender_cdk STATIC ${LIB_SOURCES})

# Include directories for the C++ library
target_include_directories(librender_cdk PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/librender_cdk
    ${CMAKE_CURRENT_SOURCE_DIR}/librender_cdk/common
)

# Add a custom target to build the Rust project
add_custom_target(render_cdk ALL
    COMMAND cargo build --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/render_cdk/Cargo.toml
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/render_cdk
    COMMENT "Runnig [cargo build] on specified manifest path.."
)

# If you have any additional dependencies for the C++ library, link them here
# target_link_libraries(librender_cdk <dependency>)

# Add dependencies to ensure C++ library is built before Rust project
add_dependencies(render_cdk librender_cdk)

# Specify that 'render_cdk' should be built when invoking 'make'
add_custom_target(build_all
    DEPENDS librender_cdk render_cdk
)
