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

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Content {
    String(String),
    #[default]
    None,
}

impl Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Content::String(ref x) => serializer.serialize_str(x),
            Content::None => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for Content {
    fn deserialize<D>(deserializer: D) -> Result<Content, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ContentVisitor;

        impl<'de> serde::de::Visitor<'de> for ContentVisitor {
            type Value = Content;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or array of strings")
            }

            fn visit_str<E>(self, value: &str) -> Result<Content, E>
            where
                E: serde::de::Error,
            {
                Ok(Content::String(value.to_string()))
            }

            fn visit_none<E>(self) -> Result<Content, E>
            where
                E: serde::de::Error,
            {
                Ok(Content::None)
            }
        }

        deserializer.deserialize_any(ContentVisitor)
    }
}
