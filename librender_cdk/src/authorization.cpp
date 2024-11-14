#include "authorization.h"
#include <curl/curl.h>
#include <iostream>
#include <jsoncpp/json/json.h> // Include this if using a JSON parsing library like JSON for Modern C++
#include <sstream>

namespace {

// Callback function for handling the data received from `libcurl`.
size_t WriteCallback(void *contents, size_t size, size_t nmemb,
                     std::string *response) {
  response->append((char *)contents, size * nmemb);
  return size * nmemb;
}

} // anonymous namespace

std::vector<OwnerResponse>
AuthorizationManager::list_authorized_users(const std::string &email,
                                            const std::string &limit) {
  CURL *curl = curl_easy_init();
  std::vector<OwnerResponse> ownerResponses;

  if (curl) {
    std::string response;
    std::string api_url = base_url + "/owners?limit=" + limit;

    // Set up headers
    struct curl_slist *headers = nullptr;
    headers = curl_slist_append(headers, "Content-Type: application/json");
    headers = curl_slist_append(headers,
                                ("Authorization: Bearer " + api_key_).c_str());

    // Configure CURL options
    curl_easy_setopt(curl, CURLOPT_URL, api_url.c_str());
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response);

    // Perform the request
    CURLcode res = curl_easy_perform(curl);

    // Check if request was successful
    if (res == CURLE_OK) {
      long http_code = 0;
      curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
      if (http_code == 200) {
        // Parse JSON response
        Json::Value jsonData;
        Json::CharReaderBuilder reader;
        std::string errs;
        std::istringstream s(response);

        if (Json::parseFromStream(reader, s, &jsonData, &errs)) {
          for (const auto &item : jsonData) {
            Owner owner{item["owner"]["id"].asString(),
                        item["owner"]["name"].asString(),
                        item["owner"]["email"].asString(),
                        item["owner"]["twoFactorAuthEnabled"].asBool(),
                        item["owner"]["type"].asString()};

            OwnerResponse ownerResponse{owner, item["cursor"].asString()};

            // Filter owners based on the provided email
            if (owner.email == email) {
              ownerResponses.push_back(ownerResponse);
            }
          }
        } else {
          std::cerr << "Error parsing JSON response: " << errs << std::endl;
        }
      } else {
        std::cerr << "Request failed with HTTP status code: " << http_code
                  << std::endl;
      }
    } else {
      std::cerr << "curl_easy_perform() failed: " << curl_easy_strerror(res)
                << std::endl;
    }

    // Clean up
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
  }

  return ownerResponses;
}
// g++ -I./librender_cdk/extern/dotenv-cpp src/main.cpp src/environment_manager.cpp src/authorization.cpp -o main_executable -lcurl -ljsoncpp -ldotenv
