//
// Copyright RIME Developers
// Distributed under the BSD License
//
// 2011-04-24 GONG Chen <chen.sst@gmail.com>
//
#include <cstring>
#include <iostream>
#include <rime/candidate.h>
#include <rime/common.h>
#include <rime/composition.h>
#include <rime/context.h>
#include <rime/deployer.h>
#include <rime/engine.h>
#include <rime/key_event.h>
#include <rime/menu.h>
#include <rime/schema.h>
#include <rime/setup.h>
#include <rime/dict/dictionary.h>
#include <rime/dict/dict_compiler.h>
#include <rime/lever/deployment_tasks.h>
#include "codepage.h"

using namespace rime;

class RimeConsole {
 public:
  RimeConsole() : interactive_(false), engine_(Engine::Create()) {
    conn_ = engine_->sink().connect([this](const string& x) { OnCommit(x); });
  }
  ~RimeConsole() { conn_.disconnect(); }

  void OnCommit(const string& commit_text) {
    if (interactive_) {
      std::cout << "commit : [" << commit_text << "]" << std::endl;
    } else {
      std::cout << commit_text << std::endl;
    }
  }

  void PrintComposition(const Context* ctx) {
    if (!ctx || !ctx->IsComposing())
      return;
    std::cout << "input  : [" << ctx->input() << "]" << std::endl;
    const Composition& comp = ctx->composition();
    if (comp.empty())
      return;
    std::cout << "comp.  : [" << comp.GetDebugText() << "]" << std::endl;
    const Segment& current(comp.back());
    if (!current.menu)
      return;
    int page_size = engine_->active_engine()->schema()->page_size();
    int page_no = current.selected_index / page_size;
    the<Page> page(current.menu->CreatePage(page_size, page_no));
    if (!page)
      return;
    std::cout << "page_no: " << page_no << ", index: " << current.selected_index
              << std::endl;
    int i = 0;
    for (const an<Candidate>& cand : page->candidates) {
      std::cout << "cand. " << (++i % 10) << ": [";
      std::cout << cand->text();
      std::cout << "]";
      if (!cand->comment().empty())
        std::cout << "  " << cand->comment();
      std::cout << "  quality=" << cand->quality();
      std::cout << std::endl;
    }
  }

  void ProcessLine(const string& line) {
    KeySequence keys;
    if (!keys.Parse(line)) {
      LOG(ERROR) << "error parsing input: '" << line << "'";
      return;
    }
    for (const KeyEvent& key : keys) {
      engine_->ProcessKey(key);
    }
    Context* ctx = engine_->active_engine()->context();
    if (interactive_) {
      PrintComposition(ctx);
    } else {
      if (ctx && ctx->IsComposing()) {
        ctx->Commit();
      }
    }
  }

  void set_interactive(bool interactive) { interactive_ = interactive; }
  bool interactive() const { return interactive_; }

 private:
  bool interactive_;
  the<Engine> engine_;
  connection conn_;
};

// program entry
int main(int argc, char* argv[]) {
  unsigned int codepage = SetConsoleOutputCodePage();
  // initialize la Rime
  SetupLogging("rime.console");
  LoadModules(kDefaultModules);

  Deployer deployer;
  InstallationUpdate installation;
  if (!installation.Run(&deployer)) {
    std::cerr << "failed to initialize installation." << std::endl;
    SetConsoleOutputCodePage(codepage);
    return 1;
  }
  std::cerr << "initializing...";
  WorkspaceUpdate workspace_update;
  if (!workspace_update.Run(&deployer)) {
    std::cerr << "failure!" << std::endl;
    SetConsoleOutputCodePage(codepage);
    return 1;
  }
  std::cerr << "ready." << std::endl;

  RimeConsole console;
  // "-i" turns on interactive mode.
  //
  // In non-interactive mode (default), each input line is parsed as a key
  // sequence and then committed at end-of-line; committed text is printed to
  // stdout via OnCommit().
  //
  // In interactive mode (-i), rime_console does NOT auto-commit at end-of-line.
  // Instead it prints debug information about the current composition/menu,
  // which is useful to inspect the engine state when feeding key sequences.
  // Typical output looks like:
  //
  //   input  : [nihao]
  //   comp.  : [{abc}nihao=>你好]
  //   page_no: 0, index: 0
  //   cand. 1: [你好]  quality=...
  //   cand. 2: [妳好]  quality=...
  //
  // Where:
  // - input: raw input string in the Context.
  // - comp.: Composition::GetDebugText(); it includes the current segment tag
  //   (e.g. {abc}), the preedit spelling, and the preview text after "=>".
  // - page_no/index: current candidate page number and the selected_index of
  //   the current segment.
  // - cand.*: candidates on the current page, with their model "quality" score.
  bool interactive = argc > 1 && !strcmp(argv[1], "-i");
  console.set_interactive(interactive);

  // process input
  string line;
  while (std::getline(std::cin, line)) {
    console.ProcessLine(line);
  }
  SetConsoleOutputCodePage(codepage);
  return 0;
}
