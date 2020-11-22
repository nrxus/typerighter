use std::fmt;

#[derive(Debug)]
pub struct Left(pub u16);

impl fmt::Display for Left {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\r{}", termion::cursor::Right(self.0))
    }
}
