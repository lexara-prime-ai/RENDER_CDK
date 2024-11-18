#include "service_manager.h"
#include <iostream>
#include <jsoncpp/json/json.h>
#include <map>
#include <nlohmann/json.hpp>
#include <sstream>

/**
 * Define constructor with `<default limit value>`.
 */
ServiceManager::ServiceManager(const std::string &api_key,
                               const std::string &limit)
    : api_key_(api_key), limit_(limit.empty() ? "100" : limit) {}

size_t ServiceManager::WriteCallback(void *contents, size_t size, size_t nmemb,
                                     std::string *response) {
  response->append((char *)contents, size * nmemb);
  return size * nmemb;
}

/**
 * @brief Constructs a query string from a set of key-value pairs.
 *
 * This method takes a map of options and generates a URL-style query string.
 * Each key-value pair is formatted as `key=value` and separated by an ampersand
 * (`&`).
 *
 * @param options A map containing key-value pairs to be included in the query
 * string.
 *                - **Key**: The query parameter name (e.g., "name").
 *                - **Value**: The query parameter value (e.g., "John").
 *
 * @return A std::string containing the constructed query string.
 *         The string begins with an ampersand (`&`) before the first key-value
 * pair.
 *
 * @note The resulting query string starts with an `&`.
 *       If this behavior is not desired, adjust the implementation to handle
 *       the first pair differently.
 * @note This method does not URL-encode the keys or values. Consider applying
 *       URL encoding if the data includes special characters.
 *
 */
std::string
build_query_string(const std::map<std::string, std::string> &options) {
  std::ostringstream query;
  for (const auto &option : options) {
    query << "&" << option.first << "=" << option.second;
  }
  return query.str();
}

/**
 * @brief Retrieves a list of services from the Render Cloud API.
 *
 * This method initializes a CURL session and sends a request to fetch a list of
 * services based on the provided options. It returns a vector of `Service`
 * objects containing the details of the services retrieved.
 *
 * @param options A map of query parameters to filter or modify the list of
 * services. For example, it can include filters like service type, status, or
 * date. The map is expected to contain key-value pairs representing the query
 * parameters.
 *
 * @return A `std::vector<Service>` containing the services retrieved from the
 * Render Cloud API. If the request fails or no services are found, the returned
 * vector will be empty.
 *
 * @note This method requires the CURL library to be initialized properly.
 *       If CURL initialization fails, the method will return an empty vector.
 */
std::vector<Service> ServiceManager::list_services(
    const std::map<std::string, std::string> &options) {
  CURL *curl = curl_easy_init();
  std::vector<Service> services;

  if (curl) {
    std::string response;

    std::string api_url =
        "https://api.render.com/v1/services?includePreviews=true";

    // Add the <limit> if it's not overridden by options.
    if (options.find("limit") == options.end()) {
      api_url += "&limit=" + limit_;
    }

    // Append additional <query parameters> from options.
    api_url += build_query_string(options);

    std::cout << "\n[URL] <request> -> " + api_url + "\n" << std::endl;

    struct curl_slist *headers = nullptr;
    headers = curl_slist_append(headers, "accept: application/json");
    headers = curl_slist_append(headers,
                                ("Authorization: Bearer " + api_key_).c_str());

    // Configure CURL <options>.
    curl_easy_setopt(curl, CURLOPT_URL, api_url.c_str());
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION,
                     ServiceManager::WriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response);

    // Perform the <request>.
    CURLcode res = curl_easy_perform(curl);

    // <request> validation.
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

          // Format <debug> logs
          try {
            nlohmann::json prettyJson = nlohmann::json::parse(response);
            std::cout << "\n<response>::<Services [LIST]> -> \n"
                      << prettyJson.dump(4) << std::endl;
          } catch (const nlohmann::json::parse_error &e) {
            std::cerr << "\nFailed to parse JSON with nlohmann::json: "
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
          std::cerr << "\nError parsing JSON response: " << errs << std::endl;
        }
      } else {
        std::cerr << "\nRequest failed with HTTP status code: " << http_code
                  << std::endl;
      }
    } else {
      std::cerr << "\ncurl_easy_perform() failed: " << curl_easy_strerror(res)
                << std::endl;
    }

    // Clean up.
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
  }

  return services;
}

/**
 * @brief Creates a new service by sending a request to the Render Cloud API.
 *
 * This method initializes a CURL session to send a request for creating a new
 * service using the provided service data. If the CURL session cannot be
 * initialized, the method returns `false` and outputs an error message.
 *
 * @param service_data A JSON object containing the data necessary to create the
 * service. The structure of the JSON object depends on the Render Cloud API
 *                     requirements for service creation (e.g., service name,
 * configuration).
 *
 * @return `true` if the service was successfully created; otherwise, `false`.
 *
 * @note This method requires the CURL library to be initialized properly.
 *       If CURL initialization fails, an error message will be printed and the
 * method will return `false`.
 *
 * @example
 *
 * `Json::Value service_data;`
 * `service_data["name"] = "MyNewService";`
 * `service_data["type"] = "web";`
 * `bool success = service_manager.create_service(service_data);`
 * `if (success) {`
 *     `std::cout << "Service created successfully." << std::endl;`
 * `} else {`
 *     `std::cout << "Failed to create service." << std::endl;`
 * `}`
 *
 */
