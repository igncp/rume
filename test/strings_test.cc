#include <gtest/gtest.h>
#include <string>
#include "rume.h"

using namespace std;

class RimeStringsTest : public ::testing::Test {
 protected:
  RimeStringsTest() = default;

  virtual void SetUp() {}

  virtual void TearDown() {}
};

TEST(RimeStringsTest, Dummy) {
  auto s = "Hello, world!";
  auto delim = ",";
  auto split_strings_ptr = rume_strings_split(s, delim, NULL);
  vector<string> split_strings;
  int i = 0;
  if (split_strings_ptr == NULL) {
    cout << "split_strings_ptr is NULL" << endl;
    ASSERT_TRUE(false);
  }
  auto ptr_size = sizeof(split_strings_ptr) / sizeof(split_strings_ptr[0]);
  cout << "ptr_size: " << ptr_size << endl;

  ASSERT_TRUE(false);
}
