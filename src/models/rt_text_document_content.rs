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
pub struct RtTextDocumentContent {
    #[serde(rename = "document")]
    pub document: Box<crate::models::RtDocument>,
}

impl RtTextDocumentContent {
    pub fn new(document: crate::models::RtDocument) -> RtTextDocumentContent {
        RtTextDocumentContent {
            document: Box::new(document),
        }
    }
}


