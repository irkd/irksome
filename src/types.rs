use std::fmt;

/// A message source.
#[derive(Debug, Eq, PartialEq)]
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
