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
pub struct Modelregistryservicecreatemodelversion200Response {
    #[serde(rename = "model_version", skip_serializing_if = "Option::is_none")]
    pub model_version: Option<Box<models::Mlflowmodelversion>>,
}

impl Modelregistryservicecreatemodelversion200Response {
    pub fn new() -> Modelregistryservicecreatemodelversion200Response {
        Modelregistryservicecreatemodelversion200Response {
            model_version: None,
        }
    }
}

