use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingFreeze {
    pub package_name: String,
    pub uid: u32,
    pub due_at_ms: u128,
}

#[derive(Debug, Default)]
pub struct FreezeScheduler {
    pending: BTreeMap<(String, u32), PendingFreeze>,
}

impl FreezeScheduler {
    pub fn schedule_background(
        &mut self,
        package_name: impl Into<String>,
        uid: u32,
        now_ms: u128,
        delay_ms: u64,
    ) {
        let package_name = package_name.into();
        self.pending.insert(
            (package_name.clone(), uid),
            PendingFreeze {
                package_name,
                uid,
                due_at_ms: now_ms + u128::from(delay_ms),
            },
        );
    }

    pub fn cancel_foreground(&mut self, package_name: &str, uid: u32) -> Option<PendingFreeze> {
        self.pending.remove(&(package_name.to_owned(), uid))
    }

    pub fn due_at(&self, now_ms: u128) -> Vec<PendingFreeze> {
        self.pending
            .values()
            .filter(|pending| pending.due_at_ms <= now_ms)
            .cloned()
            .collect()
    }
}
