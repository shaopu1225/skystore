/*
 * FastAPI
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContinueUploadRequest {
    #[serde(rename = "bucket")]
    pub bucket: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "client_from_region")]
    pub client_from_region: String,
    #[serde(rename = "multipart_upload_id")]
    pub multipart_upload_id: String,
    #[serde(rename = "do_list_parts", skip_serializing_if = "Option::is_none")]
    pub do_list_parts: Option<bool>,
    #[serde(rename = "copy_src_bucket", skip_serializing_if = "Option::is_none")]
    pub copy_src_bucket: Option<String>,
    #[serde(rename = "copy_src_key", skip_serializing_if = "Option::is_none")]
    pub copy_src_key: Option<String>,
}

impl ContinueUploadRequest {
    pub fn new(
        bucket: String,
        key: String,
        client_from_region: String,
        multipart_upload_id: String,
    ) -> ContinueUploadRequest {
        ContinueUploadRequest {
            bucket,
            key,
            client_from_region,
            multipart_upload_id,
            do_list_parts: None,
            copy_src_bucket: None,
            copy_src_key: None,
        }
    }
}
