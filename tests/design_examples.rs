use pauli_repl::repl::render_once;

#[test]
fn test_design_example_xy() {
    let output = render_once("X*Y").unwrap();
    assert!(output.contains("result: iZ"));
}

#[test]
fn test_design_example_commutator() {
    let output = render_once("[X,Y]").unwrap();
    assert!(output.contains("result: 2iZ"));
}

#[test]
fn test_design_example_square() {
    let output = render_once("(X+Y)*(X+Y)").unwrap();
    assert!(output.contains("result: 2I"));
}
