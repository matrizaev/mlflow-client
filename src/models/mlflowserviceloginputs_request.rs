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
pub struct MlflowserviceloginputsRequest {
    /// ID of the run to log under This field is required.
    #[serde(rename = "run_id")]
    pub run_id: String,
    /// Dataset inputs
    #[serde(rename = "datasets", skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<models::Mlflowdatasetinput>>,
}

impl MlflowserviceloginputsRequest {
    pub fn new(run_id: String) -> MlflowserviceloginputsRequest {
        MlflowserviceloginputsRequest {
            run_id,
            datasets: None,
        }
    }
}

