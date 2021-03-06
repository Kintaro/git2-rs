use libc::{c_int, c_uint};

use raw;
use util::Binding;

/// Time in a signature
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Time {
    raw: raw::git_time,
}

/// Time structure used in a git index entry.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IndexTime {
    raw: raw::git_index_time,
}

impl Time {
    /// Creates a new time structure from its components.
    pub fn new(time: i64, offset: i32) -> Time {
        unsafe {
            Binding::from_raw(raw::git_time {
                time: time as raw::git_time_t,
                offset: offset as c_int,
            })
        }
    }

    /// Return the time, in seconds, from epoch
    pub fn seconds(&self) -> i64 { self.raw.time as i64 }

    /// Return the timezone offset, in minutes
    pub fn offset_minutes(&self) -> i32 { self.raw.offset as i32 }
}

impl Binding for Time {
    type Raw = raw::git_time;
    unsafe fn from_raw(raw: raw::git_time) -> Time {
        Time { raw: raw }
    }
    fn raw(&self) -> raw::git_time { self.raw }
}

impl IndexTime {
    /// Creates a new time structure from its components.
    pub fn new(seconds: i64, nanoseconds: u32) -> IndexTime {
        unsafe {
            Binding::from_raw(raw::git_index_time {
                seconds: seconds as raw::git_time_t,
                nanoseconds: nanoseconds as c_uint,
            })
        }
    }

    /// Returns the number of seconds in the second component of this time.
    pub fn seconds(&self) -> i64 { self.raw.seconds as i64 }
    /// Returns the nanosecond component of this time.
    pub fn nanoseconds(&self) -> u32 { self.raw.nanoseconds as u32 }
}

impl Binding for IndexTime {
    type Raw = raw::git_index_time;
    unsafe fn from_raw(raw: raw::git_index_time) -> IndexTime {
        IndexTime { raw: raw }
    }
    fn raw(&self) -> raw::git_index_time { self.raw }
}
