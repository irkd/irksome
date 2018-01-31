use combine::Parser;

use types::Source;
use parser::source;

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
