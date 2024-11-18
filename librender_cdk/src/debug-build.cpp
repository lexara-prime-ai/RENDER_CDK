#include "authorization.h"
#include "environment_manager.h"
#include "service_manager.h"
#include <curl/curl.h>
#include <iostream>
#include <jsoncpp/json/json.h>
#include <map>
#include <nlohmann/json.hpp>

int test_delete_service() {
  // Load <environment> configuration.
  Config config = load_config();
  if (config.api_key.empty()) {
    std::cerr << "\nAPI key is missing in the configuration." << std::endl;
    return -1;
  }

  ServiceManager service_manager = ServiceManager(config.api_key);
  std::string service_id = "srv-xxXXxxXXxx";
  service_manager.delete_service(service_id);

  return 0;
}

int test_create_service() {
  // Load <environment> configuration.
  Config config = load_config();
  if (config.api_key.empty()) {
    std::cerr << "\nAPI key is missing in the configuration." << std::endl;
    return -1;
  }

  // Initialize the AuthorizationManager with the loaded <environment>
  // variables.
  AuthorizationManager auth_manager(config.api_key);

  // Set the owner email (as a request parameter).
  std::string email = config.owner_credentials;

  // Get a list of authorized users.
  auto authorized_users = auth_manager.list_authorized_users(email);

  // Find the owner with the matching email, <owner credentials>.
  std::string owner_id;

  for (const auto &owner_response : authorized_users) {
    const auto &owner = owner_response.owner;
    if (owner.email == config.owner_credentials) {
      owner_id = owner.id;
      break;
    }
  }

  if (owner_id.empty()) {
    std::cerr << "\nNo authorized user found..." << std::endl;
    return -1;
  }

  // Sample <service data>.
  Json::Value service_data;
  service_data["type"] = "static_site";
  service_data["autoDeploy"] = "yes";
  service_data["serviceDetails"]["pullRequestPreviewsEnabled"] = "no";
  service_data["serviceDetails"]["previews"]["generation"] = "off";
  service_data["name"] = "cpp_site";
  service_data["ownerId"] = owner_id;
  service_data["repo"] =
      "https://github.com/lexara-prime-ai/SAMPLE_STATIC_SITE";
  service_data["branch"] = "main";
  service_data["rootDir"] = "./";

  // Sample <environment> variables.
  Json::Value envVar;
  envVar["key"] = "SAMPLE_KEY";
  envVar["value"] = "SAMPLE_VALUE";
  service_data["envVars"].append(envVar);

  ServiceManager service_manager(config.api_key);
  service_manager.create_service(service_data);

  return 0;
}

int test_list_services() {
  // Load <environment> configuration.
  Config config = load_config();
  if (config.api_key.empty()) {
    std::cerr << "\nAPI key is missing in the configuration." << std::endl;
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
  // Load <environment> configuration.
  Config config = load_config();
  if (config.api_key.empty()) {
    std::cerr << "\nAPI key is missing in the configuration." << std::endl;
    return -1;
  }

  // Initialize the AuthorizationManager with the loaded <environment>
  // variables.
  AuthorizationManager auth_manager(config.api_key);

  // Set the owner email (as a request parameter).
  std::string email = config.owner_credentials;

  // Get a list of authorized users.
  auto authorized_users = auth_manager.list_authorized_users(email);

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
  // test_create_service();
  // test_list_authorized_users();
  // test_list_services();
  test_delete_service();
}
