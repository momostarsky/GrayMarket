//! 内存镜像：日初从 JSON 加载（将来换成 PG 快照 + LSN 锚点），
//! 运行期由单线程内核独占写、多读者读。
use std::path::Path;
use std::collections::HashMap;
use account::amount::Money;
use crate::domain::{Account, Asset, Position, Security, User};

/// 一次性载入的全部主数据 —— 对应「日初加载进内存」的边界。
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    pub users: HashMap<String, User>,
    pub accounts: HashMap<String, Account>,
    pub securities: HashMap<String, Security>,
    /// 可变状态：内核独占写。
    pub assets: HashMap<String, Asset>,
    /// key = `account_id:symbol`
    pub positions: HashMap<String, Position>,
    /// 加载时的数据源标识（将来是 `lsn`，现在是文件目录 + 文件数）。
    pub source: String,
}

impl Snapshot {
    /// 从 `data/` 目录加载全部主数据并做完整性校验。
    ///
    /// 失败以 `Err` 返回，由启动流程决定处置 —— 预期行为是终止启动：
    /// 带着脏字典开盘比启动失败危险得多（与 `amount.rs` 的「不静默」原则一致）。
    pub fn load(data_root: impl AsRef<Path>) -> anyhow::Result<Self> {
        let root = data_root.as_ref();
        let snapshot = Snapshot {
            users: load_json(&root.join("dict/user.json"))?,
            accounts: load_json(&root.join("state/account.json"))?,
            securities: load_json(&root.join("dict/security.json"))?,
            assets: load_json(&root.join("state/asset.json"))?,
            positions: load_json(&root.join("state/position.json"))?,
            source: root.display().to_string(),
        };
        snapshot.check_integrity()?;
        Ok(snapshot)
    }
    /// 加载即校验：外键完整性 + 资金非负。脏数据宁可不启动。
    fn check_integrity(&self) -> anyhow::Result<()> {
        for account in self.accounts.values() {
            anyhow::ensure!(
                self.users.contains_key(&account.user_id),
                "账户 {} 引用了不存在的用户 {}",
                account.account_id,
                account.user_id
            );
            anyhow::ensure!(
                self.assets.contains_key(&account.account_id),
                "账户 {} 缺少资金记录",
                account.account_id
            );
        }
        for asset in self.assets.values() {
            anyhow::ensure!(
                !asset.available.is_negative() && !asset.frozen.is_negative(),
                "账户 {} 资金为负",
                asset.account_id
            );
        }
        for position in self.positions.values() {
            let key = position_key(&position.account_id, &position.symbol);
            anyhow::ensure!(
                self.securities.contains_key(&position.symbol),
                "持仓 {key} 引用了未知证券 {}",
                position.symbol
            );
            anyhow::ensure!(
                self.assets.contains_key(&position.account_id),
                "持仓 {key} 引用了未知账户 {}",
                position.account_id
            );
            anyhow::ensure!(
                position.quantity >= position.available_qty,
                "持仓 {key} 可卖数量大于总数量"
            );
        }
        Ok(())
    }

    #[must_use]
    pub fn asset(&self, account_id: &str) -> Option<&Asset> {
        self.assets.get(account_id)
    }

    #[must_use]
    pub fn position(&self, account_id: &str, symbol: &str) -> Option<&Position> {
        self.positions.get(&position_key(account_id, symbol))
    }

    #[must_use]
    pub fn security(&self, symbol: &str) -> Option<&Security> {
        self.securities.get(symbol)
    }

    #[must_use]
    pub fn is_tradable(&self, account_id: &str, symbol: &str) -> bool {
        let account_active = self
            .accounts
            .get(account_id)
            .is_some_and(|a| a.status == crate::domain::AccountStatus::Active);
        account_active && self.securities.contains_key(symbol)
    }

