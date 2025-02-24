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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseRetrieveModelV1ModelsModelIdGet {
    #[serde(rename="base")]
    Base(Box<models::BaseModelCard>),
    #[serde(rename="fine-tuned")]
    FineTuned(Box<models::FtModelCard>),
}

impl Default for ResponseRetrieveModelV1ModelsModelIdGet {
    fn default() -> Self {
        Self::Base(Default::default())
    }
}