bool ServiceManager::create_service(const Json::Value &service_data) {
  CURL *curl = curl_easy_init();
  if (!curl) {
    std::cerr << "\nFailed to initialize CURL" << std::endl;
    return false;
  }

  std::string response;
  std::string api_url = "https://api.render.com/v1/services";

  // Serialize JSON data to string.
  Json::StreamWriterBuilder writer;
  std::string request_body = Json::writeString(writer, service_data);

  // Debug: Log the request JSON body (formatted output using nlohmann/json for
  // readability).
  try {
    nlohmann::json formatted_request = nlohmann::json::parse(request_body);
    std::cout << "\n<request>::<Create Service> -> \n"
              << formatted_request.dump(4) << std::endl;
  } catch (const nlohmann::json::parse_error &e) {
    std::cerr << "\nFailed to parse request JSON for debug: " << e.what()
              << std::endl;
  }

  struct curl_slist *headers = nullptr;
  headers = curl_slist_append(headers, "Content-Type: application/json");
  headers = curl_slist_append(headers, "accept: application/json");
  headers =
      curl_slist_append(headers, ("Authorization: Bearer " + api_key_).c_str());

  // Configure CURL <options>.
  curl_easy_setopt(curl, CURLOPT_URL, api_url.c_str());
  curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
  curl_easy_setopt(curl, CURLOPT_POST, 1L);
  curl_easy_setopt(curl, CURLOPT_POSTFIELDS, request_body.c_str());
  curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
  curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response);

  // Perform <request>.
  CURLcode res = curl_easy_perform(curl);
  bool success = false;

  // Validate <request>.
  if (res == CURLE_OK) {
    long http_code = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

    // Format <debug> logs.
    try {
      nlohmann::json formatted_response = nlohmann::json::parse(response);
      std::cout << "\n<response>::<Create Service> -> \n"
                << formatted_response.dump(4) << std::endl;
    } catch (const nlohmann::json::parse_error &e) {
      std::cerr << "\nFailed to parse response JSON for debug: " << e.what()
                << std::endl;
    }

    if (http_code == 201) {
      success = true;
    } else {
      std::cerr << "\nService creation failed with HTTP status code: "
                << http_code << std::endl;
      std::cerr << "\nResponse: " << response << std::endl;
    }
  } else {
    std::cerr << "\ncurl_easy_perform() failed: " << curl_easy_strerror(res)
              << std::endl;
  }

  // Clean up.
  curl_slist_free_all(headers);
  curl_easy_cleanup(curl);

  return success;
}

/**
 * @brief Deletes a service identified by its ID from the Render Cloud API.
 *
 * This method initializes a CURL session and sends a request to delete a
 * service specified by the given `service_id` from the Render Cloud API. The
 * service ID is used to identify which service should be deleted.
 *
 * @param service_id A string containing the ID of the service to be deleted.
 *                   This ID is required to specify which service should be
 * removed.
 *
 * @return `true` if the service was successfully deleted; otherwise, `false`.
 *         The method returns `false` if the CURL initialization fails or if the
 * deletion request is unsuccessful (e.g., the service could not be found or a
 * network error occurred).
 *
 * @note This method requires the CURL library to be properly initialized.
 *       If CURL initialization fails, an error message will be printed and the
 * method will return `false`.
 *
 * @example
 * `std::string service_id = "12345";`
 * `bool success = service_manager.delete_service(service_id);`
 * `if (success) {`
 *     `std::cout << "Service deleted successfully." << std::endl;`
 * `} else {`
 *     `std::cout << "Failed to delete service." << std::endl;`
 * `}`
 */
bool ServiceManager::delete_service(const std::string &service_id) {
  CURL *curl = curl_easy_init();
  if (!curl) {
    std::cerr << "\nFailed to initialize CURL" << std::endl;
    return false;
  }

  std::string api_url = "https://api.render.com/v1/services/" + service_id;

  // Set up headers
  struct curl_slist *headers = nullptr;
  headers = curl_slist_append(headers, "accept: application/json");
  headers =
      curl_slist_append(headers, ("Authorization: Bearer " + api_key_).c_str());

  // Configure CURL
  curl_easy_setopt(curl, CURLOPT_URL, api_url.c_str());
  curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
  curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "DELETE");

  // Perform request
  CURLcode res = curl_easy_perform(curl);
  bool success = false;

  if (res == CURLE_OK) {
    long http_code = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);

    if (http_code == 204) { // HTTP No Content.
      std::cout << "\nService deleted successfully." << std::endl;
      success = true;
    } else {
      std::cerr << "\nService deletion failed with HTTP status code: "
                << http_code << std::endl;
    }
  } else {
    std::cerr << "\ncurl_easy_perform() failed: " << curl_easy_strerror(res)
              << std::endl;
  }

  // Cleanup
  curl_slist_free_all(headers);
  curl_easy_cleanup(curl);

  return success;
}
