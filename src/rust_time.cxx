#include "rust_time.h"

#if __cplusplus
namespace rust {
namespace time {
extern "C" {
#endif

struct _Static _rust_time_static;

#if __cplusplus
}
}
}
#endif

extern "C" {
void rust_time_init() { rust::time::init(); }
void rust_time_instant_now(rust::time::Instant *instant) {
  *instant = rust::time::Instant::now();
}
}
