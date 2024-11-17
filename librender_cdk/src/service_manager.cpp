#include "service_manager.h"
#include <iostream>
#include <jsoncpp/json/json.h>
#include <map>
#include <nlohmann/json.hpp>
#include <sstream>

// Define constructor with default limit value.
ServiceManager::ServiceManager(const std::string &api_key,
                               const std::string &limit)
    : api_key_(api_key), limit_(limit.empty() ? "100" : limit) {}

size_t ServiceManager::WriteCallback(void *contents, size_t size, size_t nmemb,
                                     std::string *response) {
  response->append((char *)contents, size * nmemb);
  return size * nmemb;
}

std::string
build_query_string(const std::map<std::string, std::string> &options) {
  std::ostringstream query;
  for (const auto &option : options) {
    query << "&" << option.first << "=" << option.second;
  }
  return query.str();
}

std::vector<Service> ServiceManager::list_services(
    const std::map<std::string, std::string> &options) {
  CURL *curl = curl_easy_init();
  std::vector<Service> services;

  if (curl) {
    std::string response;

    // Base API URL
    std::string api_url =
        "https://api.render.com/v1/services?includePreviews=true";

    // Add the limit if it's not overridden by options
    if (options.find("limit") == options.end()) {
      api_url += "&limit=" + limit_;
    }

    // Append additional query parameters from options
    api_url += build_query_string(options);

    // Print <request> url
    std::cout << "\n<request> -> " + api_url + "\n" << std::endl;

    // Set up headers
    struct curl_slist *headers = nullptr;
    headers = curl_slist_append(headers, "accept: application/json");
    headers = curl_slist_append(headers,
                                ("Authorization: Bearer " + api_key_).c_str());

    // Configure CURL options
    curl_easy_setopt(curl, CURLOPT_URL, api_url.c_str());
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION,
                     ServiceManager::WriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response);

    // Perform the request
    CURLcode res = curl_easy_perform(curl);

    // Request validation
    if (res == CURLE_OK) {
      long http_code = 0;
      curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

      if (http_code == 200) {
        // JSON response parsing
        Json::Value jsonData;
        Json::CharReaderBuilder reader;
        std::string errs;
        std::istringstream s(response);

        if (Json::parseFromStream(reader, s, &jsonData, &errs)) {

          // Format <debug> logs
          try {
            nlohmann::json prettyJson = nlohmann::json::parse(response);
            std::cout << "<response>::<Services> -> \n"
                      << prettyJson.dump(4) << std::endl;
          } catch (const nlohmann::json::parse_error &e) {
            std::cerr << "Failed to parse JSON with nlohmann::json: "
                      << e.what() << std::endl;
          }

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

    // Clean up
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
  }

  return services;
}
