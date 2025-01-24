#pragma once

#include <assert.h>
#include <stdint.h>

#if __cplusplus >= 201402L
#define __NOEXCEPT noexcept
#define __CONSTEXPR constexpr
#define __CONSTRUCT_CONSTEXPR constexpr
#else
#define __NOEXCEPT
#define __CONSTEXPR const
#define __CONSTRUCT_CONSTEXPR
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
__CONSTEXPR Nanoseconds __NANOSECONDS_NONE = uint32_t(-1);

struct OptionalDuration;

/// std::time::Duration
struct Duration {
    // leading bit incompatibility here. Rust `Duration` uses u64 for secs.
    // But int64_t here for timespec compatibility and simpler multi platform
    // support.
    int64_t secs;
    Nanoseconds nanos;
    uint32_t _padding;

#if __cplusplus
    Duration() = default;
    __CONSTRUCT_CONSTEXPR Duration(int64_t secs, Nanoseconds nanos) __NOEXCEPT
        : secs(secs),
          nanos(nanos),
          _padding(0) {}
    Duration(const Duration& other) __NOEXCEPT {
        this->secs = other.secs;
        this->nanos = other.nanos;
        // ignore padding
    }
    Duration(Duration&& other) __NOEXCEPT {
        this->secs = other.secs;
        this->nanos = other.nanos;
        // ignore padding
    }
    static __CONSTEXPR Duration ZERO() __NOEXCEPT {
        return {0, 0};
    }
    static __CONSTEXPR Duration from_parts(int64_t secs, Nanoseconds nanos)
            __NOEXCEPT {
        if (nanos < NANOS_PER_SEC) {
            return {secs, nanos};
        } else {
            return {secs + nanos / NANOS_PER_SEC, nanos % NANOS_PER_SEC};
        }
    }

    static __CONSTEXPR Duration from_secs(uint64_t secs) __NOEXCEPT {
        return Duration::from_parts(secs, 0);
    }
    static __CONSTEXPR Duration from_nanos(uint64_t nanos) __NOEXCEPT {
        const uint64_t NANOS_PER_SEC64 = NANOS_PER_SEC;
        const auto secs = nanos / NANOS_PER_SEC64;
        const auto subsec_nanos = (Nanoseconds)(nanos % NANOS_PER_SEC64);
        return Duration::from_parts(secs, subsec_nanos);
    }
    uint64_t __CONSTEXPR as_secs() const __NOEXCEPT {
        return secs;
    }
    Nanoseconds __CONSTEXPR subsec_nanos() const __NOEXCEPT {
        return nanos;
    }
    uint64_t __CONSTEXPR as_nanos64() const __NOEXCEPT {
        __CONSTEXPR Duration MAXIMUM_DURATION =
                Duration::from_nanos(UINT64_MAX);
        if (*this >= MAXIMUM_DURATION) {
            return UINT64_MAX;
        }
        return secs * NANOS_PER_SEC + nanos;
    }
    OptionalDuration checked_sub(const Duration& rhs) const __NOEXCEPT;
    Duration& operator=(const Duration& other) __NOEXCEPT {
        this->secs = other.secs;
        this->nanos = other.nanos;
        // ignore padding
        return *this;
    }
    // implementing checked_add in C++ is not trivial
    __CONSTEXPR Duration wrapping_add(const Duration& other) const __NOEXCEPT {
        const auto nanos = this->nanos + other.nanos;
        if (nanos < NANOS_PER_SEC) {
            return {secs + other.secs, nanos};
        } else {
            return {secs + other.secs + 1, nanos - NANOS_PER_SEC};
        }
    }
    // FIXME: implement checked add
    __CONSTEXPR Duration operator+(const Duration& rhs) const __NOEXCEPT {
        return wrapping_add(rhs);
    }
    Duration operator-(const Duration& rhs) const __NOEXCEPT;
    __CONSTEXPR bool operator==(const Duration& rhs) const __NOEXCEPT {
        return this->secs == rhs.secs && this->nanos == rhs.nanos;
    }
    __CONSTEXPR bool operator!=(const Duration& rhs) const __NOEXCEPT {
        return !(*this == rhs);
    }
    __CONSTEXPR bool operator<(const Duration& rhs) const __NOEXCEPT {
        return this->secs < rhs.secs ||
               (this->secs == rhs.secs && this->nanos < rhs.nanos);
    }
    __CONSTEXPR bool operator<=(const Duration& rhs) const __NOEXCEPT {
        return this->secs < rhs.secs ||
               (this->secs == rhs.secs && this->nanos <= rhs.nanos);
    }
    __CONSTEXPR bool operator>(const Duration& rhs) const __NOEXCEPT {
        return !(*this <= rhs);
    }
    __CONSTEXPR bool operator>=(const Duration& rhs) const __NOEXCEPT {
        return !(*this < rhs);
    }
    __CONSTEXPR bool is_zero() const __NOEXCEPT {
        return secs == 0 && nanos == 0;
    }
#if defined(__unix__) || defined(__APPLE__)
    struct timespec& as_timespec() __NOEXCEPT {
        return *reinterpret_cast<struct timespec*>(this);
    }
    Duration& operator=(const struct timespec& other) __NOEXCEPT {
        this->secs = other.tv_sec;
        this->nanos = other.tv_nsec;
        // ignore padding
        return *this;
    }
#endif
#endif
};
static_assert(sizeof(Duration) == sizeof(uint64_t) * 2, "Duration size");

