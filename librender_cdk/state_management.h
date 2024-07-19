#ifndef _STATE_MANAGEMENT_H_
#define _STATE_MANAGEMENT_H_

#include "environment_management.h"
#include <curl/curl.h>
#include <jsoncpp/json/json.h>
#include <string>
#include <vector>

struct Owner {
  std::string id;
  std::string name;
  std::string email;
  bool twoFactorAuthEnabled;
  std::string type;

  static std::vector<Owner> retrieveAuthorizedUsers(const std::string &email,
                                                    const std::string &limit);
};

struct OwnerResponse {
  Owner owner;
  std::string cursor;

  static std::vector<OwnerResponse> parseJson(const Json::Value &json);
};

static size_t WriteCallback(void *contents, size_t size, size_t nmemb,
                            void *userp);

#endif
