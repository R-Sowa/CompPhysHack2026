use pauli_repl::ast::{Expr, Pauli};
use pauli_repl::simplifier::simplify;

#[test]
fn test_commutator_xy_reduces_to_2iz() {
    let expr = Expr::Commutator(Box::new(Expr::Sym(Pauli::X)), Box::new(Expr::Sym(Pauli::Y)));
    let simplified = simplify(expr);
    assert_eq!(simplified.to_string(), "2iZ");
}
