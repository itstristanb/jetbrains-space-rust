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
pub struct EsGithubProfileLoginDetailsAllEmailsInner {
    #[serde(rename = "first")]
    pub first: String,
    #[serde(rename = "second")]
    pub second: bool,
}

impl EsGithubProfileLoginDetailsAllEmailsInner {
    pub fn new(first: String, second: bool) -> EsGithubProfileLoginDetailsAllEmailsInner {
        EsGithubProfileLoginDetailsAllEmailsInner {
            first,
            second,
        }
    }
}


