#ifndef	_NETINET_IN_H
#define	_NETINET_IN_H

#include <libc.h>
typedef unsigned short sa_family_t;
typedef uint16_t in_port_t;
typedef uint32_t in_addr_t;
struct in_addr { in_addr_t s_addr; };

struct sockaddr_in {
	sa_family_t sin_family;
	in_port_t sin_port;
	struct in_addr sin_addr;
	uint8_t sin_zero[8];
};

#endif