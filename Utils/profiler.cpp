// Tiny C++ profiling function
#include <chrono>

void profile(std::string message) {
  static auto t = std::chrono::high_resolution_clock::now().time_since_epoch().count();
  auto now = std::chrono::high_resolution_clock::now().time_since_epoch().count();
  printf("%s: %lums\n", message.c_str(), (now - t) / 1'000'000'000);
  t = now;
}
