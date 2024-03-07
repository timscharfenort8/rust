use rustc_span::symbol::sym;
use rustc_span::symbol::Symbol;

/// Features that control behaviour of rustc, rather than the codegen.
pub const RUSTC_SPECIFIC_FEATURES: &[Symbol] = &[sym::crt_dash_static];

/// Stability information for target features.
#[derive(Debug, Clone, Copy)]
pub enum Stability {
    /// This target feature is stable, it can be used in `#[target_feature]` and
    /// `#[cfg(target_feature)]`.
    Stable,
    /// This target feature is unstable; using it in `#[target_feature]` or `#[cfg(target_feature)]`
    /// requires enabling the given nightly feature.
    Unstable(Symbol),
}
use Stability::*;

impl Stability {
    pub fn as_feature_name(self) -> Option<Symbol> {
        match self {
            Stable => None,
            Unstable(s) => Some(s),
        }
    }

    pub fn is_stable(self) -> bool {
        matches!(self, Stable)
    }
}

pub type TargetFeature = (Symbol, Stability);

// Here we list target features that rustc "understands": they can be used in `#[target_feature]`
// and `#[cfg(target_feature)]`. They also do not trigger any warnings when used with
// `-Ctarget-feature`.
//
// When adding features to the below lists
// check whether they're named already elsewhere in rust
// e.g. in stdarch and whether the given name matches LLVM's
// if it doesn't, to_llvm_feature in llvm_util in rustc_codegen_llvm needs to be adapted.
//
// Also note that all target features listed here must be purely additive: for target_feature 1.1 to
// be sound, we can never allow features like `+soft-float` (on x86) to be controlled on a
// per-function level, since we would then allow safe calls from functions with `+soft-float` to
// functions without that feature!
//
// When adding a new feature, be particularly mindful of features that affect function ABIs. Those
// need to be treated very carefully to avoid introducing unsoundness! This often affects features
// that enable/disable hardfloat support (see https://github.com/rust-lang/rust/issues/116344 for an
// example of this going wrong), but features enabling new SIMD registers are also a concern (see
// https://github.com/rust-lang/rust/issues/116558 for an example of this going wrong).
//
// Stabilizing a target feature requires t-lang approval.

const ARM_ALLOWED_FEATURES: &[TargetFeature] = &[
    // tidy-alphabetical-start
    (sym::aclass, Unstable(sym::arm_target_feature)),
    (sym::aes, Unstable(sym::arm_target_feature)),
    (sym::crc, Unstable(sym::arm_target_feature)),
    (sym::d32, Unstable(sym::arm_target_feature)),
    (sym::dotprod, Unstable(sym::arm_target_feature)),
    (sym::dsp, Unstable(sym::arm_target_feature)),
    (sym::fp_armv8, Unstable(sym::arm_target_feature)),
    (sym::i8mm, Unstable(sym::arm_target_feature)),
    (sym::mclass, Unstable(sym::arm_target_feature)),
    (sym::neon, Unstable(sym::arm_target_feature)),
    (sym::rclass, Unstable(sym::arm_target_feature)),
    (sym::sha2, Unstable(sym::arm_target_feature)),
    // This is needed for inline assembly, but shouldn't be stabilized as_is
    // since it should be enabled per_function using #[instruction_set], not
    // #[target_feature].
    (sym::thumb_mode, Unstable(sym::arm_target_feature)),
    (sym::thumb2, Unstable(sym::arm_target_feature)),
    (sym::trustzone, Unstable(sym::arm_target_feature)),
    (sym::v5te, Unstable(sym::arm_target_feature)),
    (sym::v6, Unstable(sym::arm_target_feature)),
    (sym::v6k, Unstable(sym::arm_target_feature)),
    (sym::v6t2, Unstable(sym::arm_target_feature)),
    (sym::v7, Unstable(sym::arm_target_feature)),
    (sym::v8, Unstable(sym::arm_target_feature)),
    (sym::vfp2, Unstable(sym::arm_target_feature)),
    (sym::vfp3, Unstable(sym::arm_target_feature)),
    (sym::vfp4, Unstable(sym::arm_target_feature)),
    (sym::virtualization, Unstable(sym::arm_target_feature)),
    // tidy-alphabetical-end
];

