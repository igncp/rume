#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>





struct Foo {
  enum class Tag {
    A,
  };

  struct A_Body {
    float _0[2];
  };

  Tag tag;
  union {
    A_Body a;
  };
};

extern "C" {

extern const int STRING_SPLIT_BEHAVIOR_KEEP_TOKEN;

extern const int STRING_SPLIT_BEHAVIOR_SKIP_TOKEN;

/// # Safety
/// This function is unsafe because it dereferences the `desc` pointer.
int32_t rume_get_init_str(char **desc);

/// # Safety
/// This function is unsafe because it dereferences the `test_param` pointer.
char *rume_use_foo(Foo test_param);

/// # Safety
/// This function is unsafe because it dereferences the `str_ptr` and `delim_str` pointers.
char **rume_strings_split(const char *str_ptr, const char *delim_str, const int *behavior_ptr);

}  // extern "C"
