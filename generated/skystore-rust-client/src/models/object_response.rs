/*
 * FastAPI
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectResponse {
    #[serde(rename = "bucket")]
    pub bucket: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "size")]
    pub size: u64,
    #[serde(rename = "etag")]
    pub etag: String,
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<crate::models::Status>>,
    #[serde(rename = "last_modified")]
    pub last_modified: String,
}

impl ObjectResponse {
    pub fn new(bucket: String, key: String, size: u64, etag: String, last_modified: String) -> ObjectResponse {
        ObjectResponse {
            bucket,
            key,
            size,
            etag,
            status: None,
            last_modified,
        }
    }
}