const AARCH64_ALLOWED_FEATURES: &[TargetFeature] = &[
    // tidy-alphabetical-start
    // FEAT_AES & FEAT_PMULL
    (sym::aes, Stable),
    // FEAT_BF16
    (sym::bf16, Stable),
    // FEAT_BTI
    (sym::bti, Stable),
    // FEAT_CRC
    (sym::crc, Stable),
    // FEAT_DIT
    (sym::dit, Stable),
    // FEAT_DotProd
    (sym::dotprod, Stable),
    // FEAT_DPB
    (sym::dpb, Stable),
    // FEAT_DPB2
    (sym::dpb2, Stable),
    // FEAT_F32MM
    (sym::f32mm, Stable),
    // FEAT_F64MM
    (sym::f64mm, Stable),
    // FEAT_FCMA
    (sym::fcma, Stable),
    // FEAT_FHM
    (sym::fhm, Stable),
    // FEAT_FLAGM
    (sym::flagm, Stable),
    // FEAT_FP16
    (sym::fp16, Stable),
    // FEAT_FRINTTS
    (sym::frintts, Stable),
    // FEAT_I8MM
    (sym::i8mm, Stable),
    // FEAT_JSCVT
    (sym::jsconv, Stable),
    // FEAT_LOR
    (sym::lor, Stable),
    // FEAT_LSE
    (sym::lse, Stable),
    // FEAT_MTE & FEAT_MTE2
    (sym::mte, Stable),
    // FEAT_AdvSimd & FEAT_FP
    (sym::neon, Stable),
    // FEAT_PAUTH (address authentication)
    (sym::paca, Stable),
    // FEAT_PAUTH (generic authentication)
    (sym::pacg, Stable),
    // FEAT_PAN
    (sym::pan, Stable),
    // FEAT_PMUv3
    (sym::pmuv3, Stable),
    // FEAT_RAND
    (sym::rand, Stable),
    // FEAT_RAS & FEAT_RASv1p1
    (sym::ras, Stable),
    // FEAT_RCPC
    (sym::rcpc, Stable),
    // FEAT_RCPC2
    (sym::rcpc2, Stable),
    // FEAT_RDM
    (sym::rdm, Stable),
    // FEAT_SB
    (sym::sb, Stable),
    // FEAT_SHA1 & FEAT_SHA256
    (sym::sha2, Stable),
    // FEAT_SHA512 & FEAT_SHA3
    (sym::sha3, Stable),
    // FEAT_SM3 & FEAT_SM4
    (sym::sm4, Stable),
    // FEAT_SPE
    (sym::spe, Stable),
    // FEAT_SSBS & FEAT_SSBS2
    (sym::ssbs, Stable),
    // FEAT_SVE
    (sym::sve, Stable),
    // FEAT_SVE2
    (sym::sve2, Stable),
    // FEAT_SVE2_AES
    (sym::sve2_aes, Stable),
    // FEAT_SVE2_BitPerm
    (sym::sve2_bitperm, Stable),
    // FEAT_SVE2_SHA3
    (sym::sve2_sha3, Stable),
    // FEAT_SVE2_SM4
    (sym::sve2_sm4, Stable),
    // FEAT_TME
    (sym::tme, Stable),
    (sym::v8_1a, Unstable(sym::aarch64_ver_target_feature)),
    (sym::v8_2a, Unstable(sym::aarch64_ver_target_feature)),
    (sym::v8_3a, Unstable(sym::aarch64_ver_target_feature)),
    (sym::v8_4a, Unstable(sym::aarch64_ver_target_feature)),
    (sym::v8_5a, Unstable(sym::aarch64_ver_target_feature)),
    (sym::v8_6a, Unstable(sym::aarch64_ver_target_feature)),
    (sym::v8_7a, Unstable(sym::aarch64_ver_target_feature)),
    // FEAT_VHE
    (sym::vh, Stable),
    // tidy-alphabetical-end
];

const AARCH64_TIED_FEATURES: &[&[Symbol]] = &[
    &[sym::paca, sym::pacg], // Together these represent `pauth` in LLVM
];

