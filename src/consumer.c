/*
 * Copyright (c) 2022 Rodrigo Peixoto <rodrigopex@gmail.com>
 * SPDX-License-Identifier: Apache-2.0
 */
#include "messages.h"

#include <zephyr/logging/log.h>
#include <zephyr/zbus/zbus.h>
LOG_MODULE_DECLARE(zbus, CONFIG_ZBUS_LOG_LEVEL);

ZBUS_SUBSCRIBER_DEFINE(consumer_sub, 4);

ZBUS_CHAN_DEFINE(ack_chan,	 /* Name */
		 struct ack_msg, /* Message type */

		 NULL,						      /* Validator */
		 NULL,						      /* User data */
		 ZBUS_OBSERVERS(c_listener, rust_listener, rust_sub), /* observers */
		 ZBUS_MSG_INIT(.sequence = 0)			      /* Initial value */
);

static void consumer_subscriber_thread(void)
{
	const struct zbus_channel *chan;
	struct ack_msg ack = {.sequence = 0};
	while (!zbus_sub_wait(&consumer_sub, &chan, K_FOREVER)) {
		struct acc_msg acc;

		zbus_chan_read(chan, &acc, K_MSEC(500));
		LOG_INF(" --> Consuming data: Acc x=%d, y=%d, z=%d", acc.x, acc.y, acc.z);

		++ack.sequence;
		zbus_chan_pub(&ack_chan, &ack, K_MSEC(250));
	}
}

K_THREAD_DEFINE(consumer_thread_id, 512, consumer_subscriber_thread, NULL, NULL, NULL, 3, 0, 0);
