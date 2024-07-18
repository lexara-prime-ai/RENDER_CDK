#ifndef _STATE_MANAGEMENT_H_
#define _STATE_MANAGEMENT_H_

#include "common/common.h"
#include "common/constants.h"
#include "environment_manager.h"
#include <memory>
#include <vector>

class State {
public:
  CURL *client;
  std::string apiKey;

  State() {
    client = curl_easy_init();
    apiKey = EnvironmentManager::getApiKey();
  }

  ~State() {
    if (client)
      curl_easy_cleanup(client);
  }

  static std::shared_ptr<State> init() { return std::make_shared<State>(); }
};

struct Owner {
  std::string id;
  std::string name;
  std::string email;
  bool twoFactorAuthEnabled;
  std::string type;

  static std::vector<Owner> retrieveAuthorizedUsers(const std::string &email,
                                                    const std::string &limit);
};


#endif
