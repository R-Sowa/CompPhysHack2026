use crate::ast::Expr;
use crate::parser::{ParseError, parse_expr};
use crate::scalar::Scalar;
use crate::simplifier::simplify;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ReplError {
    Parse(ParseError),
}

impl fmt::Display for ReplError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReplError::Parse(err) => write!(f, "{err}"),
        }
    }
}

impl Error for ReplError {}

impl From<ParseError> for ReplError {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

pub fn render_once(input: &str) -> Result<String, ReplError> {
    let parsed = parse_expr(input)?;
    let steps = trace_steps(parsed.clone());
    let result = simplify(parsed.clone());

    let mut lines = vec![format!("parsed: {parsed}")];
    for (index, (rule, expr)) in steps.iter().enumerate() {
        lines.push(format!("step {}: {rule} -> {expr}", index + 1));
    }
    lines.push(format!("result: {result}"));

    Ok(lines.join("\n"))
}

fn trace_steps(expr: Expr) -> Vec<(&'static str, Expr)> {
    let mut steps = Vec::new();
    let mut current = expr.clone();

    if let Some(lowered) = lower_commutator(&current) {
        steps.push(("commutator", lowered.clone()));
        current = lowered;
    }

    if let Some(expanded) = expand_additions(&current) {
        steps.push(("distribute", expanded.clone()));
    }

    let simplified = simplify(expr);
    if steps.last().map(|(_, step)| step) != Some(&simplified) {
        steps.push(("simplify", simplified));
    }

    steps
}

fn lower_commutator(expr: &Expr) -> Option<Expr> {
    match expr {
        Expr::Commutator(lhs, rhs) => Some(Expr::Add(vec![
            Expr::Mul(vec![(**lhs).clone(), (**rhs).clone()]),
            Expr::Mul(vec![
                Expr::Scalar(Scalar::from_int(-1)),
                (**rhs).clone(),
                (**lhs).clone(),
            ]),
        ])),
        _ => None,
    }
}

fn expand_additions(expr: &Expr) -> Option<Expr> {
    match expr {
        Expr::Mul(factors) => {
            let mut products: Vec<Vec<Expr>> = vec![Vec::new()];
            let mut saw_add = false;

            for factor in factors {
                match factor {
                    Expr::Add(terms) => {
                        saw_add = true;
                        let mut next = Vec::new();
                        for prefix in &products {
                            for term in terms {
                                let mut new_prefix = prefix.clone();
                                new_prefix.push(term.clone());
                                next.push(new_prefix);
                            }
                        }
                        products = next;
                    }
                    other => {
                        for prefix in &mut products {
                            prefix.push(other.clone());
                        }
                    }
                }
            }

            if !saw_add {
                return None;
            }

            let terms = products
                .into_iter()
                .map(|factors| match factors.len() {
                    0 => Expr::Scalar(Scalar::from_int(1)),
                    1 => factors.into_iter().next().expect("single factor"),
                    _ => Expr::Mul(factors),
                })
                .collect::<Vec<_>>();

            Some(match terms.len() {
                0 => Expr::Scalar(Scalar::from_int(0)),
                1 => terms.into_iter().next().expect("single term"),
                _ => Expr::Add(terms),
            })
        }
        _ => None,
    }
}
