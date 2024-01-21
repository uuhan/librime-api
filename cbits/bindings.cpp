#include "bindings.h"

// --------------------------------------------------
// Export To Rust
// --------------------------------------------------

RimeTraits
rime_traits_init() {
  RIME_STRUCT(RimeTraits, traits);

  return traits;
}

