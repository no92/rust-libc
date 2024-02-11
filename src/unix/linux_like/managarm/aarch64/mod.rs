use crate::prelude::*;
use crate::{
    off64_t,
    off_t,
};

pub type nlink_t = c_uint;
pub type blksize_t = c_int;

pub type stat64 = stat;

s! {
    pub struct stat {
        pub st_dev: crate::dev_t,
        pub st_ino: crate::ino_t,
        pub st_mode: crate::mode_t,
        pub st_nlink: crate::nlink_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        pub st_rdev: crate::dev_t,
        __pad1: Padding<crate::dev_t>,
        pub st_size: off_t,
        pub st_blksize: crate::blksize_t,
        __pad2: Padding<c_int>,
        pub st_blocks: crate::blkcnt_t,
        pub st_atime: crate::time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: crate::time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: crate::time_t,
        pub st_ctime_nsec: c_long,
        __unused: [c_int; 2],
    }

    pub struct ucontext_t {
        pub uc_flags: c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: crate::stack_t,
        pub uc_sigmask: crate::sigset_t,
        pub uc_mcontext: mcontext_t,
    }

    #[repr(align(16))]
    pub struct mcontext_t {
        pub fault_address: c_ulonglong,
        pub regs: [c_ulonglong; 31],
        pub sp: c_ulonglong,
        pub pc: c_ulonglong,
        pub pstate: c_ulonglong,
        __reserved: [u64; 512],
    }
}

s_no_extra_traits! {
    #[repr(align(16))]
    pub struct max_align_t {
        priv_: [f32; 8],
    }
}

pub const O_DIRECTORY: c_int = 0x4000;
pub const O_NOFOLLOW: c_int = 0x8000;
pub const O_DIRECT: c_int = 0x10000;
pub const O_LARGEFILE: c_int = 0x20000;
