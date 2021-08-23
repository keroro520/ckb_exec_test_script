use crate::ckb_std::error::SysError;
use crate::molecule::error::VerificationError as MolError;

/// Error
#[repr(i8)]
pub enum Error {
    SysErrorIndexOutOfBound = 1,
    SysErrorItemMissing,
    SysErrorLengthNotEnough,
    SysErrorEncoding,

    MolVerification,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        match err {
            SysError::IndexOutOfBound => Self::SysErrorIndexOutOfBound,
            SysError::ItemMissing => Self::SysErrorItemMissing,
            SysError::LengthNotEnough(_) => Self::SysErrorLengthNotEnough,
            SysError::Encoding => Self::SysErrorEncoding,
            SysError::Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}

impl From<MolError> for Error {
    fn from(_err: MolError) -> Self {
        Self::MolVerification
    }
}
