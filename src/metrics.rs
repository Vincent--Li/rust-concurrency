// metrics data structure
// 基本功能: inc/dec/snapshot

use anyhow::Result;
use dashmap::DashMap;
use std::{fmt::Display, sync::Arc};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn incr(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Metrics::new()
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.data.iter() {
            writeln!(f, "{}: {}", item.key(), item.value())?;
        }
        Ok(())
    }
}
