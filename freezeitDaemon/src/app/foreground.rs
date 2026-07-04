use crate::domain::{policy::ForegroundStrategy, runtime::ProcessState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ForegroundSignals {
    pub has_visible_ui: bool,
    pub has_foreground_service: bool,
    pub has_overlay: bool,
    pub has_audio: bool,
}

pub fn classify_foreground(
    proc_state: ProcessState,
    strategy: ForegroundStrategy,
    signals: ForegroundSignals,
) -> bool {
    match proc_state {
        ProcessState::Foreground | ProcessState::Visible => true,
        ProcessState::Unknown => strategy == ForegroundStrategy::Strict,
        ProcessState::Service | ProcessState::Cached | ProcessState::Empty => {
            strategy == ForegroundStrategy::Permissive
                && (signals.has_visible_ui
                    || signals.has_foreground_service
                    || signals.has_overlay
                    || signals.has_audio)
        }
    }
}
