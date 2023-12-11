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
pub struct ContinueUploadResponse {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "cloud")]
    pub cloud: String,
    #[serde(rename = "bucket")]
    pub bucket: String,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(rename = "last_modified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(rename = "multipart_upload_id")]
    pub multipart_upload_id: String,
    #[serde(rename = "iv", skip_serializing_if = "Option::is_none")]
    pub iv: Option<Vec<i32>>,
    #[serde(rename = "encrypted", skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "parts", skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<crate::models::ContinueUploadPhysicalPart>>,
    #[serde(rename = "copy_src_bucket", skip_serializing_if = "Option::is_none")]
    pub copy_src_bucket: Option<String>,
    #[serde(rename = "copy_src_key", skip_serializing_if = "Option::is_none")]
    pub copy_src_key: Option<String>,
}

impl ContinueUploadResponse {
    pub fn new(id: i32, tag: String, cloud: String, bucket: String, region: String, key: String, multipart_upload_id: String) -> ContinueUploadResponse {
        ContinueUploadResponse {
            id,
            tag,
            cloud,
            bucket,
            region,
            key,
            size: None,
            last_modified: None,
            etag: None,
            multipart_upload_id,
            iv: None,
            encrypted: None,
            parts: None,
            copy_src_bucket: None,
            copy_src_key: None,
        }
    }
}


