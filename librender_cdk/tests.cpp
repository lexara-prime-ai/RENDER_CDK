#include "environment_management.h"
#include <iostream>

int main() {
  std::string apiKey = EnvironmentManager::getApiKey();

  std::cout << "[DEBUG] logs -> " << apiKey << std::endl;
}