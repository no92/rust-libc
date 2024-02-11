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
}

s_no_extra_traits! {
    pub struct ucontext_t {
        pub uc_flags: c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: crate::stack_t,
        pub uc_sigmask: crate::sigset_t,
        pub uc_mcontext: mcontext_t,
    }

    #[repr(align(16))]
    pub struct mcontext_t {
        pub __gregs: [c_ulong; 32],
        pub __fpregs: __riscv_mc_fp_state,
    }

    pub union __riscv_mc_fp_state {
        pub __f: __riscv_mc_f_ext_state,
        pub __d: __riscv_mc_d_ext_state,
        pub __q: __riscv_mc_q_ext_state,
    }

    pub struct __riscv_mc_f_ext_state {
        pub __f: [c_uint; 32],
        pub __fcsr: c_uint,
    }

    pub struct __riscv_mc_d_ext_state {
        pub __f: [c_ulonglong; 32],
        pub __fcsr: c_uint,
    }

    #[repr(align(16))]
    pub struct __riscv_mc_q_ext_state {
        pub __f: [c_ulonglong; 64],
        pub __fcsr: c_uint,
        pub __reserved: [c_uint; 3],
    }
}

pub const NGREG: usize = 32;
pub const REG_PC: usize = 0;
pub const REG_RA: usize = 1;
pub const REG_SP: usize = 2;
pub const REG_TP: usize = 4;
pub const REG_S0: usize = 8;
pub const REG_A0: usize = 10;

pub const O_DIRECT: c_int = 0x4000;
pub const O_LARGEFILE: c_int = 0x8000;
pub const O_DIRECTORY: c_int = 0x10000;
pub const O_NOFOLLOW: c_int = 0x20000;
