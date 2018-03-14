use std::io;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Io(#[cause]
       io::Error),
    #[fail(display = "{}", _0)]
    DatabaseError(String),
    #[fail(display = "database is already open.")]
    AlreadyOpen,
    #[fail(display = "unknown block parent")]
    UnknownParent,
    #[fail(display = "can't canonize block")]
    CannotCanonize,
}
