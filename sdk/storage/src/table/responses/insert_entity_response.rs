use crate::EntityWithMetadata;
use azure_core::{
    headers::{etag_from_headers, string_from_headers_mandatory, CommonStorageResponseHeaders},
    prelude::Etag,
};
use bytes::Bytes;
use http::Response;
use serde::de::DeserializeOwned;
use std::convert::{TryFrom, TryInto};
use url::Url;

#[derive(Debug, Clone)]
pub struct InsertEntityResponse<E>
where
    E: DeserializeOwned,
{
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Etag,
    pub location: Url,
    pub entity_with_metadata: Option<EntityWithMetadata<E>>,
}

impl<E> TryFrom<&Response<Bytes>> for InsertEntityResponse<E>
where
    E: DeserializeOwned,
{
    type Error = crate::Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        println!("{}", std::str::from_utf8(response.body())?);
        println!("headers == {:#?}", response.headers());

        let entity_with_metadata =
            match string_from_headers_mandatory(response.headers(), "preference-applied")? {
                "return-no-content" => None,
                "return-content" => Some(response.try_into()?),
                _ => {
                    return Err(crate::Error::GenericErrorWithText(
                        "Unexpected value for preference-applied header".to_owned(),
                    ))
                }
            };

        Ok(InsertEntityResponse {
            common_storage_response_headers: response.headers().try_into()?,
            etag: etag_from_headers(response.headers())?.into(),
            location: Url::parse(string_from_headers_mandatory(
                response.headers(),
                "location",
            )?)?,
            entity_with_metadata,
        })
    }
}
