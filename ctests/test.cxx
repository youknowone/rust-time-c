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
void rust_time_init() {
    rust::time::init();
}
void rust_time_instant_now(rust::time::Instant* instant) {
    *instant = rust::time::Instant::now();
}
void rust_time_instant_duration_since(
        const rust::time::Instant* lhs, const rust::time::Instant* rhs,
        rust::time::Duration* duration) {
    *duration = lhs->duration_since(*rhs);
}

void rust_time_duration_add(
        const rust::time::Duration* lhs, const rust::time::Duration* rhs,
        rust::time::Duration* result) {
    *result = *lhs + *rhs;
}
void rust_time_duration_sub(
        const rust::time::Duration* lhs, const rust::time::Duration* rhs,
        rust::time::Duration* result) {
    *result = *lhs - *rhs;
}

bool rust_time_duration_eq(
        const rust::time::Duration* lhs, const rust::time::Duration* rhs) {
    return *lhs == *rhs;
}
bool rust_time_duration_ne(
        const rust::time::Duration* lhs, const rust::time::Duration* rhs) {
    return *lhs != *rhs;
}
bool rust_time_duration_lt(
        const rust::time::Duration* lhs, const rust::time::Duration* rhs) {
    return *lhs < *rhs;
}
bool rust_time_duration_le(
        const rust::time::Duration* lhs, const rust::time::Duration* rhs) {
    return *lhs <= *rhs;
}
bool rust_time_duration_gt(
        const rust::time::Duration* lhs, const rust::time::Duration* rhs) {
    return *lhs > *rhs;
}
bool rust_time_duration_ge(
        const rust::time::Duration* lhs, const rust::time::Duration* rhs) {
    return *lhs >= *rhs;
}

void rust_time_instant_elapsed(
        const rust::time::Instant* instant, rust::time::Duration* duration) {
    *duration = instant->elapsed();
}
void rust_time_instant_add(
        const rust::time::Instant* lhs, const rust::time::Duration* rhs,
        rust::time::Instant* result) {
    *result = *lhs + *rhs;
}
void rust_time_instant_sub(
        const rust::time::Instant* lhs, const rust::time::Instant* rhs,
        rust::time::Duration* result) {
    *result = *lhs - *rhs;
}

bool rust_time_instant_eq(
        const rust::time::Instant* lhs, const rust::time::Instant* rhs) {
    return *lhs == *rhs;
}
bool rust_time_instant_ne(
        const rust::time::Instant* lhs, const rust::time::Instant* rhs) {
    return *lhs != *rhs;
}
bool rust_time_instant_lt(
        const rust::time::Instant* lhs, const rust::time::Instant* rhs) {
    return *lhs < *rhs;
}
bool rust_time_instant_le(
        const rust::time::Instant* lhs, const rust::time::Instant* rhs) {
    return *lhs <= *rhs;
}
bool rust_time_instant_gt(
        const rust::time::Instant* lhs, const rust::time::Instant* rhs) {
    return *lhs > *rhs;
}
bool rust_time_instant_ge(
        const rust::time::Instant* lhs, const rust::time::Instant* rhs) {
    return *lhs >= *rhs;
}

}  // extern "C"