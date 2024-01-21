#include "bindings.h"

// --------------------------------------------------
// Export To Rust
// --------------------------------------------------

RimeTraits *
rime_traits_init() {
  RimeTraits* traits = new RimeTraits;

  RIME_STRUCT_INIT(RimeTraits, *traits);

  return traits;
}

