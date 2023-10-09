use crate::error::Error;
use core::result::Result;

const res: Result<(), Error> = if cfg!(myfeature) {
    Ok(())
} else {
    panic!("myfeature not found in command line args");
};

pub fn main() -> Result<(), Error> {
    return res;
}
