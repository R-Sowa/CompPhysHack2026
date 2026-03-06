use pauli_repl::scalar::Scalar;

#[test]
fn test_i_squared_is_minus_one() {
    let i = Scalar::i();
    assert_eq!(i * i, Scalar::from_int(-1));
}
