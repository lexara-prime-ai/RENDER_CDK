#include "environment_manager.h"
#include "authorization.h"
#include "service_manager.h"
#include <iostream>

int test_list_services() {
  // Load environment configuration.
  Config config = load_config();
  if (config.api_key.empty()) {
    std::cerr << "API key is missing in the configuration." << std::endl;
    return -1;
  }

  // Initialize the ServiceManager with the loaded API key.
  ServiceManager service_manager(config.api_key);

  // Get a list of services.
  auto services = service_manager.list_services();

  // Process the response and print output for debugging.
  if (!services.empty()) {
    std::cout << "Fetched Services: " << std::endl;
    for (const auto &service : services) {
      std::cout << "ID: " << service.id << ", Name: " << service.name
                << ", Branch: " << service.branch
                << ", Dashboard URL: " << service.dashboardUrl
                << ", Type: " << service.type
                << ", Repository: " << service.repo
                << ", Created At: " << service.createdAt
                << ", Updated At: " << service.updatedAt << std::endl;
    }
  } else {
    std::cout << "No services found." << std::endl;
  }

  return 0;
}

int test_list_authorized_users() {
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

int main() {
  test_list_authorized_users();
  test_list_services();
}

// Testing
// g++ -I./librender_cdk/extern/dotenv-cpp/include src/main.cpp
// src/environment_manager.cpp src/authorization.cpp src/service_manager.cpp -o
// main_executable -lcurl -ljsoncpp


// g++ -I./librender_cdk/extern/dotenv-cpp/include src/main.cpp src/environment_manager.cpp src/authorization.cpp src/service_manager.cpp -o main_executable -lcurl -ljsoncpp