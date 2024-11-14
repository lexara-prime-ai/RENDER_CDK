#include "authorization.h"
#include "environment_manager.h"
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

  // Process the response and print the result
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

// Testing
// g++ -I./librender_cdk/extern/dotenv-cpp/include src/main.cpp
// src/environment_manager.cpp src/authorization.cpp -o main_executable -lcurl
// -ljsoncpp
