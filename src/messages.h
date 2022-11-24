/*
 * Copyright (c) 2022 Rodrigo Peixoto <rodrigopex@gmail.com>
 * SPDX-License-Identifier: Apache-2.0
 */
#ifndef _MESSAGES_H_
#define _MESSAGES_H_
#include <stdint.h>

struct version_msg {
	uint8_t major;
	uint8_t minor;
	uint16_t build;
};

struct acc_msg {
	int x;
	int y;
	int z;
};

struct ack_msg {
	uint64_t sequence;
};

#endif /* _MESSAGES_H_ */
