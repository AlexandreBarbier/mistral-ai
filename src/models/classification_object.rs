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
pub struct ClassificationObject {
    /// Classifier result thresholded
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<std::collections::HashMap<String, bool>>,
    /// Classifier result
    #[serde(rename = "category_scores", skip_serializing_if = "Option::is_none")]
    pub category_scores: Option<std::collections::HashMap<String, f64>>,
}

impl ClassificationObject {
    pub fn new() -> ClassificationObject {
        ClassificationObject {
            categories: None,
            category_scores: None,
        }
    }
}

