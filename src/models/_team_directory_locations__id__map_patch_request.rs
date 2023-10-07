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
pub struct TeamDirectoryLocationsIdMapPatchRequest {
    #[serde(rename = "mapPictureId")]
    pub map_picture_id: String,
}

impl TeamDirectoryLocationsIdMapPatchRequest {
    pub fn new(map_picture_id: String) -> TeamDirectoryLocationsIdMapPatchRequest {
        TeamDirectoryLocationsIdMapPatchRequest {
            map_picture_id,
        }
    }
}