const X86_ALLOWED_FEATURES: &[TargetFeature] = &[
    // tidy-alphabetical-start
    (sym::adx, Stable),
    (sym::aes, Stable),
    (sym::avx, Stable),
    (sym::avx2, Stable),
    (sym::avx512bf16, Unstable(sym::avx512_target_feature)),
    (sym::avx512bitalg, Unstable(sym::avx512_target_feature)),
    (sym::avx512bw, Unstable(sym::avx512_target_feature)),
    (sym::avx512cd, Unstable(sym::avx512_target_feature)),
    (sym::avx512dq, Unstable(sym::avx512_target_feature)),
    (sym::avx512er, Unstable(sym::avx512_target_feature)),
    (sym::avx512f, Unstable(sym::avx512_target_feature)),
    (sym::avx512fp16, Unstable(sym::avx512_target_feature)),
    (sym::avx512ifma, Unstable(sym::avx512_target_feature)),
    (sym::avx512pf, Unstable(sym::avx512_target_feature)),
    (sym::avx512vbmi, Unstable(sym::avx512_target_feature)),
    (sym::avx512vbmi2, Unstable(sym::avx512_target_feature)),
    (sym::avx512vl, Unstable(sym::avx512_target_feature)),
    (sym::avx512vnni, Unstable(sym::avx512_target_feature)),
    (sym::avx512vp2intersect, Unstable(sym::avx512_target_feature)),
    (sym::avx512vpopcntdq, Unstable(sym::avx512_target_feature)),
    (sym::bmi1, Stable),
    (sym::bmi2, Stable),
    (sym::cmpxchg16b, Stable),
    (sym::ermsb, Unstable(sym::ermsb_target_feature)),
    (sym::f16c, Stable),
    (sym::fma, Stable),
    (sym::fxsr, Stable),
    (sym::gfni, Unstable(sym::avx512_target_feature)),
    (sym::lahfsahf, Unstable(sym::lahfsahf_target_feature)),
    (sym::lzcnt, Stable),
    (sym::movbe, Stable),
    (sym::pclmulqdq, Stable),
    (sym::popcnt, Stable),
    (sym::prfchw, Unstable(sym::prfchw_target_feature)),
    (sym::rdrand, Stable),
    (sym::rdseed, Stable),
    (sym::rtm, Unstable(sym::rtm_target_feature)),
    (sym::sha, Stable),
    (sym::sse, Stable),
    (sym::sse2, Stable),
    (sym::sse3, Stable),
    (sym::sse4_1, Stable),
    (sym::sse4_2, Stable),
    (sym::sse4a, Unstable(sym::sse4a_target_feature)),
    (sym::ssse3, Stable),
    (sym::tbm, Unstable(sym::tbm_target_feature)),
    (sym::vaes, Unstable(sym::avx512_target_feature)),
    (sym::vpclmulqdq, Unstable(sym::avx512_target_feature)),
    (sym::xsave, Stable),
    (sym::xsavec, Stable),
    (sym::xsaveopt, Stable),
    (sym::xsaves, Stable),
    // tidy-alphabetical-end
];

const HEXAGON_ALLOWED_FEATURES: &[TargetFeature] = &[
    // tidy_alphabetical-start
    (sym::hvx, Unstable(sym::hexagon_target_feature)),
    (sym::hvx_length128b, Unstable(sym::hexagon_target_feature)),
    // tidy_alphabetical-end
];

const POWERPC_ALLOWED_FEATURES: &[TargetFeature] = &[
    // tidy-alphabetical-start
    (sym::altivec, Unstable(sym::powerpc_target_feature)),
    (sym::power10_vector, Unstable(sym::powerpc_target_feature)),
    (sym::power8_altivec, Unstable(sym::powerpc_target_feature)),
    (sym::power8_vector, Unstable(sym::powerpc_target_feature)),
    (sym::power9_altivec, Unstable(sym::powerpc_target_feature)),
    (sym::power9_vector, Unstable(sym::powerpc_target_feature)),
    (sym::vsx, Unstable(sym::powerpc_target_feature)),
    // tidy-alphabetical-end
];

const MIPS_ALLOWED_FEATURES: &[TargetFeature] = &[
    // tidy-alphabetical-start
    (sym::fp64, Unstable(sym::mips_target_feature)),
    (sym::msa, Unstable(sym::mips_target_feature)),
    (sym::virt, Unstable(sym::mips_target_feature)),
    // tidy-alphabetical-end
];

