use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub trait Store<T, K> {
    fn load_all(&self) -> std::io::Result<HashMap<K, T>>;
    fn save_all(&self, items: &HashMap<K, T>) -> std::io::Result<()>;
}

/// 文件型实现：`data/dict/security.json` 等。将来换 PG 实现同名 trait 即可。
pub struct JsonFileStore {
    base: PathBuf,
}

impl JsonFileStore {
    pub fn new(base: impl AsRef<Path>) -> Self {
        Self { base: base.as_ref().to_path_buf() }
    }
    fn file(&self, name: &str) -> PathBuf {
        self.base.join(format!("{name}.json"))
    }
}

impl<T, K> Store<T, K> for JsonFileStore
where
    T: serde::Serialize + serde::de::DeserializeOwned,
    K: std::hash::Hash + Eq + serde::Serialize + serde::de::DeserializeOwned,
{
    fn load_all(&self) ->  std::io::Result<HashMap<K, T>> {
        let path = self.file(std::any::type_name::<T>());   // 或用表名参数化
        let raw = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&raw)?)
    }
    fn save_all(&self, items: &HashMap<K, T>) -> std::io::Result<()> {
        let path = self.file(std::any::type_name::<T>());
        std::fs::write(&path, serde_json::to_string_pretty(items)?)?;
        Ok(())
    }
}