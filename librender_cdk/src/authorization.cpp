#include "authorization.h"
#include <curl/curl.h>
#include <iostream>
#include <jsoncpp/json/json.h>
#include <nlohmann/json.hpp>
#include <sstream>

// Define an anonymous namespace for the WriteCallback function.
namespace {
// Callback function for handling the data received from `libcurl`.
size_t WriteCallback(void *contents, size_t size, size_t nmemb,
                     std::string *response) {
  response->append((char *)contents, size * nmemb);
  return size * nmemb;
}
} // namespace

std::vector<OwnerResponse>
AuthorizationManager::list_authorized_users(const std::string &email,
                                            const std::string &limit) {
  CURL *curl = curl_easy_init();
  std::vector<OwnerResponse> ownerResponses;

  if (curl) {
    std::string response;
    std::string api_url = base_url + "/owners?limit=" + limit;

    // Set up headers.
    struct curl_slist *headers = nullptr;
    headers = curl_slist_append(headers, "Content-Type: application/json");
    headers = curl_slist_append(headers,
                                ("Authorization: Bearer " + api_key_).c_str());

    // Configure CURL options.
    curl_easy_setopt(curl, CURLOPT_URL, api_url.c_str());
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response);

    // Perform <request>.
    CURLcode res = curl_easy_perform(curl);

    // Validate <request>.
    if (res == CURLE_OK) {
      long http_code = 0;
      curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
      if (http_code == 200) {

        // JSON <response> parsing.
        Json::Value jsonData;
        Json::CharReaderBuilder reader;
        std::string errs;
        std::istringstream s(response);

        if (Json::parseFromStream(reader, s, &jsonData, &errs)) {

          // Format <debug> logs.
          try {
            nlohmann::json prettyJson = nlohmann::json::parse(response);
            std::cout << "<response>::<Authorized Users> -> \n"
                      << prettyJson.dump(4) << std::endl;
          } catch (const nlohmann::json::parse_error &e) {
            std::cerr << "Failed to parse JSON with nlohmann::json: "
                      << e.what() << std::endl;
          }

          for (const auto &item : jsonData) {
            Owner owner{item["owner"]["id"].asString(),
                        item["owner"]["name"].asString(),
                        item["owner"]["email"].asString(),
                        item["owner"]["twoFactorAuthEnabled"].asBool(),
                        item["owner"]["type"].asString()};

            OwnerResponse ownerResponse{owner, item["cursor"].asString()};

            // Filter owners based on the owner's credentials.
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

    // Clean up.
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
  }

  return ownerResponses;
}