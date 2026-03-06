use crate::ast::{Expr, Pauli};
use crate::scalar::Scalar;

pub fn simplify(expr: Expr) -> Expr {
    normalize(expr).into_expr()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LinearExpr {
    coeffs: [Scalar; 4],
}

impl LinearExpr {
    fn zero() -> Self {
        Self {
            coeffs: [Scalar::from_int(0); 4],
        }
    }

    fn scalar(value: Scalar) -> Self {
        let mut expr = Self::zero();
        expr.coeffs[pauli_index(Pauli::I)] = value;
        expr
    }

    fn basis(pauli: Pauli) -> Self {
        let mut expr = Self::zero();
        expr.coeffs[pauli_index(pauli)] = Scalar::from_int(1);
        expr
    }

    fn add_assign(&mut self, other: &Self) {
        for (lhs, rhs) in self.coeffs.iter_mut().zip(other.coeffs.iter()) {
            *lhs += *rhs;
        }
    }

    fn mul(&self, other: &Self) -> Self {
        let mut result = Self::zero();
        for lhs_index in 0..self.coeffs.len() {
            for rhs_index in 0..other.coeffs.len() {
                let lhs_coeff = self.coeffs[lhs_index];
                let rhs_coeff = other.coeffs[rhs_index];
                if lhs_coeff.is_zero() || rhs_coeff.is_zero() {
                    continue;
                }

                let lhs_pauli = index_pauli(lhs_index);
                let rhs_pauli = index_pauli(rhs_index);
                let (phase, product) = multiply_paulis(lhs_pauli, rhs_pauli);
                let contribution = lhs_coeff * rhs_coeff * phase;
                let slot = &mut result.coeffs[pauli_index(product)];
                *slot += contribution;
            }
        }
        result
    }

    fn into_expr(self) -> Expr {
        let mut terms = Vec::new();
        for index in 0..self.coeffs.len() {
            let coeff = self.coeffs[index];
            if coeff.is_zero() {
                continue;
            }

            let pauli = index_pauli(index);
            let term = match (coeff, pauli) {
                (coeff, Pauli::I) if coeff == Scalar::from_int(1) => Expr::Sym(Pauli::I),
                (coeff, pauli) if coeff == Scalar::from_int(1) => Expr::Sym(pauli),
                (coeff, pauli) => Expr::Mul(vec![Expr::Scalar(coeff), Expr::Sym(pauli)]),
            };
            terms.push(term);
        }

        match terms.len() {
            0 => Expr::Scalar(Scalar::from_int(0)),
            1 => terms.pop().expect("single term should exist"),
            _ => Expr::Add(terms),
        }
    }
}

fn normalize(expr: Expr) -> LinearExpr {
    match expr {
        Expr::Scalar(value) => LinearExpr::scalar(value),
        Expr::Sym(pauli) => LinearExpr::basis(pauli),
        Expr::Add(terms) => {
            let mut result = LinearExpr::zero();
            for term in terms {
                result.add_assign(&normalize(term));
            }
            result
        }
        Expr::Mul(factors) => {
            let mut result = LinearExpr::basis(Pauli::I);
            for factor in factors {
                result = result.mul(&normalize(factor));
            }
            result
        }
        Expr::Commutator(_, _) => todo!("commutator support is implemented in the next TDD step"),
    }
}

fn pauli_index(pauli: Pauli) -> usize {
    match pauli {
        Pauli::I => 0,
        Pauli::X => 1,
        Pauli::Y => 2,
        Pauli::Z => 3,
    }
}

fn index_pauli(index: usize) -> Pauli {
    match index {
        0 => Pauli::I,
        1 => Pauli::X,
        2 => Pauli::Y,
        3 => Pauli::Z,
        _ => unreachable!("Pauli basis index out of range"),
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
