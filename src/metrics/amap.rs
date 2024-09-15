use anyhow::Result;
use std::sync::atomic::Ordering;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    sync::{atomic::AtomicI64, Arc},
};

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    // map 入参使用 &'static -> 只能初始化一次
    pub fn new(metric_names: &[&'static str]) -> Self {
        let data = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        AmapMetrics {
            data: Arc::new(data),
        }
    }

    pub fn incr(&self, key: impl AsRef<str>) -> Result<()> {
        let counter = self
            .data
            .get(key.as_ref())
            .ok_or(anyhow::anyhow!("unknown metric {}", key.as_ref()))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        AmapMetrics {
            data: Arc::clone(&self.data),
        }
    }
}

impl Display for AmapMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (key, counter) in self.data.iter() {
            writeln!(f, "{}: {}", key, counter.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}
