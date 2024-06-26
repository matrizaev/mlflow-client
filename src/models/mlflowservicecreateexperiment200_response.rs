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
pub struct Mlflowservicecreateexperiment200Response {
    /// Unique identifier for the experiment.
    #[serde(rename = "experiment_id", skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
}

impl Mlflowservicecreateexperiment200Response {
    pub fn new() -> Mlflowservicecreateexperiment200Response {
        Mlflowservicecreateexperiment200Response {
            experiment_id: None,
        }
    }
}

