#include "state_management.h"
#include "environment_management.h"

size_t WriteCallback(void *contents, size_t size, size_t nmemb,
                     std::string *s) {
  size_t newLength = size * nmemb;
  s->append((char *)contents, newLength);
  return newLength;
}

State::State() {
  client = curl_easy_init();
  apiKey = EnvironmentManager::getApiKey();
}

State::~State() {
  if (client)
    curl_easy_cleanup(client);
}

std::shared_ptr<State> State::init() { return std::make_shared<State>(); }

std::vector<Owner> Owner::retrieveAuthorizedUsers(const std::string &email,
                                                  const std::string &limit) {
  auto state = State::init();
  std::string apiUrl = BASE_URL + "/owners?limit=" + limit;
  std::string readBuffer;

  curl_easy_setopt(state->client, CURLOPT_URL, apiUrl.c_str());
  struct curl_slist *headers = nullptr;
  headers = curl_slist_append(headers, "Accept: application/json");
  headers = curl_slist_append(
      headers, ("Authorization: Bearer " + state->apiKey).c_str());
  curl_easy_setopt(state->client, CURLOPT_HTTPHEADER, headers);
  curl_easy_setopt(state->client, CURLOPT_WRITEFUNCTION, WriteCallback);
  curl_easy_setopt(state->client, CURLOPT_WRITEDATA, &readBuffer);

  CURLcode res = curl_easy_perform(state->client);
  curl_slist_free_all(headers);

  if (res != CURLE_OK) {
    throw std::runtime_error("Request failed: " +
                             std::string(curl_easy_strerror(res)));
  }

  Json::CharReaderBuilder reader;
  Json::Value root;
  std::string errs;
  std::istringstream s(readBuffer);

  if (!Json::parseFromStream(reader, s, &root, &errs)) {
    throw std::runtime_error("Failed to parse JSON: " + errs);
  }

  std::vector<Owner> authorizedUsers;
  for (const auto &item : root) {
    Owner owner;
    owner.id = item["owner"]["id"].asString();
    owner.name = item["owner"]["name"].asString();
    owner.email = item["owner"]["email"].asString();
    owner.twoFactorAuthEnabled = item["owner"]["twoFactorAuthEnabled"].asBool();
    owner.type = item["owner"]["type"].asString();

    if (owner.email == email) {
      authorizedUsers.push_back(owner);
    }
  }

  return authorizedUsers;
}