struct OptionalDuration {
    Duration inner;

#if __cplusplus
    static __CONSTEXPR OptionalDuration none() __NOEXCEPT {
        return {{0, __NANOSECONDS_NONE}};
    }
    Duration unwrap() const __NOEXCEPT {
        return inner;
    }
    Duration expect() const __NOEXCEPT {
        assert(is_some());
        return inner;
    }
    bool is_none() const __NOEXCEPT {
        return inner.nanos == __NANOSECONDS_NONE;
    }
    bool is_some() const __NOEXCEPT {
        return !is_none();
    }
#endif
};

#if __cplusplus
inline OptionalDuration Duration::checked_sub(const Duration& rhs) const
        __NOEXCEPT {
    if (this->secs < rhs.secs) {
        return OptionalDuration::none();
    }
    auto secs_ = this->secs - rhs.secs;
    Nanoseconds nanos_;
    if (this->nanos >= rhs.nanos) {
        nanos_ = this->nanos - rhs.nanos;
    } else if (secs_ >= 1) {
        secs_ -= 1;
        nanos_ = this->nanos + NANOS_PER_SEC - rhs.nanos;
    } else {
        return OptionalDuration::none();
    }
    assert(nanos_ < NANOS_PER_SEC);
    return {{secs_, nanos_}};
}
inline Duration Duration::operator-(const Duration& rhs) const __NOEXCEPT {
    const auto duration = checked_sub(rhs);
    return duration.expect();  // "overflow when subtracting durations"
}
#endif

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
inline uint64_t _mul_div_u64(uint64_t value, uint64_t numer, uint64_t denom)
        __NOEXCEPT {
    const auto q = value / denom;
    const auto r = value % denom;
    return q * numer + r * numer / denom;
}

inline int64_t _perf_counter_query_frequency() __NOEXCEPT {
    int64_t frequency;
    QueryPerformanceFrequency(reinterpret_cast<LARGE_INTEGER*>(&frequency));
    return frequency;
}

inline int64_t _perf_counter_query() __NOEXCEPT {
    int64_t counter;
    QueryPerformanceCounter(reinterpret_cast<LARGE_INTEGER*>(&counter));
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
    Duration t;

#if defined(__unix__) || defined(__APPLE__)

    static Instant now() __NOEXCEPT {
        struct Instant instant;
#if defined(__APPLE__)
        clockid_t clock_id = CLOCK_UPTIME_RAW;
#else
        clockid_t clock_id = CLOCK_MONOTONIC;
#endif
        struct timespec t;
        clock_gettime(clock_id, &t);
        instant.t = t;
        return instant;
    }

    OptionalDuration checked_duration_since(const Instant& other) const
            __NOEXCEPT {
        return this->t.checked_sub(other.t);
    }
#endif
#if defined(_WIN32)
    static Instant now() __NOEXCEPT {
        const auto now = _PerformanceCounterInstant::now();
        return Instant::from(now);
    }

    static Instant from(const _PerformanceCounterInstant& other) __NOEXCEPT {
        const uint64_t freq = _PerformanceCounterInstant::frequency();
        const auto instant_nsec =
                _mul_div_u64((uint64_t)other.ts, NANOS_PER_SEC, freq);
        return Instant{Duration::from_nanos(instant_nsec)};
    }

    OptionalDuration checked_duration_since(const Instant& other) const
            __NOEXCEPT {
        const auto epsilon = _PerformanceCounterInstant::epsilon();
        if (other.t > this->t && other.t - this->t < epsilon) {
            return {{0, 0}};
        } else {
            return this->t.checked_sub(other.t);
        }
    }
#endif

    Duration operator-(const Instant& rhs) const __NOEXCEPT {
        return duration_since(rhs);
    }

    Duration saturating_duration_since(const Instant& earlier) const
            __NOEXCEPT {
        const auto duration = checked_duration_since(earlier);
        if (duration.is_none()) {
            return {0, 0};
        }
        return duration.inner;
    }
    Duration duration_since(const Instant& earlier) const __NOEXCEPT {
        assert(this->t.nanos < 1000000000);
        assert(earlier.t.nanos < 1000000000);
        const auto duration = checked_duration_since(earlier);
        return duration.expect();
    }

    Duration elapsed() const __NOEXCEPT {
        return Instant::now().saturating_duration_since(*this);
    }

    __CONSTEXPR Instant operator+(const Duration& other) const __NOEXCEPT {
        return {t + other};
    }
    Instant& operator+=(const Duration& other) __NOEXCEPT {
        *this = *this + other;
        return *this;
    }

    __CONSTEXPR bool operator==(const Instant& rhs) const __NOEXCEPT {
        return this->t == rhs.t;
    }
    __CONSTEXPR bool operator!=(const Instant& rhs) const __NOEXCEPT {
        return !(*this == rhs);
    }
    __CONSTEXPR bool operator<(const Instant& rhs) const __NOEXCEPT {
        return this->t < rhs.t;
    }
    __CONSTEXPR bool operator<=(const Instant& rhs) const __NOEXCEPT {
        return this->t <= rhs.t;
    }
    __CONSTEXPR bool operator>(const Instant& rhs) const __NOEXCEPT {
        return this->t > rhs.t;
    }
    __CONSTEXPR bool operator>=(const Instant& rhs) const __NOEXCEPT {
        return this->t >= rhs.t;
    }
};
static_assert(sizeof(Instant) == sizeof(uint64_t) * 2, "Instant size");

inline void init() __NOEXCEPT {
#if defined(_WIN32)
    _rust_time_static.perf_counter_frequency = _perf_counter_query_frequency();
#endif
}

#if __cplusplus
}  // namespace time
}  // namespace rust
#endif

#undef __NOEXCEPT
