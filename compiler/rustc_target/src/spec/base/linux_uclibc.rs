use crate::spec::cow;
use crate::spec::{base, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions { env: cow!("uclibc"), ..base::linux::opts() }
}
