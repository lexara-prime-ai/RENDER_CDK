// service_manager.h
#ifndef SERVICE_MANAGER_H
#define SERVICE_MANAGER_H

#include <curl/curl.h>
#include <jsoncpp/json/json.h>
#include <string>
#include <vector>

struct Service {
  std::string id;
  std::string name;
  std::string branch;
  std::string dashboardUrl;
  std::string type;
  std::string repo;
  std::string createdAt;
  std::string updatedAt;
};

class ServiceManager {
public:
  ServiceManager(const std::string &api_key);
  std::vector<Service> list_services();

private:
  std::string api_key_;

  // Declare WriteCallback as a static member function.
  static size_t WriteCallback(void *contents, size_t size, size_t nmemb,
                              std::string *response);
};

#endif
