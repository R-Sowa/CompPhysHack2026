use crate::scalar::Scalar;

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
