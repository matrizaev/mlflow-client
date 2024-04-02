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
pub struct MlflowservicerestorerunRequest {
    /// ID of the run to restore. This field is required.
    #[serde(rename = "run_id")]
    pub run_id: String,
}

impl MlflowservicerestorerunRequest {
    pub fn new(run_id: String) -> MlflowservicerestorerunRequest {
        MlflowservicerestorerunRequest {
            run_id,
        }
    }
}
