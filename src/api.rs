use totp_rs::{Algorithm, TOTP};

use crate::types::{TwoFA, OtpAuthCreate};
use crate::error::{Result, CustomError};

impl TwoFA {
    pub async fn new_with_secret(secret: String) -> Result<Self> {
        let secret = base32::decode(
            base32::Alphabet::RFC4648 { padding: false },
            secret.as_str(),
        ).ok_or(CustomError::UBError)?;

        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret,
            None,
            "".to_string(),
        )?;

        let code = totp.generate_current()
            .map_err(|_| CustomError::UBError)?;

        let ttl = totp.ttl()
            .map_err(|_| CustomError::UBError)?;

        Ok(Self { code, ttl: ttl.to_string()})
    }

    pub async fn new_with_url(data: OtpAuthCreate) -> Result<Self> {
        let totp = TOTP::from_url(data.url)?;

        let code = totp.generate_current()
            .map_err(|_| CustomError::UBError)?;

        let ttl = totp.ttl()
            .map_err(|_| CustomError::UBError)?;

        Ok(Self { code, ttl: ttl.to_string()})
    }
}