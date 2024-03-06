use crate::spec::cow;
use crate::spec::Target;

pub static TARGET: Target = {
    let mut base = super::i686_pc_windows_msvc::target();
    base.cpu = cow!("pentium");
    base.llvm_target = cow!("i586-pc-windows-msvc");
    base
};
