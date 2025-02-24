/*
 * Mistral AI API
 *
 * Our Chat Completion and Embeddings APIs specification. Create your account on [La Plateforme](https://console.mistral.ai) to get access and read the [docs](https://docs.mistral.ai) to learn how to use it.
 *
 * The version of the OpenAPI document: 0.0.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassificationResponse {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<serde_json::Value>>,
}

impl ClassificationResponse {
    pub fn new() -> ClassificationResponse {
        ClassificationResponse {
            id: None,
            model: None,
            results: None,
        }
    }
}

