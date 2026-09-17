#include <cstdint>

struct Large {
  uint64_t a[3];
};

extern "C" Large cpp_return_large(uint64_t x) {
  return Large{x, 0, 0};
}
