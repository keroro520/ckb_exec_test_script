// Import from `core` instead of from `std` since we are in no-std mode
use crate::error::Error;
use core::result::Result;

pub fn main() -> Result<(), Error> {
    Ok(())
}

