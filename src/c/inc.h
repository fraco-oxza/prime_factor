#ifndef _INC_H_
#define _INC_H_
#include <stdbool.h>

bool is_prime(unsigned long long number);

unsigned long long *get_all_factors(unsigned long long number, int *len);

#endif
