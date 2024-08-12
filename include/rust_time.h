#pragma once

#include <assert.h>
#include <stdint.h>

#if __cplusplus >= 201402L
#define __NOEXCEPT noexcept
#define __CONSTEXPR constexpr
#else
#define __NOEXCEPT
#define __CONSTEXPR const
#endif

#if defined(__unix__) || defined(__APPLE__)
#include <sys/time.h>
#include <sys/types.h>
#include <time.h>
#elif defined(_WIN32)
#include <Windows.h>
#endif

#if __cplusplus
namespace rust {
namespace time {
#endif

typedef uint32_t Nanoseconds;

__CONSTEXPR uint32_t NANOS_PER_SEC = 1000000000;
__CONSTEXPR Nanoseconds NANOSECONDS_NONE = uint32_t(-1);

/// std::time::Duration
struct Duration {
  uint64_t secs;
  Nanoseconds nanos;

#if __cplusplus
  Duration() = default;
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
  Duration checked_sub(const Duration &rhs) const {
    if (this->secs < rhs.secs) {
      return {0, NANOSECONDS_NONE};
    }
    auto secs_ = this->secs - rhs.secs;
    Nanoseconds nanos_;
    if (this->nanos >= rhs.nanos) {
      nanos_ = this->nanos - rhs.nanos;
    } else if (secs_ >= 1) {
      secs_ -= 1;
      nanos_ = this->nanos + NANOS_PER_SEC - rhs.nanos;
    } else {
      return {0, NANOSECONDS_NONE};
    }
    assert(nanos_ < NANOS_PER_SEC);
    return {secs_, nanos_};
  }
  Duration operator-(const Duration &rhs) const __NOEXCEPT {
    const auto duration = checked_sub(rhs);
    assert(duration.nanos != NANOSECONDS_NONE); // "overflow when subtracting durations"
    return duration;
  }
  bool operator==(const Duration &rhs) const __NOEXCEPT {
    return this->secs == rhs.secs && this->nanos == rhs.nanos;
  }
  bool operator!=(const Duration &rhs) const __NOEXCEPT {
    return !(*this == rhs);
  }
  bool operator<(const Duration &rhs) const __NOEXCEPT {
    return this->secs < rhs.secs ||
           (this->secs == rhs.secs && this->nanos < rhs.nanos);
  }
  bool operator<=(const Duration &rhs) const __NOEXCEPT {
    return this->secs < rhs.secs ||
        (this->secs == rhs.secs && this->nanos <= rhs.nanos);
  }
  bool operator>(const Duration &rhs) const __NOEXCEPT {
    return !(*this <= rhs);
  }
  bool operator>=(const Duration &rhs) const __NOEXCEPT {
    return !(*this < rhs);
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

struct _Static {};
#endif

#if defined(_WIN32)
struct _Static {
  int64_t perf_counter_frequency;
};
#endif

#if __cplusplus
extern "C" {
#endif
extern struct _Static _rust_time_static;
#if __cplusplus
}
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
  QueryPerformanceFrequency(reinterpret_cast<LARGE_INTEGER *>(&frequency));
  return frequency;
}

inline int64_t _perf_counter_query() __NOEXCEPT {
  int64_t counter;
  QueryPerformanceCounter(reinterpret_cast<LARGE_INTEGER *>(&counter));
  return counter;
}

struct _PerformanceCounterInstant {
  int64_t ts;

  static _PerformanceCounterInstant now() __NOEXCEPT {
    struct _PerformanceCounterInstant instant;
    instant.ts = _perf_counter_query();
    return instant;
  }
  static int64_t frequency() __NOEXCEPT {
    return _rust_time_static.perf_counter_frequency;
  }
  static Duration epsilon() __NOEXCEPT {
    const auto epsilon = NANOS_PER_SEC / (uint64_t)frequency();
    return Duration::from_nanos(epsilon);
  }
};
#endif

/// std::time::Instant
struct Instant {
#if defined(__unix__) || defined(__APPLE__)
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

  Duration _sub_timespec(const struct timespec &lhs,
                         const struct timespec &rhs) const __NOEXCEPT {
    const auto is_larger =
        lhs.tv_sec > rhs.tv_sec ||
        (lhs.tv_sec == rhs.tv_sec && lhs.tv_nsec > rhs.tv_nsec);
    if (is_larger) {
      uint64_t secs;
      Nanoseconds nsec;
      if (lhs.tv_nsec >= rhs.tv_nsec) {
        secs = lhs.tv_sec - rhs.tv_sec;
        nsec = lhs.tv_nsec - rhs.tv_nsec;
      } else {
        secs = lhs.tv_sec - rhs.tv_sec - 1;
        nsec = lhs.tv_nsec + NANOS_PER_SEC - rhs.tv_nsec;
      }
      return {secs, nsec};
    } else {
      return {0, NANOSECONDS_NONE};
    }
  }

  Duration checked_sub_instant(const Instant &other) const __NOEXCEPT {
    return _sub_timespec(this->t, other.t);
  }
#endif
#if defined(_WIN32)
  Duration t;

  static Instant now() __NOEXCEPT {
    const auto now = _PerformanceCounterInstant::now();
    return Instant::from(now);
  }

  static Instant from(const _PerformanceCounterInstant &other) __NOEXCEPT {
    const uint64_t freq = _PerformanceCounterInstant::frequency();
    const auto instant_nsec =
        _mul_div_u64((uint64_t)other.ts, NANOS_PER_SEC, freq);
    return Instant{Duration::from_nanos(instant_nsec)};
  }

  Duration checked_sub_instant(const Instant &other) const __NOEXCEPT {
    const auto epsilon = _PerformanceCounterInstant::epsilon();
    if (other.t > this->t && other.t - this->t < epsilon) {
      return {0, NANOSECONDS_NONE};
    } else {
      return this->t.checked_sub(other.t);
    }
  }
#endif

  Duration operator-(const Instant &rhs) const __NOEXCEPT {
    return duration_since(rhs);
  }

  Duration duration_since(const Instant &earlier) const __NOEXCEPT {
    const auto duration = checked_duration_since(earlier);
    assert(duration.nanos != NANOSECONDS_NONE);
    return duration;
  }

  Duration checked_duration_since(const Instant &earlier) const __NOEXCEPT {
    return checked_sub_instant(earlier);
  }
};

inline void init() __NOEXCEPT {
#if defined(_WIN32)
  _rust_time_static.perf_counter_frequency = _perf_counter_query_frequency();
#endif
}

#if __cplusplus
} // namespace time
} // namespace rust
#endif

#undef __NOEXCEPT
