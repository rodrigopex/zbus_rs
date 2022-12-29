/*
 * Copyright (c) 2022 Rodrigo Peixoto <rodrigopex@gmail.com>
 * SPDX-License-Identifier: Apache-2.0
 */
#include <zephyr/kernel.h>
#include <zephyr/zbus/zbus.h>
#include <zephyr/logging/log.h>
#define LOG_LEVEL LOG_LEVEL_DBG
LOG_MODULE_REGISTER(rust_bridge);

#include "messages.h"

ZBUS_CHAN_DEFINE(version_chan,	     /* Name */
		 struct version_msg, /* Message type */

		 NULL,		       /* Validator */
		 NULL,		       /* User data */
		 ZBUS_OBSERVERS_EMPTY, /* observers */
		 ZBUS_MSG_INIT(.major = 0, .minor = 1,
			       .build = 2) /* Initial value major 0, minor 1, build 2 */
);

static struct version_msg acc_fw_version = {1, 3, 2089};

ZBUS_CHAN_DEFINE(acc_data_chan,	 /* Name */
		 struct acc_msg, /* Message type */

		 NULL,				       /* Validator */
		 &acc_fw_version,		       /* User data */
		 ZBUS_OBSERVERS(consumer_sub),	       /* observers */
		 ZBUS_MSG_INIT(.x = 0, .y = 0, .z = 0) /* Initial value */
);

/* External declaration for the Rust callback. */
void rust_function(const struct zbus_channel *chan);

/* External declaration for the Rust thread. */
void rust_thread(void);

void c_listener_callback(const struct zbus_channel *chan)
{

	const struct ack_msg *msg = zbus_chan_const_msg(chan);
	LOG_DBG("C listener sequence: %u", msg->sequence);
}


ZBUS_SUBSCRIBER_DEFINE(rust_sub, 4);

ZBUS_LISTENER_DEFINE(c_listener, c_listener_callback);
ZBUS_LISTENER_DEFINE(rust_listener, rust_function);

K_THREAD_DEFINE(rust_thread_id, 1024, rust_thread, NULL, NULL, NULL, 3, 0, 0);
