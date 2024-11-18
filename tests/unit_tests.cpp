#include "../librender_cdk/src/authorization.h"
#include "../librender_cdk/src/environment_manager.h"
#include <iostream>

int main() {
  try {
    Config config = load_config();
    AuthorizationManager auth_manager(config.api_key);
    auth_manager.list_authorized_users(config.owner_credentials, "10");
  } catch (const std::exception &e) {
    std::cerr << "Error: " << e.what() << "\n";
    return 1;
  }
  return 0;
}