const RISCV_ALLOWED_FEATURES: &[TargetFeature] = &[
    // tidy-alphabetical-start
    (sym::a, Stable),
    (sym::c, Stable),
    (sym::d, Unstable(sym::riscv_target_feature)),
    (sym::e, Unstable(sym::riscv_target_feature)),
    (sym::f, Unstable(sym::riscv_target_feature)),
    (sym::fast_unaligned_access, Unstable(sym::riscv_target_feature)),
    (sym::m, Stable),
    (sym::relax, Unstable(sym::riscv_target_feature)),
    (sym::v, Unstable(sym::riscv_target_feature)),
    (sym::zba, Stable),
    (sym::zbb, Stable),
    (sym::zbc, Stable),
    (sym::zbkb, Stable),
    (sym::zbkc, Stable),
    (sym::zbkx, Stable),
    (sym::zbs, Stable),
    (sym::zdinx, Unstable(sym::riscv_target_feature)),
    (sym::zfh, Unstable(sym::riscv_target_feature)),
    (sym::zfhmin, Unstable(sym::riscv_target_feature)),
    (sym::zfinx, Unstable(sym::riscv_target_feature)),
    (sym::zhinx, Unstable(sym::riscv_target_feature)),
    (sym::zhinxmin, Unstable(sym::riscv_target_feature)),
    (sym::zk, Stable),
    (sym::zkn, Stable),
    (sym::zknd, Stable),
    (sym::zkne, Stable),
    (sym::zknh, Stable),
    (sym::zkr, Stable),
    (sym::zks, Stable),
    (sym::zksed, Stable),
    (sym::zksh, Stable),
    (sym::zkt, Stable),
    // tidy-alphabetical-end
];

const WASM_ALLOWED_FEATURES: &[TargetFeature] = &[
    // tidy_alphabetical-start
    (sym::atomics, Unstable(sym::wasm_target_feature)),
    (sym::bulk_memory, Unstable(sym::wasm_target_feature)),
    (sym::exception_handling, Unstable(sym::wasm_target_feature)),
    (sym::multivalue, Unstable(sym::wasm_target_feature)),
    (sym::mutable_globals, Unstable(sym::wasm_target_feature)),
    (sym::nontrapping_fptoint, Unstable(sym::wasm_target_feature)),
    (sym::reference_types, Unstable(sym::wasm_target_feature)),
    (sym::relaxed_simd, Unstable(sym::wasm_target_feature)),
    (sym::sign_ext, Unstable(sym::wasm_target_feature)),
    (sym::simd128, Stable),
    // tidy-alphabetical-end
];

const BPF_ALLOWED_FEATURES: &[TargetFeature] = &[(sym::alu32, Unstable(sym::bpf_target_feature))];

const CSKY_ALLOWED_FEATURES: &[TargetFeature] = &[
    // tidy-alphabetical-start
    (sym::_10e60, Unstable(sym::csky_target_feature)),
    (sym::_2e3, Unstable(sym::csky_target_feature)),
    (sym::_3e3r1, Unstable(sym::csky_target_feature)),
    (sym::_3e3r2, Unstable(sym::csky_target_feature)),
    (sym::_3e3r3, Unstable(sym::csky_target_feature)),
    (sym::_3e7, Unstable(sym::csky_target_feature)),
    (sym::_7e10, Unstable(sym::csky_target_feature)),
    (sym::cache, Unstable(sym::csky_target_feature)),
    (sym::doloop, Unstable(sym::csky_target_feature)),
    (sym::dsp1e2, Unstable(sym::csky_target_feature)),
    (sym::dspe60, Unstable(sym::csky_target_feature)),
    (sym::e1, Unstable(sym::csky_target_feature)),
    (sym::e2, Unstable(sym::csky_target_feature)),
    (sym::edsp, Unstable(sym::csky_target_feature)),
    (sym::elrw, Unstable(sym::csky_target_feature)),
    (sym::float1e2, Unstable(sym::csky_target_feature)),
    (sym::float1e3, Unstable(sym::csky_target_feature)),
    (sym::float3e4, Unstable(sym::csky_target_feature)),
    (sym::float7e60, Unstable(sym::csky_target_feature)),
    (sym::floate1, Unstable(sym::csky_target_feature)),
    (sym::hard_tp, Unstable(sym::csky_target_feature)),
    (sym::high_registers, Unstable(sym::csky_target_feature)),
    (sym::hwdiv, Unstable(sym::csky_target_feature)),
    (sym::mp, Unstable(sym::csky_target_feature)),
    (sym::mp1e2, Unstable(sym::csky_target_feature)),
    (sym::nvic, Unstable(sym::csky_target_feature)),
    (sym::trust, Unstable(sym::csky_target_feature)),
    (sym::vdsp2e60f, Unstable(sym::csky_target_feature)),
    (sym::vdspv1, Unstable(sym::csky_target_feature)),
    (sym::vdspv2, Unstable(sym::csky_target_feature)),
    // tidy-alphabetical-end
    //fpu
    // tidy-alphabetical-start
    (sym::fdivdu, Unstable(sym::csky_target_feature)),
    (sym::fpuv2_df, Unstable(sym::csky_target_feature)),
    (sym::fpuv2_sf, Unstable(sym::csky_target_feature)),
    (sym::fpuv3_df, Unstable(sym::csky_target_feature)),
    (sym::fpuv3_hf, Unstable(sym::csky_target_feature)),
    (sym::fpuv3_hi, Unstable(sym::csky_target_feature)),
    (sym::fpuv3_sf, Unstable(sym::csky_target_feature)),
    (sym::hard_float, Unstable(sym::csky_target_feature)),
    (sym::hard_float_abi, Unstable(sym::csky_target_feature)),
    // tidy-alphabetical-end
];

