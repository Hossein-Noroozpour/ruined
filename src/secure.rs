use super::info::Info;

pub struct SecureConnection {
    pub info: Info,
}

impl SecureConnection {
    fn new(info: Info) -> Self {
        SecureConnection {
            info: info
        }
    }
}