use sled::Db;
use crate::storage::StorageEngine;
use anyhow::Result;

struct SledStorage(Db);

impl StorageEngine for SledStorage {
    fn get(&mut self, k: String) -> anyhow::Result<Option<String>> {
        let s = &self.0;
        Ok(s
            .get(k)?
            .map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&mut self, k: String) -> anyhow::Result<()> {
        let s = &self.0;
        s.remove(k)?;
        Ok(())
    }

    fn set(&mut self, k: String, v: String) -> anyhow::Result<()> {
        let s = &self.0;
        s.insert(k, v.into_bytes())?;
        Ok(())
    }
}