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
pub struct RetrieveFileOut {
    /// The unique identifier of the file.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The object type, which is always \"file\".
    #[serde(rename = "object")]
    pub object: String,
    /// The size of the file, in bytes.
    #[serde(rename = "bytes")]
    pub bytes: i32,
    /// The UNIX timestamp (in seconds) of the event.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The name of the uploaded file.
    #[serde(rename = "filename")]
    pub filename: String,
    /// The intended purpose of the uploaded file. Only accepts fine-tuning (`fine-tune`) for now.
    #[serde(rename = "purpose")]
    pub purpose: models::FilePurpose,
    #[serde(rename = "sample_type")]
    pub sample_type: models::SampleType,
    #[serde(rename = "num_lines", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub num_lines: Option<Option<i32>>,
    #[serde(rename = "source")]
    pub source: models::Source,
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl RetrieveFileOut {
    pub fn new(id: uuid::Uuid, object: String, bytes: i32, created_at: i32, filename: String, purpose: models::FilePurpose, sample_type: models::SampleType, source: models::Source, deleted: bool) -> RetrieveFileOut {
        RetrieveFileOut {
            id,
            object,
            bytes,
            created_at,
            filename,
            purpose,
            sample_type,
            num_lines: None,
            source,
            deleted,
        }
    }
}

