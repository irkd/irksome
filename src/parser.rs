use std::str;

use combine::*;
use combine::byte::*;
use combine::primitives::RangeStream;
use combine::range::*;

use types::Source;

/// Whether or not a character is valid for a nickname when it is not the first character.
fn is_nick_trailing_char(c: u8) -> bool {
    match c {
        b'A'...b'Z'
        | b'a'...b'z'
        | b'0'...b'9'
        | b'['
        | b']'
        | b'`'
        | b'^'
        | b'\\'
        | b'-'
        | b'{'
        | b'}' => true,
        _ => false,
    }
}

/// Whether or not a character is valid for an ident.
///
/// RFC 1459 and 2811 only go so far as to specify it must be non-whitespace. However,
/// in practice, they are limited to alphanumeric + `~`.
///
/// As such, this will accept non-compliant identifiers, but they will be valid for the server that
/// sends them.
fn is_ident_char(c: u8) -> bool {
    match c {
        b'\0' | b' ' | b'\t' | b'\r' | b'\n' | b'@' => false,
        _ => true,
    }
}

/// Whether or not a character is valid for a hostname.
///
/// This will result in accepting not-strictly-compliant hostnames, but some IRCDs (e.g., Freenode)
/// will provide vhosts that are not FQDNs, such as `unaffilliated/username`.
fn is_host_char(c: u8) -> bool {
    match c {
        b'\0' | b' ' | b'\t' | b'\r' | b'\n' => false,
        _ => true,
    }
}

/// Parse the source of a message.
pub fn source<'a, I>() -> impl Parser<Input = I, Output = Source>
where
    I: RangeStream<Item = u8, Range = &'a [u8]>,
    I::Error: ParseError<u8, &'a [u8], I::Position>,
    <I::Error as ParseError<I::Item, I::Range, I::Position>>::StreamError: From<str::Utf8Error>,
{
    let nick = || recognize((letter(), skip_many(satisfy(is_nick_trailing_char))));
    let ident = || take_while1(is_ident_char);
    let host = || take_while1(is_host_char);

    let user = || (nick().skip(byte(b'!')), ident().skip(byte(b'@')), host());

    choice!(
        try(
            user().and_then(|(nick, ident, host)| -> Result<Source, str::Utf8Error> {
                let nick = unsafe { str::from_utf8_unchecked(nick) };
                let ident = str::from_utf8(ident)?;
                let host = str::from_utf8(host)?;

                Ok(Source::User {
                    nick: nick.into(),
                    ident: ident.into(),
                    host: host.into(),
                })
            })
        ),
        host().map(|host| {
            let host = unsafe { str::from_utf8_unchecked(host) };
            Source::Server { host: host.into() }
        })
    )
}
