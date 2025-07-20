#![allow(warnings)]

use anyhow::{Result, anyhow};
use std::collections::HashSet;
use tokio::sync::RwLock;

pub struct ConcurrentSet {
    inner: RwLock<HashSet<String>>,
}

impl ConcurrentSet {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashSet::new()),
        }
    }

    pub async fn insert(&self, name: String) -> Result<bool> {
        let mut guard = self.inner.write().await;

        if guard.contains(&name) {
            return Err(anyhow!("This name exists! Try another one"));
        }

        Ok(guard.insert(name))
    }

    pub async fn contains(&self, name: String) -> bool {
        let guard = self.inner.read().await;
        guard.contains(&name)
    }
}
