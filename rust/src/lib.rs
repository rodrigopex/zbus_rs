/*
 * Copyright (c) 2022 Rodrigo Peixoto <rodrigopex@gmail.com>
 * SPDX-License-Identifier: Apache-2.0
 */
#![no_std]
#![feature(alloc_error_handler)]
#![feature(c_variadic)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::format;
use core::time::Duration;
use zephyr::*;

//./target/riscv32imac-unknown-none-elf/release/build/rust-hello-d57ce81a5a6371c7/out/bindings.rs
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
mod zephyr;

zbus_static_channel_declare! {
    name: version_chan,
    msg_type: struct_version_msg
}
zbus_static_channel_declare! {
    name: acc_data_chan,
    msg_type: struct_acc_msg
}
zbus_static_channel_declare! {
    name: ack_chan,
    msg_type: struct_ack_msg
}
zbus_static_subscriber_declare! {
    name: rust_sub
}

#[no_mangle]
pub extern "C" fn rust_function(chan: *const struct_zbus_channel) {
    printk!(
        "Rust listener sequence: %llu\n\0",
        ack_chan
            .get_const_msg()
            .sequence
    );
}

#[no_mangle]
pub extern "C" fn rust_thread() {
    z_log_inf!("Rust thread started!");

    let mut acc = struct_acc_msg { x: 1, y: 2, z: 3 };

    match version_chan.read(Duration::from_secs(1)) {
        Ok(struct_version_msg {
               major,
               minor,
               build,
           }) => {
            let v = format!("Product firmware v{major}.{minor}.{build}\0");
            debug_assert_eq!(v, "Product firmware v0.1.2");
            z_log_inf!("{}", v);
        }
        Err(e) => z_log_err!("Could not read the channel. Error code {e}"),
    }
    let _ = acc_data_chan.claim(Duration::from_millis(1000), |claimed_channel| {
        let struct_version_msg {
            major,
            minor,
            build,
        } = claimed_channel.get_user_data();
        let v = format!("Accelerometer firmware v{major}.{minor}.{build}");
        debug_assert_eq!(v, "Accelerometer firmware v1.3.2089");
        z_log_inf!("{}", v);
        Ok(())
    });

    loop {
        match acc_data_chan.publish(&acc, Duration::from_secs(1)) {
            Ok(_) => z_log_inf!("Rust producer: Message sent!"),
            Err(e) => z_log_err!("Could not publish the message. Error code {e}"),
        }
        acc.x += 1;
        acc.y += 2;
        acc.z += 3;

        match rust_sub.wait(Duration::MAX) {
            Ok(changed_channel_ptr) => {
                debug_assert_eq!(
                    ack_chan, changed_channel_ptr,
                    "This subscriber must not receive a channel other then ack_channel"
                );
                match ack_chan.read(Duration::from_secs(1)) {
                    Ok(struct_ack_msg { sequence }) => {
                        z_log_wrn!("Rust subscriber sequence: {sequence}")
                    }
                    Err(e) => z_log_err!("Could not publish the message. Error code {e}"),
                };
            }
            Err(e) => z_log_err!("No notification arrived. Reason code {e}"),
        }
        sleep(Duration::from_secs(3));
    }
}
