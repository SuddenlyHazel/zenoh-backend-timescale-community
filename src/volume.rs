use anyhow::anyhow;
use async_trait::async_trait;
use log::error;
use tokio_postgres::NoTls;
use zenoh::Result as ZResult;

use zenoh_backend_traits::{Capability, Storage, Volume, config::StorageConfig};
use zenoh_util::ffi::JsonValue;

use crate::storage::TimescaleDbStorage;

pub struct TimescaleDbVolume {}

#[async_trait]
impl Volume for TimescaleDbVolume {
    fn get_admin_status(&self) -> JsonValue {
        todo!()
    }

    fn get_capability(&self) -> Capability {
        Capability {
            persistence: zenoh_backend_traits::Persistence::Durable,
            history: zenoh_backend_traits::History::All,
        }
    }

    async fn create_storage(&self, props: StorageConfig) -> ZResult<Box<dyn Storage>> {
        let mut volume_cfg = props.volume_cfg.into_serde_value();
        let volume_cfg = volume_cfg.as_object_mut().ok_or_else(|| {
            anyhow!("TimescaleDB backed storages requires volume-specific configuration")
        })?;

        let connection_uri = volume_cfg
            .get("connectionUri")
            .map(|v| v.as_str())
            .flatten()
            .ok_or_else(|| {
                anyhow!("TimescaleDB configuration requires the connectionUri property")
            })?;

        let (client, connection) = tokio_postgres::connect(connection_uri, NoTls)
            .await
            .map_err(|e| anyhow!("Timescale Backend failed to connect to the database {e}"))?;

        // Spawn off the connection task
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                // Todo.. We should add some more details here about "which" volume failed
                error!("TimescaleDB connection filed {e}");
            }
        });

        let storage = TimescaleDbStorage::new(client);
        Ok(Box::new(storage))
    }
}

pub enum BackendTlsMode {
    None,
    // TODO add support for tls using postgres_native_tls. We'll have to do a little
    // Pem file dance
    Tls,
}
