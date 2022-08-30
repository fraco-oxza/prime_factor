cmake_minimum_required(VERSION 3.23)
project(prime_factor C)

set(CMAKE_C_STANDARD 11)

include_directories(src/c)

add_executable(prime_factor
        src/c/inc.c
        src/c/inc.h)
