use crate::blob::blob::requests::*;
use crate::blob::prelude::*;
use crate::core::prelude::*;
use crate::shared_access_signature::SharedAccessSignature;
use azure_core::prelude::*;
use azure_core::HttpClient;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

pub trait AsBlobClient<BN: Into<String>> {
    fn as_blob_client(&self, blob_name: BN) -> Arc<BlobClient>;
}

impl<BN: Into<String>> AsBlobClient<BN> for Arc<ContainerClient> {
    fn as_blob_client(&self, blob_name: BN) -> Arc<BlobClient> {
        BlobClient::new(self.clone(), blob_name.into())
    }
}

#[derive(Debug, Clone)]
pub struct BlobClient {
    container_client: Arc<ContainerClient>,
    blob_name: String,
}

impl BlobClient {
    pub(crate) fn new(container_client: Arc<ContainerClient>, blob_name: String) -> Arc<Self> {
        Arc::new(Self {
            container_client,
            blob_name,
        })
    }

    pub fn blob_name(&self) -> &str {
        &self.blob_name
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.container_client
            .storage_client()
            .storage_account_client()
            .http_client()
    }

    #[allow(dead_code)]
    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient {
        self.container_client
            .storage_client()
            .storage_account_client()
    }

    #[allow(dead_code)]
    pub(crate) fn container_client(&self) -> &ContainerClient {
        self.container_client.as_ref()
    }

    pub(crate) fn url_with_segments<'a, I>(
        &'a self,
        segments: I,
    ) -> Result<url::Url, url::ParseError>
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.container_client
            .url_with_segments(Some(self.blob_name.as_str()).into_iter().chain(segments))
    }

    pub fn get(&self) -> GetBlobBuilder {
        GetBlobBuilder::new(self)
    }

    pub fn get_properties(&self) -> GetBlobPropertiesBuilder {
        GetBlobPropertiesBuilder::new(self)
    }

    pub fn get_metadata(&self) -> GetBlobMetadataBuilder {
        GetBlobMetadataBuilder::new(self)
    }

    pub fn update_page<'a>(
        &'a self,
        ba512_range: BA512Range,
        content: impl Into<Bytes>,
    ) -> UpdatePageBuilder<'a> {
        UpdatePageBuilder::new(self, ba512_range, content)
    }

    pub fn delete(&self) -> DeleteBlobBuilder {
        DeleteBlobBuilder::new(self)
    }

    pub fn delete_snapshot(&self, snapshot: Snapshot) -> DeleteBlobSnapshotBuilder {
        DeleteBlobSnapshotBuilder::new(self, snapshot)
    }

    pub fn delete_version_id(&self, version_id: VersionId) -> DeleteBlobVersionBuilder {
        DeleteBlobVersionBuilder::new(self, version_id)
    }

    pub fn copy<'a>(&'a self, copy_source: &'a str) -> CopyBlobBuilder<'a> {
        CopyBlobBuilder::new(self, copy_source)
    }

    pub fn copy_from_url<'a>(&'a self, copy_source: &'a str) -> CopyBlobFromUrlBuilder<'a> {
        CopyBlobFromUrlBuilder::new(self, copy_source)
    }

    pub fn put_page_blob(&self, length: u128) -> PutPageBlobBuilder {
        PutPageBlobBuilder::new(self, length)
    }

    pub fn put_append_blob(&self) -> PutAppendBlobBuilder {
        PutAppendBlobBuilder::new(self)
    }

    pub fn get_block_list(&self) -> GetBlockListBuilder {
        GetBlockListBuilder::new(self)
    }

    pub fn put_block_list<'a>(&'a self, block_list: &'a BlockList) -> PutBlockListBuilder {
        PutBlockListBuilder::new(self, block_list)
    }

    pub fn put_block_blob<'a>(&'a self, body: impl Into<Bytes>) -> PutBlockBlobBuilder<'a> {
        PutBlockBlobBuilder::new(self, body.into())
    }

    pub fn append_block<'a>(&'a self, body: impl Into<Bytes>) -> AppendBlockBuilder<'a> {
        AppendBlockBuilder::new(self, body.into())
    }

    pub fn put_block<'a>(
        &'a self,
        block_id: impl Into<BlockId>,
        body: impl Into<Bytes>,
    ) -> PutBlockBuilder<'a> {
        PutBlockBuilder::new(self, block_id, body)
    }

    pub fn clear_page(&self, ba512_range: BA512Range) -> ClearPageBuilder {
        ClearPageBuilder::new(self, ba512_range)
    }

    pub fn acquire_lease<LD: Into<LeaseDuration>>(
        &self,
        lease_duration: LD,
    ) -> AcquireLeaseBuilder {
        AcquireLeaseBuilder::new(self, lease_duration.into())
    }

    pub fn break_lease(&self) -> BreakLeaseBuilder {
        BreakLeaseBuilder::new(self)
    }

    pub fn generate_signed_blob_url(
        &self,
        signature: &SharedAccessSignature,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = self.url_with_segments(None)?;
        Ok(format!("{}?{}", url.as_str(), signature.token()))
    }

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), crate::Error> {
        self.container_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}
