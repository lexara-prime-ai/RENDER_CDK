#include "environment_manager.h"
#include <iostream>

int main() {
  std::cout << "[API_KEY] -> " << EnvironmentManager::getApiKey() << std::endl;
}