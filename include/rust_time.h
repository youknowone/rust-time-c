#include <stdint.h>

#if __cplusplus >= 201402L
#define __NOEXCEPT noexcept
#else
#define __NOEXCEPT
#endif

#if defined(__unix__) || defined(__APPLE__)
#include <time.h>
#include <sys/time.h>
#include <sys/types.h>
#endif

#if __cplusplus
namespace rust {
namespace time {
#endif

typedef uint32_t Nanoseconds;

const uint32_t NANOS_PER_SEC = 1000000000;

/// std::time::Duration
struct Duration {
  uint64_t secs;
  Nanoseconds nanos;
#if __cplusplus
  Duration(uint64_t secs, Nanoseconds nanos) __NOEXCEPT {
    if (nanos < NANOS_PER_SEC) {
      this->secs = secs;
      this->nanos = nanos;
    } else {
      this->secs = secs + nanos / NANOS_PER_SEC;
      this->nanos = nanos % NANOS_PER_SEC;
    }
  }
  static Duration from_nanos(uint64_t nanos) __NOEXCEPT {
    const uint64_t NANOS_PER_SEC64 = NANOS_PER_SEC;
    const auto secs = nanos / NANOS_PER_SEC64;
    const auto subsec_nanos = (Nanoseconds)(nanos % NANOS_PER_SEC64);
    return Duration(secs, subsec_nanos);
  }
#endif
};

#if defined(__unix__) || defined(__APPLE__)
// Timespec::now
inline struct timespec _timespec_now(clockid_t clock) __NOEXCEPT {
  struct timespec t;
  clock_gettime(clock, &t);
  return t;
};

/// std::time::Instant
struct Instant {
  struct timespec t;

  static Instant now() __NOEXCEPT {
    struct Instant instant;
#if defined(__APPLE__)
    clockid_t clock_id = CLOCK_UPTIME_RAW;
#else
    clockid_t clock_id = CLOCK_MONOTONIC;
#endif
    clock_gettime(clock_id, &instant.t);
    return instant;
  }
};
#endif

#if defined(_WIN32)
uint64_t _mul_div_u64(uint64_t value, uint64_t numer,
                      uint64_t denom) __NOEXCEPT {
  const auto q = value / denom;
  const auto r = value % denom;
  return q * numer + r * numer / denom;
}

inline int64_t _perf_counter_query_frequency() __NOEXCEPT {
  int64_t frequency;
  QueryPerformanceFrequency(&frequency);
  return frequency;
}

extern int64_t _perf_counter_frequency;

inline int64_t _perf_counter_query() __NOEXCEPT {
  int64_t counter;
  QueryPerformanceCounter(&counter);
  return counter;
}

struct _PerformanceCounterInstant {
  int64_t ts;

  static _PerformanceCounterInstant now() __NOEXCEPT {
    struct _PerformanceCounterInstant instant;
    QueryPerformanceCounter(&instant.ts);
    return instant;
  }
}

/// std::time::Instant
struct Instant {
  Duration t;

  static Instant now() __NOEXCEPT {
    const auto now = _PerformanceCounterInstant::now();
    return Instant::from(now);
  }

  static Instant from(const _PerformanceCounterInstant &other) __NOEXCEPT {
    const uint64_t freq = _perf_counter_frequency;
    const auto instant_nsec =
        _mul_div_u64((uint64_t)other.ts, NANOS_PER_SEC, freq);
    return Instant{Duration::from_nanos(instant_nsec)};
  }
}
#endif

#if __cplusplus
} // namespace time
} // namespace rust
#endif

#undef __NOEXCEPT
