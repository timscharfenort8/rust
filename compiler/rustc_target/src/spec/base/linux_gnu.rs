use crate::spec::cow;
use crate::spec::{base, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions { env: cow!("gnu"), ..base::linux::opts() }
}
