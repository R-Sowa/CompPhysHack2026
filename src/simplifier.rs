use crate::ast::{Expr, Pauli};
use crate::scalar::Scalar;

pub fn simplify(expr: Expr) -> Expr {
    match expr {
        Expr::Mul(factors) if factors.len() == 2 => simplify_pair(factors),
        other => other,
    }
}

fn simplify_pair(factors: Vec<Expr>) -> Expr {
    match (&factors[0], &factors[1]) {
        (Expr::Sym(lhs), Expr::Sym(rhs)) => {
            let (scalar, result) = multiply_paulis(*lhs, *rhs);
            if scalar == Scalar::from_int(1) {
                Expr::Sym(result)
            } else {
                Expr::Mul(vec![Expr::Scalar(scalar), Expr::Sym(result)])
            }
        }
        _ => Expr::Mul(factors),
    }
}

fn multiply_paulis(lhs: Pauli, rhs: Pauli) -> (Scalar, Pauli) {
    match (lhs, rhs) {
        (Pauli::I, other) | (other, Pauli::I) => (Scalar::from_int(1), other),
        (Pauli::X, Pauli::X) | (Pauli::Y, Pauli::Y) | (Pauli::Z, Pauli::Z) => {
            (Scalar::from_int(1), Pauli::I)
        }
        (Pauli::X, Pauli::Y) => (Scalar::i(), Pauli::Z),
        (Pauli::Y, Pauli::Z) => (Scalar::i(), Pauli::X),
        (Pauli::Z, Pauli::X) => (Scalar::i(), Pauli::Y),
        (Pauli::Y, Pauli::X) => (-Scalar::i(), Pauli::Z),
        (Pauli::Z, Pauli::Y) => (-Scalar::i(), Pauli::X),
        (Pauli::X, Pauli::Z) => (-Scalar::i(), Pauli::Y),
    }
}
