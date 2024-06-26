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
pub struct MlflowservicerestoreexperimentRequest {
    /// ID of the associated experiment. This field is required.
    #[serde(rename = "experiment_id")]
    pub experiment_id: String,
}

impl MlflowservicerestoreexperimentRequest {
    pub fn new(experiment_id: String) -> MlflowservicerestoreexperimentRequest {
        MlflowservicerestoreexperimentRequest {
            experiment_id,
        }
    }
}

