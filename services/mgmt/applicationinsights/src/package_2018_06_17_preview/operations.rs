#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
pub mod workbooks {
    use crate::models::*;
    pub async fn list_by_resource_group(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        category: &str,
        tags: &Vec<&str>,
        source_id: &str,
        can_fetch_content: Option<bool>,
    ) -> std::result::Result<WorkbooksListResult, list_by_resource_group::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.insights/workbooks",
            operation_config.base_path(),
            subscription_id,
            resource_group_name
        );
        let mut url = url::Url::parse(url_str).map_err(|source| list_by_resource_group::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| list_by_resource_group::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        url.query_pairs_mut().append_pair("category", category);
        url.query_pairs_mut().append_pair("sourceId", source_id);
        if let Some(can_fetch_content) = can_fetch_content {
            url.query_pairs_mut()
                .append_pair("canFetchContent", can_fetch_content.to_string().as_str());
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| list_by_resource_group::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| list_by_resource_group::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: WorkbooksListResult =
                    serde_json::from_slice(rsp_body).map_err(|source| list_by_resource_group::Error::DeserializeError {
                        source,
                        body: rsp_body.clone(),
                    })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: WorkbookError =
                    serde_json::from_slice(rsp_body).map_err(|source| list_by_resource_group::Error::DeserializeError {
                        source,
                        body: rsp_body.clone(),
                    })?;
                Err(list_by_resource_group::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list_by_resource_group {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::WorkbookError,
            },
            #[error("Failed to parse request URL: {}", source)]
            ParseUrlError { source: url::ParseError },
            #[error("Failed to build request: {}", source)]
            BuildRequestError { source: http::Error },
            #[error("Failed to execute request: {}", source)]
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            #[error("Failed to serialize request body: {}", source)]
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            #[error("Failed to deserialize response body: {}", source)]
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            #[error("Failed to get access token: {}", source)]
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn get(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> std::result::Result<Workbook, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.insights/workbooks/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            resource_name
        );
        let mut url = url::Url::parse(url_str).map_err(|source| get::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| get::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| get::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| get::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Workbook = serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: WorkbookError = serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Err(get::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::WorkbookError,
            },
            #[error("Failed to parse request URL: {}", source)]
            ParseUrlError { source: url::ParseError },
            #[error("Failed to build request: {}", source)]
            BuildRequestError { source: http::Error },
            #[error("Failed to execute request: {}", source)]
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            #[error("Failed to serialize request body: {}", source)]
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            #[error("Failed to deserialize response body: {}", source)]
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            #[error("Failed to get access token: {}", source)]
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn create_or_update(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        source_id: &str,
        workbook_properties: &Workbook,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.insights/workbooks/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            resource_name
        );
        let mut url = url::Url::parse(url_str).map_err(|source| create_or_update::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| create_or_update::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        url.query_pairs_mut().append_pair("sourceId", source_id);
        let req_body = azure_core::to_json(workbook_properties).map_err(|source| create_or_update::Error::SerializeError { source })?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| create_or_update::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| create_or_update::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Workbook = serde_json::from_slice(rsp_body).map_err(|source| create_or_update::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: Workbook = serde_json::from_slice(rsp_body).map_err(|source| create_or_update::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: WorkbookError =
                    serde_json::from_slice(rsp_body).map_err(|source| create_or_update::Error::DeserializeError {
                        source,
                        body: rsp_body.clone(),
                    })?;
                Err(create_or_update::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        #[derive(Debug)]
        pub enum Response {
            Ok200(Workbook),
            Created201(Workbook),
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::WorkbookError,
            },
            #[error("Failed to parse request URL: {}", source)]
            ParseUrlError { source: url::ParseError },
            #[error("Failed to build request: {}", source)]
            BuildRequestError { source: http::Error },
            #[error("Failed to execute request: {}", source)]
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            #[error("Failed to serialize request body: {}", source)]
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            #[error("Failed to deserialize response body: {}", source)]
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            #[error("Failed to get access token: {}", source)]
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn update(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        source_id: &str,
        workbook_update_parameters: Option<&WorkbookUpdateParameters>,
    ) -> std::result::Result<Workbook, update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.insights/workbooks/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            resource_name
        );
        let mut url = url::Url::parse(url_str).map_err(|source| update::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PATCH);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| update::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        url.query_pairs_mut().append_pair("sourceId", source_id);
        let req_body = if let Some(workbook_update_parameters) = workbook_update_parameters {
            azure_core::to_json(workbook_update_parameters).map_err(|source| update::Error::SerializeError { source })?
        } else {
            bytes::Bytes::from_static(azure_core::EMPTY_BODY)
        };
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| update::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| update::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Workbook = serde_json::from_slice(rsp_body).map_err(|source| update::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: WorkbookError = serde_json::from_slice(rsp_body).map_err(|source| update::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Err(update::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::WorkbookError,
            },
            #[error("Failed to parse request URL: {}", source)]
            ParseUrlError { source: url::ParseError },
            #[error("Failed to build request: {}", source)]
            BuildRequestError { source: http::Error },
            #[error("Failed to execute request: {}", source)]
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            #[error("Failed to serialize request body: {}", source)]
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            #[error("Failed to deserialize response body: {}", source)]
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            #[error("Failed to get access token: {}", source)]
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.insights/workbooks/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            resource_name
        );
        let mut url = url::Url::parse(url_str).map_err(|source| delete::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::DELETE);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| delete::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| delete::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| delete::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::CREATED => Ok(delete::Response::Created201),
            http::StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: WorkbookError = serde_json::from_slice(rsp_body).map_err(|source| delete::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Err(delete::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        #[derive(Debug)]
        pub enum Response {
            Created201,
            NoContent204,
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::WorkbookError,
            },
            #[error("Failed to parse request URL: {}", source)]
            ParseUrlError { source: url::ParseError },
            #[error("Failed to build request: {}", source)]
            BuildRequestError { source: http::Error },
            #[error("Failed to execute request: {}", source)]
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            #[error("Failed to serialize request body: {}", source)]
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            #[error("Failed to deserialize response body: {}", source)]
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            #[error("Failed to get access token: {}", source)]
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
