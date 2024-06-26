/*
 * Mlflow
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v2.11.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MlflowservicedeletetagRequest {
    /// ID of the run that the tag was logged under. Must be provided. This field is required.
    #[serde(rename = "run_id")]
    pub run_id: String,
    /// Name of the tag. Maximum size is 255 bytes. Must be provided. This field is required.
    #[serde(rename = "key")]
    pub key: String,
}

impl MlflowservicedeletetagRequest {
    pub fn new(run_id: String, key: String) -> MlflowservicedeletetagRequest {
        MlflowservicedeletetagRequest {
            run_id,
            key,
        }
    }
}

