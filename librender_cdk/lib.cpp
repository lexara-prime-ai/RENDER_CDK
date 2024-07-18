#include "environment_manager.h"
#include <iostream>

/**
 * @file lib.cpp
 * @brief This file contains examples of how to use librender_cdk.
 */

/**
 * @brief Retrieves and prints the API key to the standard output.
 *
 * This example demonstrates how to retrieve the API key using the EnvironmentManager
 * and print it to the console.
 */

void retrieveApiKey() {
  std::cout << "[API_KEY] -> " << EnvironmentManager::getApiKey() << std::endl;
}