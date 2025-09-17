use gittype::error::GitTypeError;

#[test]
fn database_error_wraps_message() {
    let error = GitTypeError::database_error("failure".into());

    if let GitTypeError::DatabaseError(inner) = error {
        assert!(inner.to_string().contains("failure"));
    } else {
        panic!("expected database error variant");
    }
}
