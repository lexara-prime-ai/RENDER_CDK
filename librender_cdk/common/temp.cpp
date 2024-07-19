#include <curl/curl.h>
#include <json/json.h>
#include <iostream>
#include <string>
#include <vector>
#include <stdexcept>
#include <memory>

// sudo apt-get update
// sudo apt-get install libcurl4-openssl-dev libjsoncpp-dev g++


const std::string BASE_URL = "https://api.render.com/v1";

class EnvironmentManager {
public:
    static std::string retrieve_api_key() {
        // Replace with the actual implementation to retrieve the API key.
        return "your_api_key";
    }
};

struct Owner {
    std::string id;
    std::string name;
    std::string email;
    bool twoFactorAuthEnabled;
    std::string type_;

    static std::vector<Owner> list_authorized_users(const std::string& email, const std::string& limit);
};

struct OwnerResponse {
    Owner owner;
    std::string cursor;

    static std::vector<OwnerResponse> parse_json(const Json::Value& json);
};

static size_t WriteCallback(void* contents, size_t size, size_t nmemb, void* userp) {
    ((std::string*)userp)->append((char*)contents, size * nmemb);
    return size * nmemb;
}

std::vector<OwnerResponse> OwnerResponse::parse_json(const Json::Value& json) {
    std::vector<OwnerResponse> owners;
    for (const auto& item : json) {
        Owner owner;
        owner.id = item["id"].asString();
        owner.name = item["name"].asString();
        owner.email = item["email"].asString();
        owner.twoFactorAuthEnabled = item["twoFactorAuthEnabled"].asBool();
        owner.type_ = item["type"].asString();

        OwnerResponse response;
        response.owner = owner;
        response.cursor = item["cursor"].asString();
        owners.push_back(response);
    }
    return owners;
}

std::vector<Owner> Owner::list_authorized_users(const std::string& email, const std::string& limit) {
    CURL* curl;
    CURLcode res;
    std::string readBuffer;
    std::string api_key = "Bearer " + EnvironmentManager::retrieve_api_key();

    curl = curl_easy_init();
    if (curl) {
        std::string url = BASE_URL + "/owners?limit=" + limit;

        struct curl_slist* headers = nullptr;
        headers = curl_slist_append(headers, "Accept: application/json");
        headers = curl_slist_append(headers, ("Authorization: " + api_key).c_str());

        curl_easy_setopt(curl, CURLOPT_URL, url.c_str());
        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, &readBuffer);

        res = curl_easy_perform(curl);

        if (res != CURLE_OK) {
            fprintf(stderr, "curl_easy_perform() failed: %s\n", curl_easy_strerror(res));
        }

        curl_easy_cleanup(curl);
        curl_slist_free_all(headers);

        Json::CharReaderBuilder readerBuilder;
        Json::Value jsonData;
        std::string errs;

        std::unique_ptr<Json::CharReader> reader(readerBuilder.newCharReader());
        if (reader->parse(readBuffer.c_str(), readBuffer.c_str() + readBuffer.size(), &jsonData, &errs)) {
            std::vector<OwnerResponse> owner_responses = OwnerResponse::parse_json(jsonData);
            std::vector<Owner> owners;
            for (const auto& response : owner_responses) {
                if (response.owner.email == email) {
                    owners.push_back(response.owner);
                }
            }
            return owners;
        }
        else {
            throw std::runtime_error("Failed to parse JSON response: " + errs);
        }
    }

    return {};
}

void test_list_authorized_users() {
    std::vector<Owner> result = Owner::list_authorized_users("<user>@<email>.com", "100");

    // The result should not be empty.
    if (!result.empty()) {
        std::cout << "Test passed: authorized users found." << std::endl;
    }
    else {
        std::cout << "Test failed: no authorized users found." << std::endl;
    }
}

int main() {
    test_list_authorized_users();
    return 0;
}