    /// 日终 dump 回 JSON —— 与 `load` 对偶，让 mock 阶段能观察内核留下的痕迹。
    pub fn save(&self, data_root: impl AsRef<Path>) -> anyhow::Result<()> {
        let root = data_root.as_ref();
        std::fs::create_dir_all(root.join("state"))?;
        save_json(&root.join("state/asset.json"), &self.assets)?;
        save_json(&root.join("state/position.json"), &self.positions)?;
        Ok(())
    }
}

/// 复合主键的唯一构造点：手写 `"a:b"` 散落各处是事故源。
#[must_use]
pub fn position_key(account_id: &str, symbol: &str) -> String {
    let mut key = String::with_capacity(account_id.len() + 1 + symbol.len());
    key.push_str(account_id);
    key.push(':');
    key.push_str(symbol);
    key
}

/// 资金变动的一切入口都走这里，保证 `available + frozen` 守恒。
/// 返回 `false` 表示可用资金不足，调用方必须拒单 —— 不允许负余额出现。
pub fn try_freeze(asset: &mut Asset, amount: Money) -> bool {
    let Some(frozen) = asset.frozen.checked_add(amount) else {
        return false;
    };
    let Some(available) = asset.available.checked_sub(amount) else {
        return false;
    };
    asset.frozen = frozen;
    asset.available = available;
    true
}

/// 冻结转已用（成交扣款）：冻结金额减少，成交额离开账户体系。
pub fn settle_frozen_out(asset: &mut Asset, amount: Money) -> bool {
    let Some(frozen) = asset.frozen.checked_sub(amount) else {
        return false;
    };
    asset.frozen = frozen;
    true
}

/// 解冻回可用（撤单/废单）。
pub fn unfreeze(asset: &mut Asset, amount: Money) -> bool {
    let Some(available) = asset.available.checked_add(amount) else {
        return false;
    };
    let Some(frozen) = asset.frozen.checked_sub(amount) else {
        return false;
    };
    asset.available = available;
    asset.frozen = frozen;
    true
}

fn load_json<T: serde::de::DeserializeOwned>(path: &Path) -> anyhow::Result<T> {
    let raw = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

fn save_json<T: serde::Serialize>(path: &Path, value: &T) -> anyhow::Result<()> {
    std::fs::write(path, serde_json::to_string_pretty(value)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn snapshot() -> Snapshot {
        Snapshot::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("data")).unwrap()
    }

    #[test]
    fn loads_all_mock_json_into_memory() {
        let snap = snapshot();
        assert_eq!(snap.users.len(), 3);
        assert_eq!(snap.accounts.len(), 3);
        assert_eq!(snap.securities.len(), 2);
        assert_eq!(snap.positions.len(), 2);
        assert!(snap.check_integrity().is_ok());
    }

    #[test]
    fn tradability_respects_account_and_dict() {
        let snap = snapshot();
        assert!(snap.is_tradable("A001", "09018"));
        // A003 账户被冻结
        assert!(!snap.is_tradable("A003", "09018"));
        // 未知证券
        assert!(!snap.is_tradable("A001", "999999"));
        // 未知账户
        assert!(!snap.is_tradable("A404", "09018"));
    }

    #[test]
    fn freeze_conserves_available_plus_frozen() {
        let snap = snapshot();
        let mut asset = snap.assets["A001"].clone();
        let before = asset.available.checked_add(asset.frozen).unwrap();

        // 冻结 7202.00 元
        assert!(try_freeze(&mut asset, Money::from_units(720_200)));
        assert_eq!(asset.available.checked_add(asset.frozen).unwrap(), before);

        // 超额冻结必须被拒绝，且原值不变
        let untouched = asset;
        assert!(!try_freeze(&mut untouched.clone(), Money::from_units(i64::MAX)));
        assert_eq!(untouched.available, snap.assets["A001"].available.checked_sub(Money::from_units(720_200)).unwrap());
    }

    #[test]
    fn position_key_is_stable_and_matches_json_layout() {
        assert_eq!(position_key("A001", "09018"), "A001:09018");
        let snap = snapshot();
        assert!(snap.position("A001", "09018").is_some());
        assert!(snap.position("A001", "600000").is_none());
    }
}