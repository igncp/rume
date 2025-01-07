#ifndef RUME
#define RUME

// https://blog.asleson.org/2021/02/23/how-to-writing-a-c-shared-library-in-rust/

#include <stdint.h>

extern "C" {
  int get_some_cstr( char **s );
}

#endif
