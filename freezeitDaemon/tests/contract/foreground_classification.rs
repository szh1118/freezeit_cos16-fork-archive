use freezeit_daemon::{
    app::foreground::{classify_foreground, ForegroundSignals},
    domain::{policy::ForegroundStrategy, runtime::ProcessState},
};

#[test]
fn strict_foreground_requires_visible_or_unknown_safe_state() {
    assert!(classify_foreground(
        ProcessState::Foreground,
        ForegroundStrategy::Strict,
        ForegroundSignals::default()
    ));
    assert!(classify_foreground(
        ProcessState::Unknown,
        ForegroundStrategy::Strict,
        ForegroundSignals::default()
    ));
    assert!(!classify_foreground(
        ProcessState::Cached,
        ForegroundStrategy::Strict,
        ForegroundSignals {
            has_audio: true,
            ..ForegroundSignals::default()
        }
    ));
}

#[test]
fn permissive_foreground_treats_service_overlay_and_audio_as_visible() {
    assert!(classify_foreground(
        ProcessState::Service,
        ForegroundStrategy::Permissive,
        ForegroundSignals {
            has_foreground_service: true,
            ..ForegroundSignals::default()
        }
    ));
    assert!(classify_foreground(
        ProcessState::Cached,
        ForegroundStrategy::Permissive,
        ForegroundSignals {
            has_overlay: true,
            ..ForegroundSignals::default()
        }
    ));
    assert!(classify_foreground(
        ProcessState::Cached,
        ForegroundStrategy::Permissive,
        ForegroundSignals {
            has_audio: true,
            ..ForegroundSignals::default()
        }
    ));
}
