pub mod app;
pub mod config;
pub mod domain;
pub mod protocol;
pub mod sys;

pub fn run() -> Result<(), app::error::DaemonError> {
    app::controller::run()
}
