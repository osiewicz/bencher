use std::convert::TryFrom;

use async_trait::async_trait;
use bencher_json::JsonRestart;

use crate::{
    bencher::{backend::Backend, sub::SubCmd, wide::Wide},
    cli::admin::CliAdminRestart,
    CliError,
};

const RESTART_PATH: &str = "/v0/admin/restart";

#[derive(Debug, Clone)]
pub struct Restart {
    pub delay: Option<u64>,
    pub backend: Backend,
}

impl TryFrom<CliAdminRestart> for Restart {
    type Error = CliError;

    fn try_from(create: CliAdminRestart) -> Result<Self, Self::Error> {
        let CliAdminRestart { delay, backend } = create;
        Ok(Self {
            delay,
            backend: backend.try_into()?,
        })
    }
}

impl From<Restart> for JsonRestart {
    fn from(restart: Restart) -> Self {
        let Restart { delay, backend: _ } = restart;
        Self { delay }
    }
}

#[async_trait]
impl SubCmd for Restart {
    async fn exec(&self, _wide: &Wide) -> Result<(), CliError> {
        let restart: JsonRestart = self.clone().into();
        self.backend.post(RESTART_PATH, &restart).await?;
        Ok(())
    }
}