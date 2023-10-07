/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwoFactorAuthenticationSecret {
    #[serde(rename = "secretKey")]
    pub secret_key: String,
    #[serde(rename = "qrCode")]
    pub qr_code: Box<crate::models::QrCode>,
    #[serde(rename = "scratchCodes")]
    pub scratch_codes: Vec<i32>,
}

impl TwoFactorAuthenticationSecret {
    pub fn new(secret_key: String, qr_code: crate::models::QrCode, scratch_codes: Vec<i32>) -> TwoFactorAuthenticationSecret {
        TwoFactorAuthenticationSecret {
            secret_key,
            qr_code: Box::new(qr_code),
            scratch_codes,
        }
    }
}

