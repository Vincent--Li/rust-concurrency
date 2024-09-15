// metrics data structure
// 基本功能: inc/dec/snapshot

use anyhow::{anyhow, Result};
use std::{
    collections::BTreeMap,
    fmt::{self, Display},
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<RwLock<BTreeMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn incr(&self, key: impl Into<String>) -> Result<()> {
        let mut m = self
            .data
            .write()
            .map_err(|_| anyhow!("metrics lock failed"))?;
        let counter = m.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<BTreeMap<String, i64>> {
        Ok(self
            .data
            .read()
            .map_err(|e| anyhow!(e.to_string()))?
            .clone())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Metrics::new()
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let snapshot = self.data.read().map_err(|_| fmt::Error {})?;
        for (key, value) in snapshot.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}
