#include "../../librender_cdk/extern/dotenv-cpp/include/laserpants/dotenv/dotenv.h"
#include <cstdlib>
#include <iostream>
#include <string>

struct Config {
  std::string api_key;
  std::string owner_credentials;
};

Config load_config() {
  dotenv::init(".env");

  const char *api_key = std::getenv("API_KEY");
  const char *owner_credentials = std::getenv("OWNER_CREDENTIALS");

  Config config;
  if (api_key)
    config.api_key = api_key;
  if (owner_credentials)
    config.owner_credentials = owner_credentials;

  return config;
}