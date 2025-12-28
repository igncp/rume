//
// Copyright RIME Developers
// Distributed under the BSD License
//
// 2011-11-27 GONG Chen <chen.sst@gmail.com>
//
#ifndef RIME_DICT_COMPILER_H_
#define RIME_DICT_COMPILER_H_

#include <rime_api.h>
#include <rime/common.h>

// Simple overview:
// DictCompiler takes human-readable schema/dictionary sources and produces
// fast, load-ready binary artifacts for runtime.
//
// - Prism: a compact spelling index (double-array trie) that maps
//   pronunciations/syllables to internal IDs and helps segment input. Saved
//   as a `.prism.bin` file.
// - Table: the forward dictionary that maps codes/sequences to entries
//   (characters/phrases with weights). Saved as a `.table.bin` file.
// - ReverseDb (optional): reverse lookup data (characters → pronunciation),
//   saved as `.reverse.bin`.
//
// DictCompiler coordinates building these structures and calls their Save()
// methods to write fixed-layout binary images. Rebuilds are decided via
// checksums so startup can just memory-map these `.bin` files without
// reprocessing the sources.

namespace rime {

class Dictionary;
class Prism;
class Table;
class ReverseDb;
class DictSettings;
class EditDistanceCorrector;
class EntryCollector;
class Vocabulary;
class ResourceResolver;

class DictCompiler {
 public:
  enum Options {
    kRebuildPrism = 1,
    kRebuildTable = 2,
    kRebuild = kRebuildPrism | kRebuildTable,
    kDump = 4,
  };

  RIME_DLL explicit DictCompiler(Dictionary* dictionary);
  RIME_DLL virtual ~DictCompiler();

  RIME_DLL bool Compile(const path& schema_file);
  void set_options(int options) { options_ = options; }

 private:
  bool BuildTable(int table_index,
                  EntryCollector& collector,
                  DictSettings* settings,
                  const vector<path>& dict_files,
                  uint32_t dict_file_checksum);
  bool BuildPrism(const path& schema_file,
                  uint32_t dict_file_checksum,
                  uint32_t schema_file_checksum);
  bool BuildReverseDb(DictSettings* settings,
                      const EntryCollector& collector,
                      const Vocabulary& vocabulary,
                      uint32_t dict_file_checksum);

  const string& dict_name_;
  const vector<string>& packs_;
  an<Prism> prism_;
  an<EditDistanceCorrector> correction_;
  vector<of<Table>> tables_;
  int options_ = 0;
  the<ResourceResolver> source_resolver_;
  the<ResourceResolver> target_resolver_;
};

}  // namespace rime

#endif  // RIME_DICT_COMPILER_H_
