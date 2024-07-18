#include "environment_manager.h"
#include "dotenv.h"
#include <cstdlib>
#include <iostream>

const char *API_KEY_PATH = "cpp.env";

std::string EnvironmentManager::getApiKey() {
  dotenv::init(API_KEY_PATH);
  const std::string apiKey = std::getenv("API_KEY");

  if (!apiKey.empty()) {
    std::cout << "Retrieving [API_KEY]." << std::endl;
    return std::string(apiKey);
  } else {
    return "[API_KEY] must be set.";
  }
}