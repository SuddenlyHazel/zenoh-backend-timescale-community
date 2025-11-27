use async_trait::async_trait;
use log::error;
use zenoh::{
    Result as ZResult,
    bytes::{Encoding, ZBytes},
    key_expr::OwnedKeyExpr,
    time::Timestamp,
};
use zenoh_backend_traits::{Storage, StorageInsertionResult, StoredData};
use zenoh_util::ffi::JsonValue;

pub struct TimescaleDbStorage {
    client: tokio_postgres::Client,
}

impl TimescaleDbStorage {
  pub fn new(client : tokio_postgres::Client) -> Self {
    Self {
      client
    }
  }
}

#[async_trait]
impl Storage for TimescaleDbStorage {
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
        if let Err(e) = self.client.check_connection().await {
            error!("timescale-storage-backend cannot put connection is invalid {e}");
            return Err(e.into());
        }
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
        if let Err(e) = self.client.check_connection().await {
            error!("timescale-storage-backend cannot delete connection is invalid {e}");
            return Err(e.into());
        }
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
        if let Err(e) = self.client.check_connection().await {
            error!("timescale-storage-backend cannot get connection is invalid {e}");
            return Err(e.into());
        }
        todo!()
    }

    /// Function called to get the list of all storage content (key, timestamp)
    /// The latest Timestamp corresponding to each key is either the timestamp of the delete or put whichever is the latest.
    /// Remember to fetch the entry corresponding to the `None` key
    async fn get_all_entries(&self) -> ZResult<Vec<(Option<OwnedKeyExpr>, Timestamp)>> {
        if let Err(e) = self.client.check_connection().await {
            error!("timescale-storage-backend cannot get_all_entries connection is invalid {e}");
            return Err(e.into());
        }
        todo!()
    }
}
