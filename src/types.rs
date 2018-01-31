use std::fmt;

#[derive(Debug, Eq, PartialEq)]
/// A message source.
pub enum Source {
    /// A server message source.
    Server {
        /// The server's hostname.
        host: String,
    },
    /// A user message source.
    User {
        /// The user's nickname.
        nick: String,
        /// The user's identifier.
        ident: String,
        /// The user's hostname.
        host: String,
    },
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Source::Server { ref host } => write!(f, "{}", host),
            Source::User {
                ref nick,
                ref ident,
                ref host,
            } => write!(f, "{}!{}@{}", nick, ident, host),
        }
    }
}

#[derive(Debug, Eq)]
/// The kind of a message.
pub enum MessageKind {
    /// A command.
    ///
    /// Commands are case-insensitive.
    Command(String),
    /// A numeric reply to a command, consisting of a 3-digit number.
    Numeric(u16),
}

impl fmt::Display for MessageKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MessageKind::Command(ref c) => write!(f, "{}", c.to_ascii_uppercase()),
            MessageKind::Numeric(n) => write!(f, "{:03}", n),
        }
    }
}

impl PartialEq for MessageKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&MessageKind::Command(ref l), &MessageKind::Command(ref r)) => {
                l.eq_ignore_ascii_case(r)
            }
            (&MessageKind::Numeric(ref l), &MessageKind::Numeric(ref r)) => l == r,
            _ => false,
        }
    }
}
