#include "rust_time.h"

extern "C" {
    void rust_time_instant_now(rust::time::Instant* instant) {
        *instant = rust::time::Instant::now();
    }
}
