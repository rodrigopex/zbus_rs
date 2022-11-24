/*
* Copyright (c) 2022 Rodrigo Peixoto <rodrigopex@gmail.com>
* SPDX-License-Identifier: Apache-2.0
 */
#include <zephyr/kernel.h>
#include <zephyr/zbus/zbus.h>
#include <zephyr/logging/log.h>
#define LOG_LEVEL LOG_LEVEL_DBG
LOG_MODULE_REGISTER(zephyr_rs);

const void *zbus_chan_msg_const_wrapper(const struct zbus_channel *chan) {
	return zbus_chan_const_msg(chan);
}
void *zbus_chan_msg_wrapper(const struct zbus_channel *chan) {
	return zbus_chan_msg(chan);
}

void *zbus_chan_user_data_wrapper(const struct zbus_channel *chan) {
	return zbus_chan_user_data(chan);
}

k_timeout_t zephyr_rs_timeout_from_ms(uint32_t ms) {
	return Z_TIMEOUT_MS(ms);
}

int32_t zephyr_rs_delay_ms(uint32_t ms) {
    return k_sleep(Z_TIMEOUT_MS(ms));
}

void zephyr_rs_log(uint8_t level, const char *text) {
    /* Levels are based on `log::Level`. */
    switch (level) {
        case 1:
            LOG_ERR("%s", text);
            break;
        case 2:
            LOG_WRN("%s", text);
            break;
        case 3:
            LOG_INF("%s", text);
            break;
        case 4:
            /* Others, or trace logs as debug. */
        default:
            LOG_DBG("%s", text);
            break;
            break;
    }
}
