/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrgIndustryDto {
    #[serde(rename = "SoftwareDevelopment")]
    SoftwareDevelopment,
    #[serde(rename = "Science")]
    Science,
    #[serde(rename = "Education")]
    Education,
    #[serde(rename = "Finance")]
    Finance,
    #[serde(rename = "Health")]
    Health,
    #[serde(rename = "Manufacturing")]
    Manufacturing,
    #[serde(rename = "Retail")]
    Retail,
    #[serde(rename = "Transportation")]
    Transportation,
    #[serde(rename = "Media")]
    Media,
    #[serde(rename = "Hospitality")]
    Hospitality,
    #[serde(rename = "ProfessionalServices")]
    ProfessionalServices,
    #[serde(rename = "NonProfit")]
    NonProfit,
    #[serde(rename = "Other")]
    Other,

}

impl ToString for OrgIndustryDto {
    fn to_string(&self) -> String {
        match self {
            Self::SoftwareDevelopment => String::from("SoftwareDevelopment"),
            Self::Science => String::from("Science"),
            Self::Education => String::from("Education"),
            Self::Finance => String::from("Finance"),
            Self::Health => String::from("Health"),
            Self::Manufacturing => String::from("Manufacturing"),
            Self::Retail => String::from("Retail"),
            Self::Transportation => String::from("Transportation"),
            Self::Media => String::from("Media"),
            Self::Hospitality => String::from("Hospitality"),
            Self::ProfessionalServices => String::from("ProfessionalServices"),
            Self::NonProfit => String::from("NonProfit"),
            Self::Other => String::from("Other"),
        }
    }
}

impl Default for OrgIndustryDto {
    fn default() -> OrgIndustryDto {
        Self::SoftwareDevelopment
    }
}




