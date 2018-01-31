use combine::{Parser, State};

use types::*;
use parser::*;

#[test]
fn test_parse_source() {
    assert_eq!(
        source().easy_parse(&b"server.int"[..]),
        Ok((
            Source::Server {
                host: "server.int".into(),
            },
            &b""[..]
        ))
    );
    assert_eq!(
        source().easy_parse(&b"some-server.ny.us.somenet.org"[..]),
        Ok((
            Source::Server {
                host: "some-server.ny.us.somenet.org".into(),
            },
            &b""[..]
        ))
    );
    assert_eq!(
        source().easy_parse(&b"nick!ident@user.host"[..]),
        Ok((
            Source::User {
                nick: "nick".into(),
                ident: "ident".into(),
                host: "user.host".into(),
            },
            &b""[..]
        ))
    );
    assert_eq!(
        source().easy_parse(&b"nick[away]`-^\\{}!~ident@unaffilliated/user"[..]),
        Ok((
            Source::User {
                nick: "nick[away]`-^\\{}".into(),
                ident: "~ident".into(),
                host: "unaffilliated/user".into(),
            },
            &b""[..]
        ))
    );
}

#[test]
fn test_parse_message_kind() {
    assert_eq!(
        message_kind().easy_parse(&b"000"[..]),
        Ok((MessageKind::Numeric(0), &b""[..]))
    );
    assert_eq!(
        message_kind().easy_parse(&b"001"[..]),
        Ok((MessageKind::Numeric(1), &b""[..]))
    );
    assert_eq!(
        message_kind().easy_parse(&b"012"[..]),
        Ok((MessageKind::Numeric(12), &b""[..]))
    );
    assert_eq!(
        message_kind().easy_parse(&b"123"[..]),
        Ok((MessageKind::Numeric(123), &b""[..]))
    );
    assert_eq!(
        message_kind().easy_parse(&b"PRIVMSG"[..]),
        Ok((MessageKind::Command("PRIVMSG".into()), &b""[..]))
    );

    let result = message_kind().easy_parse(State::new(&b"0FOO"[..]));
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.position, 1);

    let result = message_kind().easy_parse(State::new(&b"00FOO"[..]));
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.position, 2);

    assert_eq!(
        message_kind().easy_parse(&b"FO0"[..]),
        Ok((MessageKind::Command("FO".into()), &b"0"[..]))
    );
}
