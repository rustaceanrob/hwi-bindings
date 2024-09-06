use std::sync::Arc;

pub use async_hwi::DeviceKind;
use async_hwi::{ledger::LedgerSimulator, HWI};
use bitcoin::bip32::DerivationPath;
use tokio::sync::Mutex;

pub struct Device {
    hwi: Arc<Mutex<dyn HWI + Send + Sync>>,
}

impl Device {
    async fn new(kind: DeviceKind) -> Result<Self, GenericError> {
        match kind {
            DeviceKind::LedgerSimulator => Ok(Self {
                hwi: Arc::new(Mutex::new(
                    LedgerSimulator::try_connect()
                        .await
                        .map_err(|_| GenericError::Any)?,
                )),
            }),
            _ => Err(GenericError::Any),
        }
    }

    async fn get_master_xpub(&self) -> Result<String, GenericError> {
        self.hwi
            .lock()
            .await
            .get_extended_pubkey(&DerivationPath::master())
            .await
            .map(|xpub| xpub.to_string())
            .map_err(|_| GenericError::Any)
    }
}

#[derive(Debug, Clone)]
pub enum GenericError {
    Any,
}

impl core::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "generic FFI error")
    }
}

impl std::error::Error for GenericError {}

uniffi::include_scaffolding!("hwi");
