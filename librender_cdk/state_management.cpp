#include "state_management.h"
#include "common/constants.h"
#include <iostream>
#include <memory>
#include <stdexcept>

static size_t WriteCallback(void *contents, size_t size, size_t nmemb,
                            void *userp) {
  ((std::string *)userp)->append((char *)contents, size * nmemb);
  return size * nmemb;
}

std::vector<OwnerResponse> OwnerResponse::parseJson(const Json::Value &json) {
  std::vector<OwnerResponse> owners;

  for (const auto &item : json) {
    Owner owner;
    owner.id = item["owner"]["id"].asString();
    owner.name = item["owner"]["name"].asString();
    owner.email = item["owner"]["email"].asString();
    owner.twoFactorAuthEnabled = item["owner"]["twoFactorAuthEnabled"].asBool();
    owner.type = item["owner"]["type"].asString();

    OwnerResponse response;
    response.owner = owner;
    response.cursor = item["cursor"].asString();
    owners.push_back(response);
  }
  return owners;
}

std::vector<Owner> Owner::retrieveAuthorizedUsers(const std::string &email,
                                                  const std::string &limit) {
  CURL *curl;
  CURLcode res;
  std::string readBuffer;
  std::string apiKey = "Bearer " + EnvironmentManager::getApiKey();

  curl = curl_easy_init();

  if (curl) {
    std::string apiUrl = BASE_URL + "/owners?limit=" + limit;

    struct curl_slist *headers = nullptr;
    headers = curl_slist_append(headers, "Accept: application/json");
    headers = curl_slist_append(headers, ("Authorization: " + apiKey).c_str());

    std::cout << "\nCreating [REQUEST] -> " << apiUrl << "\n" << std::endl;

    curl_easy_setopt(curl, CURLOPT_URL, apiUrl.c_str());
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &readBuffer);

    res = curl_easy_perform(curl);

    if (res != CURLE_OK) {
      fprintf(stderr, "curl_easy_perform() failed: %s\n",
              curl_easy_strerror(res));
    }

    /////////////////////////////
    // [DEBUG] logs.
    // std::cout << "[RESPONSE] ->" << readBuffer << std::endl;
    /////////////////////////////

    curl_easy_cleanup(curl);
    curl_slist_free_all(headers);

    Json::CharReaderBuilder readerBuilder;
    Json::Value jsonData;
    std::string errs;

    std::unique_ptr<Json::CharReader> reader(readerBuilder.newCharReader());

    if (reader->parse(readBuffer.c_str(),
                      readBuffer.c_str() + readBuffer.size(), &jsonData,
                      &errs)) {
      //////////////////////////
      // [DEBUG] logs.
      // std::cout << "\nJSON [PARSING] successful." << std::endl;
      ///////////////////////////
      std::vector<OwnerResponse> ownerResponses =
          OwnerResponse::parseJson(jsonData);
      std::vector<Owner> owners;

      for (const auto &response : ownerResponses) {
        if (response.owner.email == email) {
          owners.push_back(response.owner);
        }
      }
      return owners;
    } else {
      throw std::runtime_error("\nError parsing response: " + errs);
    }
  }

  return {};
}
