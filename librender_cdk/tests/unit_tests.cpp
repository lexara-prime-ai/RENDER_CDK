#include "../src/authorization.h"
#include "../src/environment_manager.h"
#include <iostream>

int main() {
  // Load environment configuration.
  Config config = load_config();

  // Initialize the AuthorizationManager with the loaded environment variables.
  AuthorizationManager auth_manager(config.api_key);

  // Set the owner email and limit (as request parameters).
  std::string email = config.owner_credentials;
  std::string limit = "10";

  // Get a list of authorized users.
  auto authorized_users = auth_manager.list_authorized_users(email, limit);

  // Process the response and print output for debugging.
  if (!authorized_users.empty()) {
    std::cout << "Authorized Users: " << std::endl;
    for (const auto &owner_response : authorized_users) {
      const auto &owner = owner_response.owner;
      std::cout << "ID: " << owner.id << ", Name: " << owner.name
                << ", Email: " << owner.email << ", TwoFactorAuthEnabled: "
                << (owner.twoFactorAuthEnabled ? "Yes" : "No")
                << ", Type: " << owner.type << std::endl;
    }
  } else {
    std::cout << "No authorized users found." << std::endl;
  }

  return 0;
}

// Compiling and running unit tests for the project:
// The following command compiles the source files `unit_tests.cpp`,
// `environment_manager.cpp`, and `authorization.cpp`
// into an executable named `unit_tests`. The following options are used:

// - `-I../extern/dotenv-cpp/include/`: Includes the
// directory containing the `dotenv-cpp` header files.

// - `-lcurl`: Links the `libcurl` library required for making HTTP requests in
// `authorization.cpp`.
// - `-ljsoncpp`: Links the `jsoncpp` library for parsing JSON responses.
// The resulting `unit_tests` executable can be run to execute the unit tests.

// g++ -I../extern/dotenv-cpp/include/ unit_tests.cpp ../src/environment_manager.cpp ../src/authorization.cpp -o unit_tests -lcurl -ljsoncpp
