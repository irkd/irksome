use types::Source;

#[test]
fn test_display_source() {
    assert_eq!(
        format!(
            "{}",
            Source::Server {
                host: "foo.bar.baz".into(),
            }
        ),
        "foo.bar.baz"
    );
    assert_eq!(
        format!(
            "{}",
            Source::User {
                nick: "foo".into(),
                ident: "bar".into(),
                host: "baz.qux".into(),
            }
        ),
        "foo!bar@baz.qux"
    );
    assert_eq!(
        format!(
            "{}",
            Source::User {
                nick: "foo".into(),
                ident: "~bar".into(),
                host: "unaffilliated/users/foo".into(),
            }
        ),
        "foo!~bar@unaffilliated/users/foo"
    );
}
