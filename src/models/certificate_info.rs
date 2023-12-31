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
pub struct CertificateInfo {
    #[serde(rename = "certificateType")]
    pub certificate_type: String,
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "issuedBy")]
    pub issued_by: String,
    #[serde(rename = "issuedTo")]
    pub issued_to: String,
    #[serde(rename = "validFrom")]
    pub valid_from: String,
    #[serde(rename = "validTo")]
    pub valid_to: String,
    #[serde(rename = "algorithm")]
    pub algorithm: String,
    #[serde(rename = "fingerprint")]
    pub fingerprint: Box<crate::models::Fingerprint>,
}

impl CertificateInfo {
    pub fn new(certificate_type: String, version: i32, serial_number: String, issued_by: String, issued_to: String, valid_from: String, valid_to: String, algorithm: String, fingerprint: crate::models::Fingerprint) -> CertificateInfo {
        CertificateInfo {
            certificate_type,
            version,
            serial_number,
            issued_by,
            issued_to,
            valid_from,
            valid_to,
            algorithm,
            fingerprint: Box::new(fingerprint),
        }
    }
}


