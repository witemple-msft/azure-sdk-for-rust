use crate::table::prelude::*;
use crate::table::responses::*;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use http::{method::Method, StatusCode};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SubmitTransactionBuilder<'a> {
    partition_key_client: &'a PartitionKeyClient,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a> SubmitTransactionBuilder<'a> {
    pub(crate) fn new(partition_key_client: &'a PartitionKeyClient) -> Self {
        Self {
            partition_key_client,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute(
        &self,
        batch: &Transaction,
    ) -> Result<SubmitTransactionResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self
            .partition_key_client
            .table_client()
            .url()
            .join("$batch")?;

        self.timeout.append_to_url_query(&mut url);
        println!("url = {}", url);

        let payload = batch.to_string()?;
        println!("payload == {}", payload);

        let request = self.partition_key_client.prepare_request(
            url.as_str(),
            &Method::POST,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = request.header(
                    "Content-Type",
                    &format!(
                        "multipart/mixed; boundary=batch_{}",
                        batch.batch_uuid().to_hyphenated_ref()
                    ),
                );
                request
            },
            Some(bytes::Bytes::from(payload)),
        )?;

        println!("request == {:#?}\n", request);

        let response = self
            .partition_key_client
            .http_client()
            .execute_request_check_status(request.0, StatusCode::ACCEPTED)
            .await?;

        Ok((&response).try_into()?)
    }
}
