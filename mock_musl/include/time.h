#ifndef _TIME_H
#define _TIME_H
#define NULL ((void*)0)

typedef long clock_t;
typedef long time_t;
typedef long suseconds_t;


#define CLOCKS_PER_SEC 1000000L

struct timespec { time_t tv_sec; long tv_nsec; };

struct tm {
	int tm_sec;
	int tm_min;
	int tm_hour;
	int tm_mday;
	int tm_mon;
	int tm_year;
	int tm_wday;
	int tm_yday;
	int tm_isdst;
	long __tm_gmtoff;
	const char *__tm_zone;
};

clock_t clock (void);
time_t time (time_t *);
double difftime (time_t, time_t);

#endif