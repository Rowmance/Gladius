use std::fmt::{Formatter, Result, Display};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CastleRights {
    NONE,
    KINGSIDE,
    QUEENSIDE,
    BOTH
}

impl Display for CastleRights {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Castle Rights: {:?}",  self)
    }
}
