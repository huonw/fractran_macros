//! A macro that parses & compiles fractran code at compile-time.

#![crate_type="dylib"]

#![feature(quote, plugin_registrar, macro_rules)]

extern crate num;
extern crate syntax;
extern crate rustc;

extern crate slow_primes;

use std::cmp;

use syntax::{ast, codemap, ptr};
use syntax::ext::base::{mod, ExtCtxt, MacResult, MacExpr, DummyResult};
use syntax::ext::build::AstBuilder;
use rustc::plugin::Registry;

use fract::Fract;

mod fract;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(registrar: &mut Registry) {
    registrar.register_macro("fractran", fractran);
}
fn fractran(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult+'static> {
    let exprs = match base::get_exprs_from_tts(cx, sp, tts)
                          .and_then(|e| extract_exprs(cx, e.as_slice())) {
        None => return DummyResult::expr(sp),
        Some(e) => e,
    };

    let factored = fract::factorise(exprs.as_slice());

    let (length, states) = construct_states(cx, sp, factored.as_slice());

    MacExpr::new(quote_expr!(cx, {
        mod inner {
            extern crate fractran_support;

            pub struct Machine {
                _regs: [u32, .. $length]
            }
            impl Iterator<()> for Machine {
                fn next(&mut self) -> Option<()> {
                    $states;
                    Some(())
                }
            }
            impl fractran_support::Fractran for Machine {
                fn state<'a>(&'a self) -> &'a [u32] {
                    self._regs.as_slice()
                }
            }

            pub fn construct(init: &[u32]) -> Machine {
                // FIXME
                assert!(init.len() <= $length);

                let mut prog = Machine { _regs: [0, .. $length] };
                for (place, data) in prog._regs.mut_iter().zip(init.iter()) {
                    *place = *data
                }
                prog
            }
        }
        inner::construct
    }))
}

fn extract_exprs(cx: &mut ExtCtxt, exprs: &[ptr::P<ast::Expr>]) -> Option<Vec<Fract<Vec<uint>>>> {
    exprs.iter().map(|e| extract_expr(cx, &**e)).collect()
}

fn extract_expr(cx: &mut ExtCtxt, expr: &ast::Expr) -> Option<Fract<Vec<uint>>> {
    match expr.node {
        ast::ExprLit(ref lit) => {
            match lit.node {
                ast::LitInt(x, _) => Some(Fract { numer: vec![x as uint], denom: vec![1] }),
                _ => {
                    cx.span_err(lit.span, "unsupported value in `fractran!`");
                    None
                }
            }
        }
        ast::ExprBinary(ast::BiAdd, ref l, ref r) => {
            extract_expr(cx, &**r).and_then(|r| {
                extract_expr(cx, &**l).map(|mut l| {
                    let top = l.numer.iter().chain(r.denom.iter()).fold(1, |p, &a| p * a)
                        + r.numer.iter().chain(l.denom.iter()).fold(1, |p, &a| p * a);
                    l.denom.push_all(r.denom.as_slice());
                    Fract {
                        numer: vec![top],
                        denom: l.denom
                    }
                })
            })
        }
        ast::ExprBinary(ast::BiMul, ref l, ref r) => {
            extract_expr(cx, &**r).and_then(|Fract { numer: r_n, denom: r_d }| {
                extract_expr(cx, &**l).map(|Fract { numer: mut l_n, denom: mut l_d }| {
                    l_n.push_all(r_n.as_slice());
                    l_d.push_all(r_d.as_slice());
                    Fract {
                        numer: l_n,
                        denom: l_d,
                    }
                })
            })
        }
        ast::ExprBinary(ast::BiDiv, ref l, ref r) => {
            extract_expr(cx, &**r).and_then(|Fract { numer: r_n, denom: r_d }| {
                extract_expr(cx, &**l).map(|Fract { numer: mut l_n, denom: mut l_d }| {
                    l_n.push_all(r_d.as_slice());
                    l_d.push_all(r_n.as_slice());
                    Fract {
                        numer: l_n,
                        denom: l_d,
                    }
                })
            })
        }
        // abusing ^
        ast::ExprBinary(ast::BiBitXor, ref l, ref r) => {
            extract_expr(cx, &**l).and_then(|Fract { numer, denom }| {
                extract_expr(cx, &**r).and_then(|Fract { numer: exp_n, denom: exp_d }| {
                    if !exp_d.iter().all(|n| *n == 1) {
                        cx.span_err(r.span, "exponent must be an integer");
                        return None;
                    }

                    let repeat = exp_n.iter().fold(1, |a, &n| a * n);
                    let mut ret_n = Vec::with_capacity(numer.len() * repeat);
                    let mut ret_d = Vec::with_capacity(numer.len() * repeat);
                    for _ in range(0u, repeat) {
                        ret_n.push_all(numer.as_slice());
                        ret_d.push_all(denom.as_slice());
                    }

                    Some(Fract { numer: ret_n, denom: ret_d })
                })
            })
        }
        ast::ExprParen(ref e) => extract_expr(cx, &**e),
        _ => {
            cx.span_err(expr.span,
                        "unsupported expression type inside `fractran!`");
            None
        }
    }
}

