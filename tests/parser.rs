use pauli_repl::parser::parse_expr;

#[test]
fn test_parse_nested_product_and_sum() {
    let expr = parse_expr("(X+Y)*(X+Y)").unwrap();
    assert_eq!(expr.to_string(), "(X + Y)(X + Y)");
}

#[test]
fn test_parse_commutator() {
    let expr = parse_expr("[X,Y]").unwrap();
    assert_eq!(expr.to_string(), "[X,Y]");
}
