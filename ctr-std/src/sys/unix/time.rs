// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cmp::Ordering;
use libc;
use time::Duration;

pub use self::inner::{Instant, SystemTime, UNIX_EPOCH};

const NSEC_PER_SEC: u64 = 1_000_000_000;

#[derive(Copy, Clone)]
struct Timespec {
    t: libc::timespec,
}

impl Timespec {
    fn sub_timespec(&self, other: &Timespec) -> Result<Duration, Duration> {
        if self >= other {
            Ok(if self.t.tv_nsec >= other.t.tv_nsec {
                Duration::new((self.t.tv_sec - other.t.tv_sec) as u64,
                              (self.t.tv_nsec - other.t.tv_nsec) as u32)
            } else {
                Duration::new((self.t.tv_sec - 1 - other.t.tv_sec) as u64,
                              self.t.tv_nsec as u32 + (NSEC_PER_SEC as u32) -
                              other.t.tv_nsec as u32)
            })
        } else {
            match other.sub_timespec(self) {
                Ok(d) => Err(d),
                Err(d) => Ok(d),
            }
        }
    }

    fn add_duration(&self, other: &Duration) -> Timespec {
        let secs = (self.t.tv_sec as i64).checked_add(other.as_secs() as i64);
        let mut secs = secs.expect("overflow when adding duration to time");

        // Nano calculations can't overflow because nanos are <1B which fit
        // in a u32.
        let mut nsec = other.subsec_nanos() + self.t.tv_nsec as u32;
        if nsec >= NSEC_PER_SEC as u32 {
            nsec -= NSEC_PER_SEC as u32;
            secs = secs.checked_add(1).expect("overflow when adding \
                                               duration to time");
        }
        Timespec {
            t: libc::timespec {
                tv_sec: secs as libc::time_t,
                tv_nsec: nsec as libc::c_long,
            },
        }
    }

    fn sub_duration(&self, other: &Duration) -> Timespec {
        let secs = (self.t.tv_sec as i64).checked_sub(other.as_secs() as i64);
        let mut secs = secs.expect("overflow when subtracting duration \
                                    from time");

        // Similar to above, nanos can't overflow.
        let mut nsec = self.t.tv_nsec as i32 - other.subsec_nanos() as i32;
        if nsec < 0 {
            nsec += NSEC_PER_SEC as i32;
            secs = secs.checked_sub(1).expect("overflow when subtracting \
                                               duration from time");
        }
        Timespec {
            t: libc::timespec {
                tv_sec: secs as libc::time_t,
                tv_nsec: nsec as libc::c_long,
            },
        }
    }
}

impl PartialEq for Timespec {
    fn eq(&self, other: &Timespec) -> bool {
        self.t.tv_sec == other.t.tv_sec && self.t.tv_nsec == other.t.tv_nsec
    }
}

impl Eq for Timespec {}

impl PartialOrd for Timespec {
    fn partial_cmp(&self, other: &Timespec) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timespec {
    fn cmp(&self, other: &Timespec) -> Ordering {
        let me = (self.t.tv_sec, self.t.tv_nsec);
        let other = (other.t.tv_sec, other.t.tv_nsec);
        me.cmp(&other)
    }
}

mod inner {
    use fmt;
    use libc;
    use sys::cvt;
    use time::Duration;

    use super::Timespec;

    use spin;
    use libctru;

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Instant {
        t: Timespec,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SystemTime {
        t: Timespec,
    }

    pub const UNIX_EPOCH: SystemTime = SystemTime {
        t: Timespec {
            t: libc::timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
        },
    };

    impl Instant {
        pub fn now() -> Instant {
            let ms = monotonic_ms();

            let s = libc::timeval {
                tv_sec: (ms / 1_000_000) as libc::time_t,
                tv_usec: (ms % 1_000_000) as libc::c_long,
            };
            return Instant::from(s)
        }

        pub fn sub_instant(&self, other: &Instant) -> Duration {
            self.t.sub_timespec(&other.t).unwrap_or_else(|_| {
                panic!("other was less than the current instant")
            })
        }

        pub fn add_duration(&self, other: &Duration) -> Instant {
            Instant { t: self.t.add_duration(other) }
        }

        pub fn sub_duration(&self, other: &Duration) -> Instant {
            Instant { t: self.t.sub_duration(other) }
        }
    }

    // The initial system tick after which all Instants occur
    static TICK: spin::Once<u64> = spin::Once::new();

    // Returns a monotonic timer in microseconds
    //
    // Note that svcGetSystemTick always runs at 268MHz, even on a
    // New 3DS running in 804MHz mode
    //
    // See https://www.3dbrew.org/wiki/Hardware#Common_hardware
    fn monotonic_ms() -> u64 {
        let first_tick = get_first_tick();
        let current_tick = get_system_tick();
        (current_tick - first_tick / 268)
    }

    // The first time this function is called, it generates and returns the
    // initial system tick used to create Instants
    //
    // subsequent calls to this function return the previously generated
    // tick value
    fn get_first_tick() -> u64 {
        *TICK.call_once(get_system_tick)
    }

    // Gets the current system tick
    fn get_system_tick() -> u64 {
        unsafe { libctru::svc::svcGetSystemTick() }
    }

    impl fmt::Debug for Instant {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Instant")
             .field("tv_sec", &self.t.t.tv_sec)
             .field("tv_nsec", &self.t.t.tv_nsec)
             .finish()
        }
    }

    impl From<libc::timeval> for Instant {
        fn from(t: libc::timeval) -> Instant {
            Instant::from(libc::timespec {
                tv_sec: t.tv_sec,
                tv_nsec: (t.tv_usec * 1000) as libc::c_long,
            })
        }
    }

    impl From<libc::timespec> for Instant {
        fn from(t: libc::timespec) -> Instant {
            Instant { t: Timespec { t: t } }
        }
    }

    impl SystemTime {
        pub fn now() -> SystemTime {
            use ptr;

            let mut s = libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            cvt(unsafe {
                libc::gettimeofday(&mut s, ptr::null_mut())
            }).unwrap();
            return SystemTime::from(s)
        }

        pub fn sub_time(&self, other: &SystemTime)
                        -> Result<Duration, Duration> {
            self.t.sub_timespec(&other.t)
        }

        pub fn add_duration(&self, other: &Duration) -> SystemTime {
            SystemTime { t: self.t.add_duration(other) }
        }

        pub fn sub_duration(&self, other: &Duration) -> SystemTime {
            SystemTime { t: self.t.sub_duration(other) }
        }
    }

    impl From<libc::timeval> for SystemTime {
        fn from(t: libc::timeval) -> SystemTime {
            SystemTime::from(libc::timespec {
                tv_sec: t.tv_sec,
                tv_nsec: (t.tv_usec * 1000) as libc::c_long,
            })
        }
    }

    impl From<libc::timespec> for SystemTime {
        fn from(t: libc::timespec) -> SystemTime {
            SystemTime { t: Timespec { t: t } }
        }
    }

    impl fmt::Debug for SystemTime {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("SystemTime")
             .field("tv_sec", &self.t.t.tv_sec)
             .field("tv_nsec", &self.t.t.tv_nsec)
             .finish()
        }
    }
}
