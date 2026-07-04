#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedApp {
    pub package_name: String,
    pub user_id: u32,
    pub uid: u32,
    pub label: String,
    pub is_system_app: bool,
    pub protected_reason: Option<ProtectedReason>,
    pub policy_id: String,
    pub last_seen_baseline: String,
}

impl ManagedApp {
    pub fn policy_identity(&self) -> (&str, u32) {
        (&self.package_name, self.user_id)
    }

    pub fn is_protected(&self) -> bool {
        self.protected_reason.is_some()
    }

    pub fn apply_protected_defaults(&mut self) {
        if self.protected_reason.is_some() || self.is_system_app {
            self.protected_reason = self
                .protected_reason
                .or(Some(ProtectedReason::SystemCritical));
            self.policy_id = format!("protected:{}", self.package_name);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectedReason {
    Manager,
    Launcher,
    InputMethod,
    RootManager,
    HookManager,
    SystemCritical,
    UserProtected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreezeMode {
    Protected,
    Free,
    Freeze,
    FreezeWithRestrictions,
    Terminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForegroundStrategy {
    Strict,
    Permissive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FallbackAction {
    Postpone,
    AlternateFreezer,
    Signal,
    Terminate,
    Skip,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FreezePolicy {
    Selected {
        mode: FreezeMode,
        delay_ms: u64,
        foreground_strategy: ForegroundStrategy,
        allow_network_restriction: bool,
        allow_wakelock_restriction: bool,
        fallback_strategy: Vec<FallbackAction>,
        updated_at_ms: u128,
    },
}

impl FreezePolicy {
    pub fn protected_default() -> Self {
        Self::Selected {
            mode: FreezeMode::Protected,
            delay_ms: 0,
            foreground_strategy: ForegroundStrategy::Strict,
            allow_network_restriction: false,
            allow_wakelock_restriction: false,
            fallback_strategy: vec![FallbackAction::Skip],
            updated_at_ms: 0,
        }
    }

    pub fn is_control_allowed_for(&self, app: &ManagedApp) -> bool {
        if app.is_protected() {
            return false;
        }

        matches!(
            self,
            Self::Selected {
                mode: FreezeMode::Freeze
                    | FreezeMode::FreezeWithRestrictions
                    | FreezeMode::Terminate,
                ..
            }
        )
    }
}
