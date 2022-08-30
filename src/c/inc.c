#include "inc.h"
#include <math.h>
#include <stdbool.h>
#include <stdlib.h>

bool is_prime(unsigned long long number) {
  unsigned long long sqrt_number = sqrt(number);

  for (unsigned long long i = 2; i < sqrt_number; i++) {
    if (number % i == 0) {
      return false;
    }
  }
  return true;
};

unsigned long long *get_all_factors(unsigned long long number, int *len) {
  unsigned long long *factors;
  factors = (unsigned long long *)malloc(sizeof(unsigned long long));
  factors[0] = 1;
  int len_factors = 1;
  unsigned long long possible_divisor = 2;

  while (possible_divisor <= number) {
    if (possible_divisor == number || possible_divisor * 2 > number) {
      len_factors++;
      factors = (unsigned long long *)realloc(
          factors, len_factors * sizeof(unsigned long long));
      factors[len_factors - 1] = number;
      break;
    } else if (number % possible_divisor == 0 && is_prime(possible_divisor)) {
      len_factors++;
      factors = (unsigned long long *)realloc(
          factors, len_factors * sizeof(unsigned long long));
      factors[len_factors - 1] = possible_divisor;
      number = number / possible_divisor;
      possible_divisor = 2;
    } else {
      possible_divisor++;
    }
  }
  *len = len_factors;
  return factors;
};
