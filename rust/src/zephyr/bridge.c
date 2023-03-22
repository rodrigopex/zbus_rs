/*
 * Copyright (c) 2022 Rodrigo Peixoto <rodrigopex@gmail.com>
 * SPDX-License-Identifier: Apache-2.0
 */
#include <zephyr/kernel.h>
#include <zephyr/zbus/zbus.h>
#include <zephyr/logging/log.h>
#include "rs_log.h"
#define LOG_LEVEL LOG_LEVEL_DBG
LOG_MODULE_REGISTER(zephyr_rs);

const void *zbus_chan_msg_const_wrapper(const struct zbus_channel *chan)
{
	return zbus_chan_const_msg(chan);
}
void *zbus_chan_msg_wrapper(const struct zbus_channel *chan)
{
	return zbus_chan_msg(chan);
}

void *zbus_chan_user_data_wrapper(const struct zbus_channel *chan)
{
	return zbus_chan_user_data(chan);
}

k_timeout_t zephyr_rs_timeout_from_ms(uint32_t ms)
{
	return Z_TIMEOUT_MS(ms);
}

int32_t zephyr_rs_delay_ms(uint32_t ms)
{
	return k_sleep(Z_TIMEOUT_MS(ms));
}

void log_listener_callback(const struct zbus_channel *chan)
{
	char msg[256] = {0};
	const struct rs_log_msg *l = zbus_chan_const_msg(chan);
	memcpy(msg,l->msg, l->size);
	switch (l->level) {
	case RS_ERR:
		LOG_ERR("%s", msg);
		break;
	case RS_WRN:
		LOG_WRN("%s", msg);
		break;
	case RS_INF:
		LOG_INF("%s", msg);
		break;
	case RS_DBG:
		LOG_DBG("%s", msg);
		break;
	}
}

ZBUS_LISTENER_DEFINE(log_listener, log_listener_callback);

ZBUS_CHAN_DEFINE(log_chan,	    /* Name */
		 struct rs_log_msg, /* Message type */

		 NULL,			       /* Validator */
		 NULL,			       /* User data */
		 ZBUS_OBSERVERS(log_listener), /* observers */
		 ZBUS_MSG_INIT(0)	       /* Initial value major 0, minor 1, build 2 */
);
