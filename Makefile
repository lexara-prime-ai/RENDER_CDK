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


# CPP Compiler and flags.
CXX = g++
CXX_FLAGS = -std=c++17 -fPIC -I./librender_cdk/extern/dotenv-cpp/include -I./librender_cdk/extern/nlohmann-json/include
LD_FLAGS = -lcurl -ljsoncpp
CPP_SRC = ./librender_cdk/src/authorization.cpp ./librender_cdk/src/environment_manager.cpp ./librender_cdk/src/service_manager.cpp
CPP_OUTPUT_DIR = ./librender_cdk/build
UNIT_TEST_SRC = ./tests/unit_tests.cpp

# Install missing dependencies.
check-dependencies:
	@dpkg -l | grep -E 'nlohmann-json3-dev|libjsoncpp-dev' || (echo "Installing missing dependencies..." && sudo apt update && sudo apt -y install nlohmann-json3-dev libjsoncpp-dev)

# CPP targets.
shared: $(CPP_OUTPUT_DIR) check-dependencies
	$(CXX) $(CXX_FLAGS) -shared $(CPP_SRC) -o $(CPP_OUTPUT_DIR)/librender_cdk.so $(LD_FLAGS)

$(CPP_OUTPUT_DIR):
	mkdir -p $(CPP_OUTPUT_DIR)

unit_test: shared $(UNIT_TEST_SRC)
	$(CXX) $(CXX_FLAGS) $(UNIT_TEST_SRC) -L$(CPP_OUTPUT_DIR) -lrender_cdk -ljsoncpp -o $(CPP_OUTPUT_DIR)/unit_tests
# Set the LD_LIBRARY_PATH to include the directory where the shared library is located
	export LD_LIBRARY_PATH=$(CPP_OUTPUT_DIR):$$LD_LIBRARY_PATH && ./$(CPP_OUTPUT_DIR)/unit_tests

# Clean all build artifacts (Rust and CPP).
clean:
	rm -rf $(CPP_OUTPUT_DIR) $(CPP_OUTPUT_DIR)/unit_tests
	cargo clean --manifest-path=$(RUST_MANIFEST_PATH)

# Targets to build both Rust and CPP together.
build-debug: shared debug
build-release: shared release

# Quick build for CPP (used for testing).
quick-build:
	cd ./librender_cdk && g++ -I./librender_cdk/extern/dotenv-cpp/include -I./librender_cdk/extern/nlohmann-json/include src/debug-build.cpp src/environment_manager.cpp src/authorization.cpp src/service_manager.cpp -o librender_cdk_DEBUG -lcurl -ljsoncpp

# Dependencies:
# sudo apt -y install nlohmann-json3-dev
# sudo apt -y install libjsoncpp-dev
