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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FilePurpose {
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "batch")]
    Batch,

}

impl std::fmt::Display for FilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FineTune => write!(f, "fine-tune"),
            Self::Batch => write!(f, "batch"),
        }
    }
}

impl Default for FilePurpose {
    fn default() -> FilePurpose {
        Self::FineTune
    }
}

