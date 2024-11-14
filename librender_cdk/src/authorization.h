#ifndef AUTHORIZATION_H
#define AUTHORIZATION_H

#include "environment_manager.h"
#include <curl/curl.h>
#include <optional>
#include <string>
#include <vector>

struct Owner {
  std::string id;
  std::string name;
  std::string email;
  std::optional<bool> twoFactorAuthEnabled;
  std::string type;
};

struct OwnerResponse {
  Owner owner;
  std::string cursor;
};

class AuthorizationManager {
public:
  AuthorizationManager(const std::string &api_key) : api_key_(api_key) {}

  std::vector<OwnerResponse> list_authorized_users(const std::string &email,
                                                   const std::string &limit);

private:
  std::string api_key_;
  const std::string base_url = "https://api.render.com/v1";
};

#endif
