#include <gtest/gtest.h>
#include <string>
#include "rume_extension.h"

using namespace std;

TEST(RimeStringsTest, SplitsWithDelimiter) {
  auto s = "Hello, world!";
  auto delim = ",";
  auto split_strings_ptr = rume_extension_strings_split(s, delim, 0);

  if (split_strings_ptr == NULL) {
    cout << "split_strings_ptr is NULL" << endl;
    ASSERT_TRUE(false);
  }

  vector<string> split_strings;
  while (*split_strings_ptr) {
    split_strings.push_back(*split_strings_ptr);
    ++split_strings_ptr;
  }

  ASSERT_EQ(split_strings.size(), 2);
  ASSERT_EQ(split_strings[0], "Hello");
  ASSERT_EQ(split_strings[1], " world!");
}
