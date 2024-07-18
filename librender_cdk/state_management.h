#ifndef _STATE_MANAGEMENT_H_
#define _STATE_MANAGEMENT_H_

#include "common/common.h"
#include "common/constants.h"
#include "environment_management.h"
#include <cstddef>
#include <memory>
#include <vector>

class State {
public:
  CURL *client;
  std::string apiKey;

  State();
  ~State();
  static std::shared_ptr<State> init();
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

struct OwnerResponse {
  Owner owner;
  std::string cursor;
};

#endif
