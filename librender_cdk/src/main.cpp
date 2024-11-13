#include "../cpp-httplib/httplib.h"
#include <iostream>

int main() {
  httplib::Client cli("http://example.com");
  auto res = cli.Get("/api/v1/resource");

  if (res) {
    std::cout << "Status: " << res->status << std::endl;
    std::cout << "Response Body: " << res->body << std::endl;
  }
}
