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
pub struct GitCommitSignatureJbsRaw {
    #[serde(rename = "signature")]
    pub signature: String,
    #[serde(rename = "signedDataB64")]
    pub signed_data_b64: String,
}

impl GitCommitSignatureJbsRaw {
    pub fn new(signature: String, signed_data_b64: String) -> GitCommitSignatureJbsRaw {
        GitCommitSignatureJbsRaw {
            signature,
            signed_data_b64,
        }
    }
}


