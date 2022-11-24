/*
* Copyright (c) 2022 Rodrigo Peixoto <rodrigopex@gmail.com>
* SPDX-License-Identifier: Apache-2.0
*/
#![allow(dead_code)]
extern crate alloc;

use alloc::format;
use core::alloc::{GlobalAlloc, Layout};
use core::ffi::{c_char, c_void};
use core::fmt::Debug;
use core::panic::PanicInfo;
use core::time::Duration;

pub mod ffi {
    use super::*;

    extern "C" {
        pub fn k_free(ptr: *mut u8);

        pub fn k_malloc(size: usize) -> *mut u8;

        pub fn printk(fmt: *const c_char, ...);

        pub fn zephyr_rs_delay_ms(ms: u32);

        pub fn zephyr_rs_log(level: u8, text: *const c_char);

        pub fn zephyr_rs_timeout_from_ms(ms: u32) -> struct_k_timeout_t;

        pub fn zbus_chan_pub(
            chan: *const struct_zbus_channel,
            msg: *const c_void,
            timeout: struct_k_timeout_t,
        ) -> i32;

        pub fn zbus_chan_read(
            chan: *const struct_zbus_channel,
            msg: *mut c_void,
            timeout: struct_k_timeout_t,
        ) -> i32;

        pub fn zbus_chan_notify(
            chan: *const struct_zbus_channel,
            timeout: struct_k_timeout_t,
        ) -> i32;

        pub fn zbus_chan_claim(
            chan: *const struct_zbus_channel,
            timeout: struct_k_timeout_t,
        ) -> i32;

        pub fn zbus_chan_finish(chan: *const struct_zbus_channel) -> i32;

        pub fn zbus_chan_msg_const_wrapper(chan: *const struct_zbus_channel) -> *const c_void;

        pub fn zbus_chan_msg_wrapper(chan: *const struct_zbus_channel) -> *mut c_void;

        pub fn zbus_chan_user_data_wrapper(chan: *const struct_zbus_channel) -> *mut c_void;

        pub fn zbus_sub_wait(
            obs: *const struct_zbus_observer,
            chan: *mut struct_zbus_channel,
            timeout: struct_k_timeout_t,
        ) -> i32;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct struct_zbus_observer {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct struct_zbus_channel {
    _private: [u8; 0],
}

impl struct_zbus_channel {
    pub fn init() -> *const struct_zbus_channel {
        core::ptr::null()
    }

    pub fn init_mut() -> *mut struct_zbus_channel {
        core::ptr::null_mut()
    }
}

pub type k_ticks = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct struct_k_timeout_t {
    pub ticks: k_ticks,
}

pub mod zbus {
    use super::*;
    use core::marker::PhantomData;

    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Channel<MessageType> {
        c_reference: *const struct_zbus_channel,
        phantom: PhantomData<MessageType>,
    }

    pub trait CStructWrapper {
        type Output;
        fn get_c_reference(&self) -> Self::Output;
    }

    impl<MessageType> CStructWrapper for Channel<MessageType> {
        type Output = *const struct_zbus_channel;
        fn get_c_reference(&self) -> Self::Output {
            self.c_reference
        }
    }

    impl<MessageType> PartialEq<*const struct_zbus_channel> for Channel<MessageType>
    where
        MessageType: Debug + Default,
    {
        fn eq(&self, other: &*const struct_zbus_channel) -> bool {
            self.c_reference == *other
        }
    }

    impl<MessageType> Channel<MessageType>
    where
        MessageType: Default + Debug,
    {
        pub fn new(chan_ref: *const struct_zbus_channel) -> Self {
            Self {
                c_reference: chan_ref,
                phantom: PhantomData,
            }
        }

        pub fn publish(&self, msg: &MessageType, timeout: Duration) -> Result<(), i32> {
            match unsafe {
                ffi::zbus_chan_pub(
                    self.c_reference,
                    msg as *const _ as *const c_void,
                    ffi::zephyr_rs_timeout_from_ms(timeout.as_millis() as u32),
                )
            } {
                0 => Ok(()),
                e => Err(e),
            }
        }

        pub fn notify(&self, timeout: Duration) -> Result<(), i32> {
            match unsafe {
                ffi::zbus_chan_notify(
                    self.c_reference,
                    ffi::zephyr_rs_timeout_from_ms(timeout.as_millis() as u32),
                )
            } {
                0 => Ok(()),
                e => Err(e),
            }
        }

        pub fn read(&self, timeout: Duration) -> Result<MessageType, i32> {
            let mut msg = MessageType::default();

            match unsafe {
                ffi::zbus_chan_read(
                    self.c_reference,
                    &mut msg as *mut _ as *mut c_void,
                    ffi::zephyr_rs_timeout_from_ms(timeout.as_millis() as u32),
                )
            } {
                0 => Ok(msg),
                e => Err(e),
            }
        }

        pub fn claim<F>(&self, timeout: Duration, function: F) -> Result<(), i32>
        where
            F: FnOnce(ClaimedChannel<MessageType>) -> Result<(), i32>,
        {
            match unsafe {
                ffi::zbus_chan_claim(
                    self.c_reference,
                    ffi::zephyr_rs_timeout_from_ms(timeout.as_millis() as u32),
                )
            } {
                0 => function(ClaimedChannel::new(self)),
                e => Err(e),
            }
        }

        pub fn get_const_msg(&self) -> &MessageType {
            unsafe {
                core::mem::transmute::<*const c_void, &MessageType>(
                    ffi::zbus_chan_msg_const_wrapper(self.c_reference),
                )
            }
        }
    }

    pub struct ClaimedChannel<'a, MessageType> {
        channel: &'a Channel<MessageType>,
    }

    impl<'a, MessageType> Drop for ClaimedChannel<'a, MessageType> {
        fn drop(&mut self) {
            unsafe { ffi::zbus_chan_finish(self.channel.get_c_reference()) };
        }
    }

    impl<'a, MessageType> ClaimedChannel<'a, MessageType> {
        fn new(chan: &'a Channel<MessageType>) -> Self {
            ClaimedChannel { channel: chan }
        }

        pub fn finish(self) {
            /*! The finish is performed automatically by the Drop trait */
        }

        pub fn get_msg(&self) -> &mut MessageType {
            unsafe {
                core::mem::transmute::<*mut c_void, &mut MessageType>(ffi::zbus_chan_msg_wrapper(
                    self.channel.get_c_reference(),
                ))
            }
        }

        pub fn get_user_data<UserDataType>(&self) -> &mut UserDataType {
            unsafe {
                core::mem::transmute::<*mut c_void, &mut UserDataType>(
                    ffi::zbus_chan_user_data_wrapper(self.channel.get_c_reference()),
                )
            }
        }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Subscriber {
        c_reference: *const struct_zbus_observer,
    }

    impl CStructWrapper for Subscriber {
        type Output = *const struct_zbus_observer;

        fn get_c_reference(&self) -> Self::Output {
            self.c_reference
        }
    }

    impl Subscriber {
        pub fn new(sub_ref: *const struct_zbus_observer) -> Self {
            Self {
                c_reference: sub_ref,
            }
        }

        pub fn wait(&self, timeout: Duration) -> Result<*const struct_zbus_channel, i32> {
            let mut chan = struct_zbus_channel::init_mut();
            match unsafe {
                ffi::zbus_sub_wait(
                    self.c_reference,
                    &mut chan as *mut _ as *mut struct_zbus_channel,
                    ffi::zephyr_rs_timeout_from_ms(timeout.as_millis() as u32),
                )
            } {
                0 => Ok(chan as *const struct_zbus_channel),
                e => Err(e),
            }
        }
    }
}

struct ZephyrAllocator;

unsafe impl GlobalAlloc for ZephyrAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ffi::k_malloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ffi::k_free(ptr)
    }
}

#[global_allocator]
static GLOBAL: ZephyrAllocator = ZephyrAllocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        ffi::printk(
            "[RUST PANIC] %s\n\0".as_ptr() as *const c_char,
            format!("{}\0", info).as_ptr() as *const c_char,
        );
    }
    loop {}
}

