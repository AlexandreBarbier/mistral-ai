/*
 * Mistral AI API
 *
 * Our Chat Completion and Embeddings APIs specification. Create your account on [La Plateforme](https://console.mistral.ai) to get access and read the [docs](https://docs.mistral.ai) to learn how to use it.
 *
 * The version of the OpenAPI document: 0.0.2
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`files_api_routes_delete_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesApiRoutesDeleteFileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`files_api_routes_download_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesApiRoutesDownloadFileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`files_api_routes_get_signed_url`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesApiRoutesGetSignedUrlError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`files_api_routes_list_files`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesApiRoutesListFilesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`files_api_routes_retrieve_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesApiRoutesRetrieveFileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`files_api_routes_upload_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesApiRoutesUploadFileError {
    UnknownValue(serde_json::Value),
}

/// Delete a file.
pub async fn files_api_routes_delete_file(
    configuration: &configuration::Configuration,
    file_id: &str,
) -> Result<models::DeleteFileOut, Error<FilesApiRoutesDeleteFileError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_file_id = file_id;

    let uri_str = format!(
        "{}/v1/files/{file_id}",
        configuration.base_path,
        file_id = crate::apis::urlencode(p_file_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<FilesApiRoutesDeleteFileError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Download a file
pub async fn files_api_routes_download_file(
    configuration: &configuration::Configuration,
    file_id: &str,
) -> Result<reqwest::Response, Error<FilesApiRoutesDownloadFileError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_file_id = file_id;

    let uri_str = format!(
        "{}/v1/files/{file_id}/content",
        configuration.base_path,
        file_id = crate::apis::urlencode(p_file_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(resp)
    } else {
        let content = resp.text().await?;
        let entity: Option<FilesApiRoutesDownloadFileError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn files_api_routes_get_signed_url(
    configuration: &configuration::Configuration,
    file_id: &str,
    expiry: Option<i32>,
) -> Result<models::FileSignedUrl, Error<FilesApiRoutesGetSignedUrlError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_file_id = file_id;
    let p_expiry = expiry;

    let uri_str = format!(
        "{}/v1/files/{file_id}/url",
        configuration.base_path,
        file_id = crate::apis::urlencode(p_file_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_expiry {
        req_builder = req_builder.query(&[("expiry", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<FilesApiRoutesGetSignedUrlError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a list of files that belong to the user's organization.
pub async fn files_api_routes_list_files(
    configuration: &configuration::Configuration,
    page: Option<i32>,
    page_size: Option<i32>,
    sample_type: Option<Vec<models::SampleType>>,
    source: Option<Vec<models::Source>>,
    search: Option<&str>,
    purpose: Option<models::FilePurpose>,
) -> Result<models::ListFilesOut, Error<FilesApiRoutesListFilesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_page = page;
    let p_page_size = page_size;
    let p_sample_type = sample_type;
    let p_source = source;
    let p_search = search;
    let p_purpose = purpose;

    let uri_str = format!("{}/v1/files", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sample_type {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .into_iter()
                    .map(|p| ("sample_type".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "sample_type",
                &param_value
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_source {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .into_iter()
                    .map(|p| ("source".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "source",
                &param_value
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_search {
        req_builder = req_builder.query(&[("search", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_purpose {
        req_builder = req_builder.query(&[("purpose", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<FilesApiRoutesListFilesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns information about a specific file.
pub async fn files_api_routes_retrieve_file(
    configuration: &configuration::Configuration,
    file_id: &str,
) -> Result<models::RetrieveFileOut, Error<FilesApiRoutesRetrieveFileError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_file_id = file_id;

    let uri_str = format!(
        "{}/v1/files/{file_id}",
        configuration.base_path,
        file_id = crate::apis::urlencode(p_file_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<FilesApiRoutesRetrieveFileError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Upload a file that can be used across various endpoints.  The size of individual files can be a maximum of 512 MB. The Fine-tuning API only supports .jsonl files.  Please contact us if you need to increase these storage limits.
pub async fn files_api_routes_upload_file(
    configuration: &configuration::Configuration,
    file: std::path::PathBuf,
    purpose: Option<models::FilePurpose>,
) -> Result<models::UploadFileOut, Error<FilesApiRoutesUploadFileError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let _p_file = file;
    let p_purpose = purpose;

    let uri_str = format!("{}/v1/files", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut multipart_form = reqwest::multipart::Form::new();
    // TODO: support file upload for 'file' parameter
    if let Some(param_value) = p_purpose {
        multipart_form = multipart_form.text("purpose", param_value.to_string());
    }
    req_builder = req_builder.multipart(multipart_form);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<FilesApiRoutesUploadFileError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
