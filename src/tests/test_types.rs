use types::*;

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

#[test]
fn test_display_message_kind() {
    assert_eq!(format!("{}", MessageKind::Numeric(0)), "000");
    assert_eq!(format!("{}", MessageKind::Numeric(1)), "001");
    assert_eq!(format!("{}", MessageKind::Numeric(12)), "012");
    assert_eq!(format!("{}", MessageKind::Numeric(123)), "123");
    assert_eq!(
        format!("{}", MessageKind::Command("privmsg".into())),
        "PRIVMSG"
    );
    assert_eq!(
        format!("{}", MessageKind::Command("PRIVMSG".into())),
        "PRIVMSG"
    );
}

#[test]
fn test_eq_message_kind() {
    assert_eq!(MessageKind::Numeric(0), MessageKind::Numeric(0));
    assert_eq!(
        MessageKind::Command("privmsg".into()),
        MessageKind::Command("privmsg".into())
    );
    assert_eq!(
        MessageKind::Command("PRIVMSG".into()),
        MessageKind::Command("privmsg".into())
    );
    assert_ne!(
        MessageKind::Command("privmsg".into()),
        MessageKind::Command("notice".into())
    );
}
