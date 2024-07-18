#ifndef _ENVIRONMENT_MANAGER_H
#define _ENVIRONMENT_MANAGER_H

#include "dotenv.h"
#include <iostream>

class EnvironmentManager {
public:
  static std::string getApiKey();
};

#endif