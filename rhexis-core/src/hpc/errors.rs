#[repr(i32)]
pub enum HpcErrorCode {
    Ok = 0,
    BadInput = 1,
    HardwareFailure = 2,
    Unsupported = 3,
    Unknown = -1,
}
