#include "../librender_cdk/src/authorization.h"
#include "../librender_cdk/src/environment_manager.h"
#include "../librender_cdk/src/service_manager.h"
#include <iostream>
#include <jsoncpp/json/json.h>

int test_retrieve_authorized_user() {
  try {
    Config config = load_config();
    AuthorizationManager auth_manager(config.api_key);
    auth_manager.list_authorized_users(config.owner_credentials, "10");
  } catch (const std::exception &e) {
    std::cerr << "Error: " << e.what() << "\n";
    return 1;
  }
  return 0;
}

int test_create_service() {
  Config config = load_config();
  if (config.api_key.empty()) {
    std::cerr << "\nAPI key is missing in the configuration." << std::endl;
    return -1;
  }

  AuthorizationManager auth_manager(config.api_key);
  std::string email = config.owner_credentials;
  std::string limit = "10";

  auto authorized_users = auth_manager.list_authorized_users(email, limit);

  std::string owner_id;

  for (const auto &owner_response : authorized_users) {
    const auto &owner = owner_response.owner;
    if (owner.email == config.owner_credentials) {
      owner_id = owner.id;
      break;
    }
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

  Json::Value envVar;
  envVar["key"] = "SAMPLE_KEY";
  envVar["value"] = "SAMPLE_VALUE";
  service_data["envVars"].append(envVar);

  ServiceManager service_manager(config.api_key);
  service_manager.create_service(service_data);

  return 0;
}

int main() {
  test_retrieve_authorized_user();
  test_create_service();
}