#[alloc_error_handler]
fn err_handler(layout: core::alloc::Layout) -> ! {
    unsafe {
        ffi::printk(
            "[RUST ALLOC ERROR] Layout %d\n\0".as_ptr() as *const c_char,
            layout.size() as i32,
        );
    }
    loop {}
}

pub fn sleep(timeout: Duration) {
    unsafe {
        ffi::zephyr_rs_delay_ms(timeout.as_millis() as u32);
    }
}
pub enum LogLevel {
    Err = 1,
    Wrn = 2,
    Inf = 3,
    Dbg = 4,
}

pub fn log(level: LogLevel, text: &str) {
    if let Ok(c_string) = alloc::ffi::CString::new(text) {
        unsafe {
            ffi::zephyr_rs_log(level as u8, c_string.as_c_str().as_ptr());
        }
    }
}

#[macro_export]
macro_rules! z_log_err {
    ($($args:tt)*) => {{
        crate::zephyr::log(crate::zephyr::LogLevel::Err, format!($($args)*).as_str());
    }}
}

#[macro_export]
macro_rules! z_log_wrn {
    ($($args:tt)*) => {{
        crate::zephyr::log(crate::zephyr::LogLevel::Wrn, format!($($args)*).as_str());
    }}
}
#[macro_export]
macro_rules! z_log_inf {
    ($($args:tt)*) => {{
        crate::zephyr::log(crate::zephyr::LogLevel::Inf, format!($($args)*).as_str());
    }}
}
#[macro_export]
macro_rules! z_log_dbg {
    ($($args:tt)*) => {{
        crate::zephyr::log(crate::zephyr::LogLevel::Dbg, format!($($args)*).as_str());
    }}
}

#[macro_export]
macro_rules! zbus_c_channel_declare {
    ($($arg:ident),+) => {
extern "C" {
    $(
    static $arg: struct_zbus_channel;
    )*
    }
    }
}

#[macro_export]
macro_rules! zbus_c_subscriber_declare {
    ($($arg:ident),+) => {
extern "C" {
    $(
    static $arg: struct_zbus_observer;
    )*
}}
}

#[macro_export]
macro_rules! zbus_channel_create {
    (name=$chan:ident, msg_type=$msg:ident) => {
        zbus::Channel::<$msg>::new(unsafe { &$chan as *const struct_zbus_channel })
    };
    ($chan:ident, $msg:ident) => {
        zbus::Channel::<$msg>::new(unsafe { &$chan as *const struct_zbus_channel })
    };
}

#[macro_export]
macro_rules! zbus_subscriber_create {
    ($sub:ident) => {
        zbus::Subscriber::new(unsafe { &$sub as *const struct_zbus_observer })
    };
}
#[macro_export]
macro_rules! printk {
    ($fmt:literal) => {
        if let Ok(c_string) = alloc::ffi::CString::new($fmt) {
            unsafe {
                zephyr::ffi::printk(c_string.as_ptr());
            }
        }
    };
    ($fmt:literal,$($arg:expr),+) => {
        if let Ok(c_string) = alloc::ffi::CString::new($fmt) {
            unsafe {
                zephyr::ffi::printk(c_string.as_ptr(),$($arg),+);
            }
        }
    };
}
