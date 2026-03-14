mod app_new;
mod pages;
mod widgets;
use app_new as app;

use tracing_subscriber::EnvFilter;
use std::io::Write;

/// Synchronous file writer for tracing - logs reach disk even when
/// launched via `open App.app` where stdout/stderr are /dev/null
struct SyncFileWriter(std::sync::Mutex<std::fs::File>);
impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for SyncFileWriter {
    type Writer = SyncGuardWriter<'a>;
    fn make_writer(&'a self) -> Self::Writer {
        SyncGuardWriter(self.0.lock().unwrap())
    }
}
struct SyncGuardWriter<'a>(std::sync::MutexGuard<'a, std::fs::File>);
impl<'a> Write for SyncGuardWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { self.0.write(buf) }
    fn flush(&mut self) -> std::io::Result<()> { self.0.flush() }
}

fn main() -> cosmic::iced::Result {
    // Write logs to /tmp/clawmaster.log (absolute path, sync)
    let log_file = std::fs::OpenOptions::new()
        .create(true).append(true)
        .open("/tmp/clawmaster.log")
        .expect("open /tmp/clawmaster.log");
    let writer = SyncFileWriter(std::sync::Mutex::new(log_file));
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("clawmaster_cosmic=info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(writer)
        .with_ansi(false)
        .init();

    let settings = cosmic::app::Settings::default()
        .size(cosmic::iced::Size::new(1400.0, 900.0))
        .size_limits(
            cosmic::iced::core::layout::Limits::NONE
                .min_width(1200.0)
                .min_height(700.0)
                .max_width(2000.0)
                .max_height(1400.0),
        )
        .resizable(Some(8.0))  // 圆角半径 8px
        .debug(false);

    cosmic::app::run::<app::ClawMasterApp>(settings, ())
}
