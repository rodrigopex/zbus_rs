//
// Created by rodrigopex on 29/12/22.
//

#ifndef ZBUS_RS_RS_LOG_H
#define ZBUS_RS_RS_LOG_H

enum rs_log_level {
	RS_ERR,
	RS_WRN,
	RS_INF,
	RS_DBG
};

struct rs_log_msg {
	const enum rs_log_level level;
	const char *msg;
	const uint8_t size;
};
#endif // ZBUS_RS_RS_LOG_H
