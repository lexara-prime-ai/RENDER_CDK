#include "service_manager.h"
#include <iostream>
#include <sstream>

ServiceManager::ServiceManager(const std::string &api_key)
    : api_key_(api_key) {}

// Define WriteCallback as a static member function.
size_t ServiceManager::WriteCallback(void *contents, size_t size, size_t nmemb,
                                     std::string *response) {
  response->append((char *)contents, size * nmemb);
  return size * nmemb;
}

std::vector<Service> ServiceManager::list_services() {
  CURL *curl = curl_easy_init();
  std::vector<Service> services;

  if (curl) {
    std::string response;
    std::string api_url =
        "https://api.render.com/v1/services?includePreviews=true&limit=20";

    // Set up headers.
    struct curl_slist *headers = nullptr;
    headers = curl_slist_append(headers, "accept: application/json");
    headers = curl_slist_append(headers,
                                ("Authorization: Bearer " + api_key_).c_str());

    // Configure CURL options.
    curl_easy_setopt(curl, CURLOPT_URL, api_url.c_str());
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION,
                     ServiceManager::WriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response);

    // Perform the request.
    CURLcode res = curl_easy_perform(curl);

    // Request validation.
    if (res == CURLE_OK) {
      long http_code = 0;
      curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
      if (http_code == 200) {
        // JSON response parsing.
        Json::Value jsonData;
        Json::CharReaderBuilder reader;
        std::string errs;
        std::istringstream s(response);

        if (Json::parseFromStream(reader, s, &jsonData, &errs)) {
          for (const auto &item : jsonData) {
            const auto &serviceData = item["service"];
            Service service{serviceData["id"].asString(),
                            serviceData["name"].asString(),
                            serviceData["branch"].asString(),
                            serviceData["dashboardUrl"].asString(),
                            serviceData["type"].asString(),
                            serviceData["repo"].asString(),
                            serviceData["createdAt"].asString(),
                            serviceData["updatedAt"].asString()};
            services.push_back(service);
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

  return services;
}