const LOONGARCH_ALLOWED_FEATURES: &[TargetFeature] = &[
    // tidy-alphabetical-start
    (sym::d, Unstable(sym::loongarch_target_feature)),
    (sym::f, Unstable(sym::loongarch_target_feature)),
    (sym::frecipe, Unstable(sym::loongarch_target_feature)),
    (sym::lasx, Unstable(sym::loongarch_target_feature)),
    (sym::lbt, Unstable(sym::loongarch_target_feature)),
    (sym::lsx, Unstable(sym::loongarch_target_feature)),
    (sym::lvz, Unstable(sym::loongarch_target_feature)),
    (sym::relax, Unstable(sym::loongarch_target_feature)),
    (sym::ual, Unstable(sym::loongarch_target_feature)),
    // tidy-alphabetical-end
];

/// When rustdoc is running, provide a list of all known features so that all their respective
/// primitives may be documented.
///
/// IMPORTANT: If you're adding another feature list above, make sure to add it to this iterator!
pub fn all_known_features() -> impl Iterator<Item = TargetFeature> {
    std::iter::empty()
        .chain(ARM_ALLOWED_FEATURES.iter())
        .chain(AARCH64_ALLOWED_FEATURES.iter())
        .chain(X86_ALLOWED_FEATURES.iter())
        .chain(HEXAGON_ALLOWED_FEATURES.iter())
        .chain(POWERPC_ALLOWED_FEATURES.iter())
        .chain(MIPS_ALLOWED_FEATURES.iter())
        .chain(RISCV_ALLOWED_FEATURES.iter())
        .chain(WASM_ALLOWED_FEATURES.iter())
        .chain(BPF_ALLOWED_FEATURES.iter())
        .chain(CSKY_ALLOWED_FEATURES)
        .chain(LOONGARCH_ALLOWED_FEATURES)
        .copied()
}

impl super::spec::Target {
    pub fn supported_target_features(&self) -> &'static [TargetFeature] {
        match &*self.arch {
            "arm" => ARM_ALLOWED_FEATURES,
            "aarch64" => AARCH64_ALLOWED_FEATURES,
            "x86" | "x86_64" => X86_ALLOWED_FEATURES,
            "hexagon" => HEXAGON_ALLOWED_FEATURES,
            "mips" | "mips32r6" | "mips64" | "mips64r6" => MIPS_ALLOWED_FEATURES,
            "powerpc" | "powerpc64" => POWERPC_ALLOWED_FEATURES,
            "riscv32" | "riscv64" => RISCV_ALLOWED_FEATURES,
            "wasm32" | "wasm64" => WASM_ALLOWED_FEATURES,
            "bpf" => BPF_ALLOWED_FEATURES,
            "csky" => CSKY_ALLOWED_FEATURES,
            "loongarch64" => LOONGARCH_ALLOWED_FEATURES,
            _ => &[],
        }
    }

    pub fn tied_target_features(&self) -> &'static [&'static [Symbol]] {
        match &*self.arch {
            "aarch64" => AARCH64_TIED_FEATURES,
            _ => &[],
        }
    }
}
