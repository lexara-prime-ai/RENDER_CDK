# Rust target config.
RUST_MANIFEST_PATH = ./render_cdk/Cargo.toml

# Rust targets.
format:
	cargo fmt --quiet --manifest-path=$(RUST_MANIFEST_PATH)

lint:
	cargo clippy --manifest-path=$(RUST_MANIFEST_PATH)

release:
	cargo build --manifest-path=$(RUST_MANIFEST_PATH) --release

debug:
	cargo build --manifest-path=$(RUST_MANIFEST_PATH)

run:
	cargo run --manifest-path=$(RUST_MANIFEST_PATH)

test:
	cargo test --manifest-path=$(RUST_MANIFEST_PATH) -- --nocapture


# C++ target config.
CPP_SRC_DIR = ./librender_cdk/src
CPP_BUILD_DIR = build
CPP_LIBRARY_DIR = librender_cdk
CPP_INCLUDE_DIRS = -I./librender_cdk/extern/dotenv-cpp/include -I./librender_cdk/include
CPP_FLAGS = -std=c++17 -Wall

# OpenSSL for HTTPS, json for de/serialization.
CPP_LIBS = -lssl -lcrypto -ljsoncpp


# Identifier for the static library.
CPP_LIB_NAME = librender_cdk.a

# Identifier for the shared library.
CPP_SHARED_LIB_NAME = librender_cdk.so

# Ensure the C++ build directory exists.
$(CPP_BUILD_DIR):
	mkdir -p $(CPP_BUILD_DIR)


# Static library build target.
cpp-static-lib: $(CPP_BUILD_DIR)
	ar rcs $(CPP_BUILD_DIR)/$(CPP_LIB_NAME) $(CPP_SRC_DIR)/*.cpp


# Shared library build target.
cpp-shared-lib: $(CPP_BUILD_DIR)
	g++ -shared -fPIC $(CPP_FLAGS) $(CPP_INCLUDE_DIRS) $(CPP_SRC_DIR)/*.cpp -o $(CPP_BUILD_DIR)/$(CPP_SHARED_LIB_NAME) $(CPP_LIBS)


# Clean C++ build files and libraries.
cpp-clean:
	rm -rf $(CPP_BUILD_DIR)


# Combined targets.
build: debug cpp-static-lib
release-build: release cpp-static-lib


# Optionally build shared library.
build-shared: debug cpp-shared-lib
release-build-shared: release cpp-shared-lib


# Run both Rust and C++ executables.
run-all: build
	cargo run


# Clean all build files, both Rust and C++.
clean: cpp-clean
	cargo clean



# deps:
# apt-get install libjsoncpp-dev
#