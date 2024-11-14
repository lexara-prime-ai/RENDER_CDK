#ifndef ENVIRONMENT_MANAGER_H
#define ENVIRONMENT_MANAGER_H

#include <string>

struct Config {
  std::string api_key;
  std::string owner_credentials;
};

Config load_config();

#endif