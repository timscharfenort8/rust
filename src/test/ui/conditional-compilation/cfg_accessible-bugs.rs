// check-fail
// known-bug

// This should pass.

#![feature(cfg_accessible)]
#![feature(trait_alias)]

trait TraitAlias = std::fmt::Debug + Send;

// FIXME: Currently shows "cannot determine" but should be `false`
#[cfg_accessible(unresolved)]
const C: bool = true;

// FIXME: Currently shows "not sure" but should be `false`
#[cfg_accessible(TraitAlias::unresolved)]
const D: bool = true;

fn main() {}
