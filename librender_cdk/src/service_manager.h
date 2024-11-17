// service_manager.h
#ifndef SERVICE_MANAGER_H
#define SERVICE_MANAGER_H

#include <curl/curl.h>
#include <map>
#include <nlohmann/json.hpp>
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
  ServiceManager(const std::string &api_key, const std::string &limit = "100");

  std::vector<Service>
  list_services(const std::map<std::string, std::string> &options = {});

private:
  std::string api_key_;
  std::string limit_;

  // Declare WriteCallback as a static member function.
  static size_t WriteCallback(void *contents, size_t size, size_t nmemb,
                              std::string *response);
};

#endif
