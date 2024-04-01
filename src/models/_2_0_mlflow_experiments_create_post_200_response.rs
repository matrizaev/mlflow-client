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
pub struct Model20MlflowExperimentsCreatePost200Response {
    #[serde(rename = "experiment_id", skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
}

impl Model20MlflowExperimentsCreatePost200Response {
    pub fn new() -> Model20MlflowExperimentsCreatePost200Response {
        Model20MlflowExperimentsCreatePost200Response {
            experiment_id: None,
        }
    }
}

