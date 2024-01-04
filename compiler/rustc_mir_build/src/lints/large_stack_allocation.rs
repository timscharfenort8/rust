use crate::errors::*;

use rustc_hir::def_id::LocalDefId;
use rustc_hir::HirId;
use rustc_middle::thir::visit::Visitor;
use rustc_middle::thir::*;
use rustc_middle::ty::{self, TyCtxt};
use rustc_session::lint::builtin::DANGEROUS_LARGE_STACK_ALLOCATIONS;
use rustc_session::lint::builtin::LARGE_STACK_ALLOCATIONS;
use rustc_span::ErrorGuaranteed;

const MIN_LARGE_SIZE: u64 = 1024 * 1024 * 1; // 1 Mio
const MIN_DANGEROUS_SIZE: u64 = 1024 * 1024 * 1024 * 1; // 1 Gio

pub(crate) fn check(tcx: TyCtxt<'_>, def_id: LocalDefId) -> Result<(), ErrorGuaranteed> {
    let (thir, expr) = tcx.thir_body(def_id)?;
    let thir = thir.borrow();
    let mut visitor = LargeArraysVisitor {
        tcx,
        thir: &*thir,
        param_env: tcx.param_env(def_id),
        lint_level: tcx.local_def_id_to_hir_id(def_id),
    };
    visitor.visit_expr(&thir[expr]);
    Ok(())
}

struct LargeArraysVisitor<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    param_env: ty::ParamEnv<'tcx>,
    thir: &'a Thir<'tcx>,
    lint_level: HirId,
}

impl<'a, 'thir> LargeArraysVisitor<'a, 'thir> {
    fn with_lint_level<T>(
        &mut self,
        new_lint_level: LintLevel,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        if let LintLevel::Explicit(hir_id) = new_lint_level {
            let old_lint_level = self.lint_level;
            self.lint_level = hir_id;
            let ret = f(self);
            self.lint_level = old_lint_level;
            ret
        } else {
            f(self)
        }
    }
}

impl<'a, 'tcx> Visitor<'a, 'tcx> for LargeArraysVisitor<'a, 'tcx> {
    fn thir(&self) -> &'a Thir<'tcx> {
        self.thir
    }

    fn visit_expr(&mut self, expr: &'a Expr<'tcx>) {
        match expr.kind {
            ExprKind::Scope { value, lint_level, .. } => {
                self.with_lint_level(lint_level, |this| {
                    this.visit_expr(&this.thir[value]);
                });
                return;
            }
            ExprKind::Tuple { .. }
            | ExprKind::Adt { .. }
            | ExprKind::Array { .. }
            | ExprKind::Repeat { .. }
            | ExprKind::Call { .. }
                if let Ok(layout) = self.tcx.layout_of(self.param_env.and(expr.ty)) =>
            {
                let suggestion =
                    matches!(expr.kind, ExprKind::Array { .. } | ExprKind::Repeat { .. }).then(
                        || LargeStackAllocSuggestions::VecMacro {
                            start_span: expr.span.shrink_to_lo(),
                        },
                    );

                let size = || {
                    let (size_quantity, size_unit) = human_readable_bytes(layout.size.bytes());
                    format!("{:.2} {}", size_quantity, size_unit)
                };

                if layout.size.bytes() >= MIN_DANGEROUS_SIZE {
                    self.tcx.emit_spanned_lint(
                        DANGEROUS_LARGE_STACK_ALLOCATIONS,
                        self.lint_level,
                        expr.span,
                        DangerousLargeStackAlloc { ty: expr.ty, size: size(), suggestion },
                    );
                } else if layout.size.bytes() >= MIN_LARGE_SIZE {
                    self.tcx.emit_spanned_lint(
                        LARGE_STACK_ALLOCATIONS,
                        self.lint_level,
                        expr.span,
                        LargeStackAlloc { ty: expr.ty, size: size(), suggestion },
                    );
                }
            }
            _ => {}
        }
        visit::walk_expr(self, expr);
    }

    fn visit_arm(&mut self, arm: &'a Arm<'tcx>) {
        self.with_lint_level(arm.lint_level, |this| {
            visit::walk_arm(this, arm);
        });
    }

    fn visit_stmt(&mut self, stmt: &'a Stmt<'tcx>) {
        match stmt.kind {
            StmtKind::Let { lint_level, .. } => {
                self.with_lint_level(lint_level, |this| {
                    visit::walk_stmt(this, stmt);
                });
            }
            StmtKind::Expr { .. } => {
                visit::walk_stmt(self, stmt);
            }
        }
    }
}

/// Formats a number of bytes into a human readable SI-prefixed size.
/// Returns a tuple of `(quantity, units)`.
//
// Taken from Cargo:
// https://github.com/rust-lang/cargo/blob/2ce45605d9db521b5fd6c1211ce8de6055fdb24e/src/cargo/util/mod.rs#L88-L95
pub fn human_readable_bytes(bytes: u64) -> (f32, &'static str) {
    static UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    let bytes = bytes as f32;
    let i = ((bytes.log2() / 10.0) as usize).min(UNITS.len() - 1);
    (bytes / 1024_f32.powi(i as i32), UNITS[i])
}
