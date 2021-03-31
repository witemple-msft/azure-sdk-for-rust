#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
pub mod metric_namespaces {
    use crate::models::*;
    pub async fn list(
        operation_config: &crate::OperationConfig,
        resource_uri: &str,
        start_time: Option<&str>,
    ) -> std::result::Result<MetricNamespaceCollection, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/microsoft.insights/metricNamespaces",
            operation_config.base_path(),
            resource_uri
        );
        let mut url = url::Url::parse(url_str).map_err(|source| list::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| list::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        if let Some(start_time) = start_time {
            url.query_pairs_mut().append_pair("startTime", start_time);
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| list::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| list::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: MetricNamespaceCollection =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError {
                        source,
                        body: rsp_body.clone(),
                    })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse = serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Err(list::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
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
