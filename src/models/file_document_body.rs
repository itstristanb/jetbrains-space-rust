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
pub struct FileDocumentBody {
    #[serde(rename = "versionId")]
    pub version_id: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(rename = "fileSize")]
    pub file_size: i64,
}

impl FileDocumentBody {
    pub fn new(version_id: String, content_type: String, file_size: i64) -> FileDocumentBody {
        FileDocumentBody {
            version_id,
            content_type,
            file_size,
        }
    }
}


