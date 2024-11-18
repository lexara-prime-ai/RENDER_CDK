#include "../../librender_cdk/extern/dotenv-cpp/include/laserpants/dotenv/dotenv.h"
#include <cstdlib>
#include <iostream>
#include <string>

/**
 * @brief Structure that holds configuration data.
 *
 * The `Config` struct contains two fields: `api_key` and `owner_credentials`,
 * which are loaded from environment variables. This structure is used to
 * store configuration values required for connecting to the Render Cloud API.
 */
struct Config {
  std::string api_key;
  std::string owner_credentials;
};

/**
 * @brief Loads configuration data from environment variables.
 *
 * This function initializes the dotenv library to load variables from a `.env`
 * file into the environment. It then retrieves the `API_KEY` and
 * `OWNER_CREDENTIALS` from the environment and stores them in a `Config`
 * struct, which is returned.
 *
 * @return A `Config` struct containing the loaded API key and owner
 * credentials. If the environment variables are not set, the corresponding
 * fields in the `Config` struct will be empty strings.
 *
 * @note The dotenv library must be properly configured, and the `.env` file
 * must exist in the current working directory with the necessary environment
 * variables. If the environment variables are not found, the fields in the
 * returned `Config` object will remain empty.
 *
 * @example
 * ```
 * Config config = load_config();
 * std::cout << "API Key: " << config.api_key << std::endl;
 * std::cout << "Owner Credentials: " << config.owner_credentials << std::endl;
 * ```
 */
Config load_config() {
  dotenv::init(".env");

  const char *api_key = std::getenv("API_KEY");
  const char *owner_credentials = std::getenv("OWNER_CREDENTIALS");

  Config config;
  if (api_key) {
    config.api_key = api_key;
  } else {
    std::cout << "[API_KEY] must be set." << std::endl;
  }

  if (owner_credentials) {
    config.owner_credentials = owner_credentials;
  } else {
    std::cout << "[OWNER_CREDENTIALS] must be set." << std::endl;
  }

  return config;
}