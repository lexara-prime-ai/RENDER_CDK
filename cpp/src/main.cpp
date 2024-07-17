// Copyright (c) 2024 Irfan M. Ghat <irfanghat@gmail.com>
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright
//       notice, this list of conditions and the following disclaimer.
//
//     * Redistributions in binary form must reproduce the above
//       copyright notice, this list of conditions and the following
//       disclaimer in the documentation and/or other materials provided
//       with the distribution.
//
//     * Neither the name of copyright holder nor the names of other
//       contributors may be used to endorse or promote products derived
//       from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
///
/// \see https://github.com/lexara-prime-api/RENDER_CDK

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


    <DOCS>
    LINK : https://riptutorial.com/cmake/example/7501/simple--hello-world--project

    LINk: https://iq.opengenus.org/create-shared-library-in-cpp/

    LINK: https://en.cppreference.com/w/cpp/links/libs
*
*****************************************************************/

#include <iostream>
#include <string>
#include <curl/curl.h>
#include <jsoncpp/json/json.h>
#include "../include/dotenv.h"

using namespace std;

class EnvironmentManager {
public:
  static std::string getApiKey() {
    // Load <ENVIRONMENT> variables
    dotenv::init("cpp.env");
    const std::string apiKey = getenv("API_KEY");
    return apiKey;
  }
};

size_t WriteCallback(void* contents, size_t size, size_t nmemb, void* userp) {
  ((std::string*)userp)->append((char*)contents, size * nmemb);
  return size * nmemb;
}

class ServiceManager {
public:
  static std::string listAllServices(const std::string& limit) {
    const std::string BASE_URL = "https://api.render.com/v1";
    const std::string API_URL = BASE_URL + "/services?limit=" + limit;
    const std::string API_KEY = EnvironmentManager::getApiKey();

    // Check if <API_KEY> has been loaded
    if (!API_KEY.empty()) {
      cout << "\nOUTPUT: [Redacted]\n" << endl;

      // CURL config.
      CURL* curl;
      CURLcode res;
      std::string readBuffer;

      curl_global_init(CURL_GLOBAL_DEFAULT);
      curl = curl_easy_init();

      if (curl) {
        struct curl_slist* headers = NULL;
        headers = curl_slist_append(headers, "Accept: application/json");
        headers = curl_slist_append(headers, ("Authorization: Bearer " + API_KEY).c_str());

        curl_easy_setopt(curl, CURLOPT_URL, API_URL.c_str());
        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, &readBuffer);

        res = curl_easy_perform(curl);

        if (res != CURLE_OK) {
          fprintf(stderr, "curl_easy_perform() failed: %s\n", curl_easy_strerror(res));
          curl_easy_cleanup(curl);
          curl_global_cleanup();
          return "An [ERROR] occurred!";
        }

          curl_easy_cleanup(curl);
          curl_global_cleanup();

          cout << readBuffer << endl;

          return readBuffer;
      } else {
        cerr << "[CURL] initialization failed!" << endl;
      }
    } else {
      cout << "[API_KEY] has NOT been set!" << endl;
        return 0;
    }
    return 0;
  }
};



int main() {
  ServiceManager::listAllServices("10");
  return 0;
}