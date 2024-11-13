#include "environment_manager.h"
#include <iostream>

int main() {
  // Call the load_config function to load environment variables
  Config config = load_config();

  // Print the loaded configuration values
  std::cout << "API_KEY: " << config.api_key << std::endl;
  std::cout << "OWNER_CREDENTIALS: " << config.owner_credentials << std::endl;

  return 0;
}

// #include "../cpp-httplib/httplib.h"
// #include <iostream>

// int main() {
//   httplib::Client cli("http://example.com");
//   auto res = cli.Get("/api/v1/resource");

//   if (res) {
//     std::cout << "Status: " << res->status << std::endl;
//     std::cout << "Response Body: " << res->body << std::endl;
//   }
// }
