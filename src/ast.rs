use crate::scalar::Scalar;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Pauli {
    I,
    X,
    Y,
    Z,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Scalar(Scalar),
    Sym(Pauli),
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
    Commutator(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Pauli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Pauli::I => "I",
            Pauli::X => "X",
            Pauli::Y => "Y",
            Pauli::Z => "Z",
        };

        write!(f, "{symbol}")
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Scalar(scalar) => write!(f, "{scalar}"),
            Expr::Sym(pauli) => write!(f, "{pauli}"),
            Expr::Add(terms) => {
                for (index, term) in terms.iter().enumerate() {
                    let rendered = term.to_string();
                    if index == 0 {
                        write!(f, "{rendered}")?;
                    } else if let Some(rest) = rendered.strip_prefix('-') {
                        write!(f, " - {rest}")?;
                    } else {
                        write!(f, " + {rendered}")?;
                    }
                }
                Ok(())
            }
            Expr::Mul(factors) => {
                for factor in factors {
                    write!(f, "{factor}")?;
                }
                Ok(())
            }
            Expr::Commutator(lhs, rhs) => write!(f, "[{lhs},{rhs}]"),
        }
    }
}
