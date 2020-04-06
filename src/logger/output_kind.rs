/// Identify the three kinds of output supported.
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum OutputKind<'f> {
    STDOUT,
    STDERR,
    FILE(&'f str),
}