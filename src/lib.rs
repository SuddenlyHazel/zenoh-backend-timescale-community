use async_trait::async_trait;
use zenoh::{bytes::{Encoding, ZBytes}, key_expr::OwnedKeyExpr, time::Timestamp, try_init_log_from_env, Result as ZResult};
use zenoh_backend_traits::{Storage, StorageInsertionResult, StoredData, VolumeInstance, config::VolumeConfig};
use zenoh_plugin_trait::{Plugin, plugin_long_version, plugin_version};
use zenoh_util::ffi::JsonValue;

pub mod storage;
pub mod volume;

#[cfg(feature = "dynamic_plugin")]
zenoh_plugin_trait::declare_plugin!(TimescaleDbBackend);

pub struct TimescaleDbBackend {}

impl Plugin for TimescaleDbBackend {
    type StartArgs = VolumeConfig;

    type Instance = VolumeInstance;

    const DEFAULT_NAME: &'static str = "timescale_backend_community";

    const PLUGIN_VERSION: &'static str = plugin_version!();

    const PLUGIN_LONG_VERSION: &'static str = plugin_long_version!();

    fn start(name: &str, args: &Self::StartArgs) -> zenoh::Result<Self::Instance> {
        try_init_log_from_env();

        let config = args.rest.into_serde_map();
        todo!()
    }
}

#[async_trait]
impl Storage for TimescaleDbBackend {
    /// Returns the status that will be sent as a reply to a query
    /// on the administration space for this storage.
    fn get_admin_status(&self) -> JsonValue {
        todo!()
    }

    /// Function called for each incoming data ([`Sample`](zenoh::sample::Sample)) to be stored in this storage.
    /// A key can be `None` if it matches the `strip_prefix` exactly.
    /// In order to avoid data loss, the storage must store the `value` and `timestamp` associated with the `None` key
    /// in a manner suitable for the given backend technology
    async fn put(
        &mut self,
        key: Option<OwnedKeyExpr>,
        payload: ZBytes,
        encoding: Encoding,
        timestamp: Timestamp,
    ) -> ZResult<StorageInsertionResult> {
        todo!()
    }

    /// Function called for each incoming delete request to this storage.
    /// A key can be `None` if it matches the `strip_prefix` exactly.
    /// In order to avoid data loss, the storage must delete the entry corresponding to the `None` key
    /// in a manner suitable for the given backend technology
    async fn delete(
        &mut self,
        key: Option<OwnedKeyExpr>,
        timestamp: Timestamp,
    ) -> ZResult<StorageInsertionResult> {
        todo!()
    }

    /// Function to retrieve the sample associated with a single key.
    /// A key can be `None` if it matches the `strip_prefix` exactly.
    /// In order to avoid data loss, the storage must retrieve the `value` and `timestamp` associated with the `None` key
    /// in a manner suitable for the given backend technology
    async fn get(
        &mut self,
        key: Option<OwnedKeyExpr>,
        parameters: &str,
    ) -> ZResult<Vec<StoredData>> {
        todo!()
    }

    /// Function called to get the list of all storage content (key, timestamp)
    /// The latest Timestamp corresponding to each key is either the timestamp of the delete or put whichever is the latest.
    /// Remember to fetch the entry corresponding to the `None` key
    async fn get_all_entries(&self) -> ZResult<Vec<(Option<OwnedKeyExpr>, Timestamp)>> {
        todo!()
    }
}