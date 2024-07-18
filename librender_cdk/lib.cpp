#include "environment_management.h"
#include "state_management.h"
#include <iostream>

/**
 * @file lib.cpp
 * @brief This file contains examples of how to use librender_cdk.
 *
 * @brief For quick compilation, run:
 * g++ lib.cpp <dep1> <dep2>... -lcurl -ljsoncpp -I/usr/include/jsoncpp -o
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

void retrieveAuthorizedUser() { retrieveAuthorizedUser(); }

int main() {
  // retrieveApiKey();
  retrieveAuthorizedUser();
}