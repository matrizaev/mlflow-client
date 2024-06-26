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
pub struct Modelregistryservicegetregisteredmodel200Response {
    #[serde(rename = "registered_model", skip_serializing_if = "Option::is_none")]
    pub registered_model: Option<Box<models::Mlflowregisteredmodel>>,
}

impl Modelregistryservicegetregisteredmodel200Response {
    pub fn new() -> Modelregistryservicegetregisteredmodel200Response {
        Modelregistryservicegetregisteredmodel200Response {
            registered_model: None,
        }
    }
}

