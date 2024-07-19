#include "environment_management.h"
#include "state_management.h"
#include <cstdlib>
#include <iostream>

/**
 * @file lib.cpp
 * @brief This file contains examples of how to use librender_cdk.
 *
 * @brief For quick compilation, run:
 * g++ librender_cdk.cpp <dep1> <dep2>... -lcurl -ljsoncpp -I/usr/include/jsoncpp -o
 * g++ -o librender_cdk_DEBUG librender_cdk.cpp environment_management.cpp state_management.cpp -lcurl -ljsoncpp
 * <executable_name>
 *
 */

/**
 * @brief Retrieves and prints the API key to the standard output.
 *
 * This example demonstrates how to retrieve the API key using the
 * EnvironmentManager and print it to the console.
 */

void retrieveApiKey() {
  std::cout << EnvironmentManager::getApiKey() << std::endl;
}

void retrieveOwnerInfo() {
  std::vector<Owner> result =
      Owner::retrieveAuthorizedUsers("irfanghat@gmail.com", "50");

  if (!result.empty()) {
    std::cout << "Found [AUTHORIZED] user:" << std::endl;
    for (const auto &owner : result) {
      std::cout << "ID: " << owner.id << std::endl;
      std::cout << "Name: " << owner.name << std::endl;
      std::cout << "Email: " << owner.email << std::endl;
      std::cout << "Two-Factor Auth Enabled: "
                << (owner.twoFactorAuthEnabled ? "Yes" : "No") << std::endl;
      std::cout << "Type: " << owner.type << std::endl;
      std::cout << "_______________________" << std::endl;
    }
  } else {
    std::cout << "No [AUTHORIZED] users found." << std::endl;
  }
}

int main() {
  // retrieveApiKey();
  retrieveOwnerInfo();
}