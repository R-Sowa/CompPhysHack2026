use pauli_repl::ast::{Expr, Pauli};
use pauli_repl::simplifier::simplify;

#[test]
fn test_xy_reduces_to_iz() {
    let expr = Expr::Mul(vec![Expr::Sym(Pauli::X), Expr::Sym(Pauli::Y)]);
    let simplified = simplify(expr);
    assert_eq!(simplified.to_string(), "iZ");
}

#[test]
fn test_yx_reduces_to_minus_iz() {
    let expr = Expr::Mul(vec![Expr::Sym(Pauli::Y), Expr::Sym(Pauli::X)]);
    let simplified = simplify(expr);
    assert_eq!(simplified.to_string(), "-iZ");
}

#[test]
fn test_xx_reduces_to_identity() {
    let expr = Expr::Mul(vec![Expr::Sym(Pauli::X), Expr::Sym(Pauli::X)]);
    let simplified = simplify(expr);
    assert_eq!(simplified.to_string(), "I");
}

#[test]
fn test_yz_reduces_to_ix() {
    let expr = Expr::Mul(vec![Expr::Sym(Pauli::Y), Expr::Sym(Pauli::Z)]);
    let simplified = simplify(expr);
    assert_eq!(simplified.to_string(), "iX");
}

#[test]
fn test_zx_reduces_to_iy() {
    let expr = Expr::Mul(vec![Expr::Sym(Pauli::Z), Expr::Sym(Pauli::X)]);
    let simplified = simplify(expr);
    assert_eq!(simplified.to_string(), "iY");
}

#[test]
fn test_xz_reduces_to_minus_iy() {
    let expr = Expr::Mul(vec![Expr::Sym(Pauli::X), Expr::Sym(Pauli::Z)]);
    let simplified = simplify(expr);
    assert_eq!(simplified.to_string(), "-iY");
}

#[test]
fn test_yy_reduces_to_identity() {
    let expr = Expr::Mul(vec![Expr::Sym(Pauli::Y), Expr::Sym(Pauli::Y)]);
    let simplified = simplify(expr);
    assert_eq!(simplified.to_string(), "I");
}

#[test]
fn test_zz_reduces_to_identity() {
    let expr = Expr::Mul(vec![Expr::Sym(Pauli::Z), Expr::Sym(Pauli::Z)]);
    let simplified = simplify(expr);
    assert_eq!(simplified.to_string(), "I");
}
