// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/* automatically generated by rust-bindgen */

pub const __BITS_PER_LONG: u32 = 64;
pub const __FD_SETSIZE: u32 = 1024;
pub const VIRTIO_ID_NET: u32 = 1;
pub const VIRTIO_ID_BLOCK: u32 = 2;
pub const VIRTIO_ID_CONSOLE: u32 = 3;
pub const VIRTIO_ID_RNG: u32 = 4;
pub const VIRTIO_ID_BALLOON: u32 = 5;
pub const VIRTIO_ID_RPMSG: u32 = 7;
pub const VIRTIO_ID_SCSI: u32 = 8;
pub const VIRTIO_ID_9P: u32 = 9;
pub const VIRTIO_ID_RPROC_SERIAL: u32 = 11;
pub const VIRTIO_ID_CAIF: u32 = 12;
pub const VIRTIO_ID_GPU: u32 = 16;
pub const VIRTIO_ID_INPUT: u32 = 18;
pub const VIRTIO_CONFIG_S_ACKNOWLEDGE: u32 = 1;
pub const VIRTIO_CONFIG_S_DRIVER: u32 = 2;
pub const VIRTIO_CONFIG_S_DRIVER_OK: u32 = 4;
pub const VIRTIO_CONFIG_S_FEATURES_OK: u32 = 8;
pub const VIRTIO_CONFIG_S_FAILED: u32 = 128;
pub const VIRTIO_TRANSPORT_F_START: u32 = 28;
pub const VIRTIO_TRANSPORT_F_END: u32 = 33;
pub const VIRTIO_F_NOTIFY_ON_EMPTY: u32 = 24;
pub const VIRTIO_F_ANY_LAYOUT: u32 = 27;
pub const VIRTIO_F_VERSION_1: u32 = 32;
pub const VIRTIO_BLK_F_SIZE_MAX: u32 = 1;
pub const VIRTIO_BLK_F_SEG_MAX: u32 = 2;
pub const VIRTIO_BLK_F_GEOMETRY: u32 = 4;
pub const VIRTIO_BLK_F_RO: u32 = 5;
pub const VIRTIO_BLK_F_BLK_SIZE: u32 = 6;
pub const VIRTIO_BLK_F_TOPOLOGY: u32 = 10;
pub const VIRTIO_BLK_F_MQ: u32 = 12;
pub const VIRTIO_BLK_F_BARRIER: u32 = 0;
pub const VIRTIO_BLK_F_SCSI: u32 = 7;
pub const VIRTIO_BLK_F_FLUSH: u32 = 9;
pub const VIRTIO_BLK_F_CONFIG_WCE: u32 = 11;
pub const VIRTIO_BLK_F_WCE: u32 = 9;
pub const VIRTIO_BLK_ID_BYTES: u32 = 20;
pub const VIRTIO_BLK_T_IN: u32 = 0;
pub const VIRTIO_BLK_T_OUT: u32 = 1;
pub const VIRTIO_BLK_T_SCSI_CMD: u32 = 2;
pub const VIRTIO_BLK_T_FLUSH: u32 = 4;
pub const VIRTIO_BLK_T_GET_ID: u32 = 8;
pub const VIRTIO_BLK_T_BARRIER: u32 = 2147483648;
pub const VIRTIO_BLK_S_OK: u32 = 0;
pub const VIRTIO_BLK_S_IOERR: u32 = 1;
pub const VIRTIO_BLK_S_UNSUPP: u32 = 2;
pub type __s8 = ::std::os::raw::c_schar;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __s64 = ::std::os::raw::c_longlong;
pub type __u64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct __kernel_fd_set {
    pub fds_bits: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___kernel_fd_set() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_fd_set>())).fds_bits as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_fd_set),
            "::",
            stringify!(fds_bits)
        )
    );
}
pub type __kernel_sighandler_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type __kernel_key_t = ::std::os::raw::c_int;
pub type __kernel_mqd_t = ::std::os::raw::c_int;
pub type __kernel_old_uid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_gid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_dev_t = ::std::os::raw::c_ulong;
pub type __kernel_long_t = ::std::os::raw::c_long;
pub type __kernel_ulong_t = ::std::os::raw::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = ::std::os::raw::c_uint;
pub type __kernel_pid_t = ::std::os::raw::c_int;
pub type __kernel_ipc_pid_t = ::std::os::raw::c_int;
pub type __kernel_uid_t = ::std::os::raw::c_uint;
pub type __kernel_gid_t = ::std::os::raw::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = ::std::os::raw::c_int;
pub type __kernel_uid32_t = ::std::os::raw::c_uint;
pub type __kernel_gid32_t = ::std::os::raw::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct __kernel_fsid_t {
    pub val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___kernel_fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_fsid_t>())).val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_fsid_t),
            "::",
            stringify!(val)
        )
    );
}
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = ::std::os::raw::c_longlong;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::std::os::raw::c_int;
pub type __kernel_clockid_t = ::std::os::raw::c_int;
pub type __kernel_caddr_t = *mut ::std::os::raw::c_char;
pub type __kernel_uid16_t = ::std::os::raw::c_ushort;
pub type __kernel_gid16_t = ::std::os::raw::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __virtio16 = __u16;
pub type __virtio32 = __u32;
pub type __virtio64 = __u64;
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct virtio_blk_config {
    pub capacity: __u64,
    pub size_max: __u32,
    pub seg_max: __u32,
    pub geometry: virtio_blk_config_virtio_blk_geometry,
    pub blk_size: __u32,
    pub physical_block_exp: __u8,
    pub alignment_offset: __u8,
    pub min_io_size: __u16,
    pub opt_io_size: __u32,
    pub wce: __u8,
    pub unused: __u8,
    pub num_queues: __u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct virtio_blk_config_virtio_blk_geometry {
    pub cylinders: __u16,
    pub heads: __u8,
    pub sectors: __u8,
}
#[test]
fn bindgen_test_layout_virtio_blk_config_virtio_blk_geometry() {
    assert_eq!(
        ::std::mem::size_of::<virtio_blk_config_virtio_blk_geometry>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(virtio_blk_config_virtio_blk_geometry)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_blk_config_virtio_blk_geometry>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(virtio_blk_config_virtio_blk_geometry)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<virtio_blk_config_virtio_blk_geometry>())).cylinders as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config_virtio_blk_geometry),
            "::",
            stringify!(cylinders)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<virtio_blk_config_virtio_blk_geometry>())).heads as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config_virtio_blk_geometry),
            "::",
            stringify!(heads)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<virtio_blk_config_virtio_blk_geometry>())).sectors as *const _
                as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config_virtio_blk_geometry),
            "::",
            stringify!(sectors)
        )
    );
}
#[test]
fn bindgen_test_layout_virtio_blk_config() {
    assert_eq!(
        ::std::mem::size_of::<virtio_blk_config>(),
        36usize,
        concat!("Size of: ", stringify!(virtio_blk_config))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_blk_config>(),
        1usize,
        concat!("Alignment of ", stringify!(virtio_blk_config))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_blk_config>())).capacity as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config),
            "::",
            stringify!(capacity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_blk_config>())).size_max as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config),
            "::",
            stringify!(size_max)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_blk_config>())).seg_max as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config),
            "::",
            stringify!(seg_max)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_blk_config>())).geometry as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config),
            "::",
            stringify!(geometry)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_blk_config>())).blk_size as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config),
            "::",
            stringify!(blk_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<virtio_blk_config>())).physical_block_exp as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config),
            "::",
            stringify!(physical_block_exp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<virtio_blk_config>())).alignment_offset as *const _ as usize
        },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config),
            "::",
            stringify!(alignment_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_blk_config>())).min_io_size as *const _ as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config),
            "::",
            stringify!(min_io_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_blk_config>())).opt_io_size as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config),
            "::",
            stringify!(opt_io_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_blk_config>())).wce as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config),
            "::",
            stringify!(wce)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_blk_config>())).unused as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config),
            "::",
            stringify!(unused)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_blk_config>())).num_queues as *const _ as usize },
        34usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_config),
            "::",
            stringify!(num_queues)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct virtio_blk_outhdr {
    pub type_: __virtio32,
    pub ioprio: __virtio32,
    pub sector: __virtio64,
}
#[test]
fn bindgen_test_layout_virtio_blk_outhdr() {
    assert_eq!(
        ::std::mem::size_of::<virtio_blk_outhdr>(),
        16usize,
        concat!("Size of: ", stringify!(virtio_blk_outhdr))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_blk_outhdr>(),
        8usize,
        concat!("Alignment of ", stringify!(virtio_blk_outhdr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_blk_outhdr>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_outhdr),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_blk_outhdr>())).ioprio as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_outhdr),
            "::",
            stringify!(ioprio)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_blk_outhdr>())).sector as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_blk_outhdr),
            "::",
            stringify!(sector)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct virtio_scsi_inhdr {
    pub errors: __virtio32,
    pub data_len: __virtio32,
    pub sense_len: __virtio32,
    pub residual: __virtio32,
}
#[test]
fn bindgen_test_layout_virtio_scsi_inhdr() {
    assert_eq!(
        ::std::mem::size_of::<virtio_scsi_inhdr>(),
        16usize,
        concat!("Size of: ", stringify!(virtio_scsi_inhdr))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_scsi_inhdr>(),
        4usize,
        concat!("Alignment of ", stringify!(virtio_scsi_inhdr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_scsi_inhdr>())).errors as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_inhdr),
            "::",
            stringify!(errors)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_scsi_inhdr>())).data_len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_inhdr),
            "::",
            stringify!(data_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_scsi_inhdr>())).sense_len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_inhdr),
            "::",
            stringify!(sense_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<virtio_scsi_inhdr>())).residual as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_inhdr),
            "::",
            stringify!(residual)
        )
    );
}
