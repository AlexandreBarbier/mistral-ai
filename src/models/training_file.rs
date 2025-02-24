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
pub struct TrainingFile {
    #[serde(rename = "file_id")]
    pub file_id: uuid::Uuid,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

impl TrainingFile {
    pub fn new(file_id: uuid::Uuid) -> TrainingFile {
        TrainingFile {
            file_id,
            weight: None,
        }
    }
}

