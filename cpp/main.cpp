/*****************************************************
 *
    <REFERENCE>
    curl --request GET \
      --url 'https://api.render.com/v1/services?limit=20' \
      --header 'Accept: application/json' \
      --header 'Authorization: Bearer {{render_api_token_goes_here}}'

    <DEPENDENCIES>
      JSON - libjsoncpp-dev
      CURL - libcurl4-openssl-dev

    <BUILD>
    g++ -o list_services list_services.cpp -lcurl -ljsoncpp -I/usr/include/jsoncpp

*
*****************************************************************/


#include <cstdlib>
#include <iostream>
#include <string>
#include <curl/curl.h>
#include <jsoncpp/json/json.h>
#include "./include/dotenv.h"

using namespace std;

class EnvironmentManager {
public:
  static std::string getApiKey() {
    // Load <ENVIRONMENT> variables
    dotenv::init();
    const std::string apiKey = getenv("API_KEY");
    return apiKey;
  }
};

class ServiceManager {
public:
  static std::string listAllServices(const std::string& limit) {
    const std::string BASE_URL = "https://api.render.com/api/v1";
    const std::string API_URL = BASE_URL + "/services?limit=" + limit;
    const std::string API_KEY = EnvironmentManager::getApiKey();

    // CURL* cu

  }
};



int main() {

  return 0;
}