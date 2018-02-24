use std::io;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "database is already opened.")]
    AlreadyOpen,
}
