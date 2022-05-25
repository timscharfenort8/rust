// check-fail
// known-bug

// This should pass.

#![feature(cfg_accessible)]
#![feature(trait_alias)]

enum Enum {
    Existing { existing: u8 },
}

trait TraitAlias = std::fmt::Debug + Send;

// FIXME: Currently returns `false` but should be "not sure"
#[cfg_accessible(Enum::Existing::existing)]
const A: bool = true;

// FIXME: Currently returns `false` but should be "not sure"
#[cfg_accessible(Enum::Existing::unresolved)]
const B: bool = true;

// FIXME: Currently shows "cannot determine" but should be `false`
#[cfg_accessible(unresolved)]
const C: bool = true;

// FIXME: Currently shows "not sure" but should be `false`
#[cfg_accessible(TraitAlias::unresolved)]
const D: bool = true;

fn main() {}
