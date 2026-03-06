use pauli_repl::ast::{Expr, Pauli};
use pauli_repl::simplifier::simplify;

#[test]
fn test_x_plus_y_squared_reduces_to_2_identity() {
    let expr = Expr::Mul(vec![
        Expr::Add(vec![Expr::Sym(Pauli::X), Expr::Sym(Pauli::Y)]),
        Expr::Add(vec![Expr::Sym(Pauli::X), Expr::Sym(Pauli::Y)]),
    ]);
    let simplified = simplify(expr);
    assert_eq!(simplified.to_string(), "2I");
}