fn construct_states(cx: &ExtCtxt, sp: codemap::Span,
                    fracts: &[(Fract<u64>,
                               Fract<Vec<uint>>)]) -> (uint, ptr::P<ast::Expr>) {
    let length = fracts.iter().map(|&(_, ref f)| {
        cmp::max(f.numer.len(), f.denom.len())
    }).max().unwrap_or(0);


    let regs = quote_expr!(cx, self._regs);

    let st = State {
        cx: cx, sp: sp, regs: regs
    };

    let mut states = quote_expr!(cx, { return None });

    for &(_, ref f) in fracts.iter().rev() {
        let cond = st.check_regs(f.denom.as_slice());
        let exec = st.step_regs(f.numer.as_slice(), f.denom.as_slice());
        states = cx.expr(sp, ast::ExprIf(cond, exec, Some(states)));
    }

    (length, states)
}

struct State<'a, 'b: 'a> {
    cx: &'a ExtCtxt<'b>,
    sp: codemap::Span,
    regs: ptr::P<ast::Expr>,
}

impl<'a, 'b> State<'a, 'b> {
    fn check_reg(&self, reg: uint, thresh: uint) -> ptr::P<ast::Expr> {
        let regs = &self.regs;
        let thresh = thresh as u32;
        quote_expr!(&*self.cx, $regs[$reg] >= $thresh )
    }
    fn step_reg(&self, reg: uint, amt: uint) -> ptr::P<ast::Expr> {
        let regs = &self.regs;
        let amt = amt as u32;
        quote_expr!(&*self.cx, $regs[$reg] += $amt)
    }

    fn check_regs(&self, values: &[uint]) -> ptr::P<ast::Expr> {
        let mut res = self.cx.expr_bool(self.sp, true);

        for (reg, &v) in values.iter().enumerate() {
            if v > 0 {
                res = self.cx.expr_binary(self.sp, ast::BiAnd,
                                          self.check_reg(reg, v), res)
            }
        }
        res
    }
    #[allow(unsigned_negate)]
    fn step_regs(&self, increase: &[uint], decrease: &[uint]) -> ptr::P<ast::Block> {
        let mut stmts = vec![];
        for (reg, &v) in decrease.iter().enumerate() {
            if v > 0 {
                stmts.push(self.cx.stmt_expr(self.step_reg(reg, -v)))
            }
        }
        for (reg, &v) in increase.iter().enumerate() {
            if v > 0 {
                stmts.push(self.cx.stmt_expr(self.step_reg(reg, v)))
            }
        }
        self.cx.block(self.sp, stmts, None)
    }
}
