#include "bindings.h"

// --------------------------------------------------
// Export To Rust
// --------------------------------------------------

RimeModule
rime_module_init() {
  RimeModule module = {0};
  RIME_STRUCT_INIT(RimeModule, module);

  return module;
}

extern void rime_require_module_lua();
void
rime_lua_init() {
  rime_require_module_lua();
}

