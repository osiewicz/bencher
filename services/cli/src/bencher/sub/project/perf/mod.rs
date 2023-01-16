use std::convert::TryFrom;

use async_trait::async_trait;
use bencher_json::{JsonPerfQuery, ResourceId};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{bencher::backend::Backend, cli::project::perf::CliPerf, CliError};

use crate::bencher::SubCmd;

#[derive(Debug, Clone)]
pub struct Perf {
    project: ResourceId,
    metric_kind: ResourceId,
    branches: Vec<Uuid>,
    testbeds: Vec<Uuid>,
    benchmarks: Vec<Uuid>,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    backend: Backend,
}

impl TryFrom<CliPerf> for Perf {
    type Error = CliError;

    fn try_from(perf: CliPerf) -> Result<Self, Self::Error> {
        let CliPerf {
            project,
            metric_kind,
            branches,
            testbeds,
            benchmarks,
            start_time,
            end_time,
            backend,
        } = perf;
        Ok(Self {
            project,
            metric_kind,
            branches,
            testbeds,
            benchmarks,
            start_time,
            end_time,
            backend: backend.try_into()?,
        })
    }
}

impl From<Perf> for JsonPerfQuery {
    fn from(perf: Perf) -> Self {
        let Perf {
            metric_kind,
            branches,
            testbeds,
            benchmarks,
            start_time,
            end_time,
            ..
        } = perf;
        Self {
            metric_kind,
            branches,
            testbeds,
            benchmarks,
            start_time,
            end_time,
        }
    }
}

#[async_trait]
impl SubCmd for Perf {
    async fn exec(&self) -> Result<(), CliError> {
        let perf: JsonPerfQuery = self.clone().into();
        self.backend
            .post(&format!("/v0/projects/{}/perf", self.project), &perf)
            .await?;
        Ok(())
    }
}
