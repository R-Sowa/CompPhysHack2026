use pauli_repl::repl::render_once;

#[test]
fn test_render_once_includes_steps_and_result() {
    let output = render_once("(X+Y)*(X+Y)").unwrap();
    assert!(output.contains("step 1:"));
    assert!(output.contains("result: 2I"));
}
