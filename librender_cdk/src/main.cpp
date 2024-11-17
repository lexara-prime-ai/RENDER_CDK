#include "authorization.h"
#include "environment_manager.h"
#include "service_manager.h"
#include <iostream>
#include <map>
#include <nlohmann/json.hpp>

int test_list_services() {
  // Load environment configuration.
  Config config = load_config();
  if (config.api_key.empty()) {
    std::cerr << "API key is missing in the configuration." << std::endl;
    return -1;
  }

  std::map<std::string, std::string> OPTIONS_A = {{"type", "web_service"},
                                                  {"limit", "10"}};

  std::map<std::string, std::string> OPTIONS_B = {{"name", "twitt3r"}};

  // Initialize the ServiceManager with the loaded API key with custom options.
  ServiceManager service_manager(config.api_key);
  // ServiceManager service_manager(config.api_key, "1");

  // Get a list of services with OPTIONS.
  auto service_list_A = service_manager.list_services(OPTIONS_A);
  auto service_list_B = service_manager.list_services(OPTIONS_B);

  // // Process the response and convert to JSON.
  // if (!services.empty()) {
  //   // Create a JSON array to hold the service objects.
  //   nlohmann::json services_json = nlohmann::json::array();

  //   for (const auto &service : services) {
  //     // Create a JSON object for each service.
  //     nlohmann::json service_json = {{"id", service.id},
  //                                    {"name", service.name},
  //                                    {"branch", service.branch},
  //                                    {"dashboardUrl", service.dashboardUrl},
  //                                    {"type", service.type},
  //                                    {"repository", service.repo},
  //                                    {"createdAt", service.createdAt},
  //                                    {"updatedAt", service.updatedAt}};

  //     // Add the JSON object to the JSON array.
  //     services_json.push_back(service_json);
  //   }

  //   // Print the JSON array in a pretty format.
  //   std::cout << "\n<debug>::<Services>:" << std::endl;
  //   std::cout << services_json.dump(4) << std::endl; // 4 spaces for
  //   indentation
  // } else {
  //   std::cout << "No services found." << std::endl;
  // }

  return 0;
}

int test_list_authorized_users() {
  // Load environment configuration.
  Config config = load_config();
  if (config.api_key.empty()) {
    std::cerr << "API key is missing in the configuration." << std::endl;
    return -1;
  }

  // Initialize the AuthorizationManager with the loaded environment variables.
  AuthorizationManager auth_manager(config.api_key);

  // Set the owner email and limit (as request parameters).
  std::string email = config.owner_credentials;
  std::string limit = "10";

  // Get a list of authorized users.
  auto authorized_users = auth_manager.list_authorized_users(email, limit);

  // Process the response and print output for debugging.
  // if (!authorized_users.empty()) {
  //   std::cout << "Authorized Users: " << std::endl;
  //   for (const auto &owner_response : authorized_users) {
  //     const auto &owner = owner_response.owner;
  //     std::cout << "ID: " << owner.id << ", Name: " << owner.name
  //               << ", Email: " << owner.email << ", TwoFactorAuthEnabled: "
  //               << (owner.twoFactorAuthEnabled ? "Yes" : "No")
  //               << ", Type: " << owner.type << std::endl;
  //   }
  // } else {
  //   std::cout << "No authorized users found." << std::endl;
  // }

  return 0;
}

int main() {
  test_list_authorized_users();
  test_list_services();
}
