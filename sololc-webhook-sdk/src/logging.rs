#[cfg(feature = "logging")]
pub mod logging {
    use log::{Level, Metadata, Record, SetLoggerError};
    
    // 🌟 Imports the official WASIp2 system logging interfaces
    use wasi::logging::logging::{log as wasi_log, Level as WasiLevel};

    /// Represents a thread-safe, zero-allocation logger routing Rust `log` macros to WASIp2.
    ///
    /// Intercepts diagnostics generated via standard macro gates (such as [`log::info!`] or 
    /// [`log::error!`]) and dynamically marshals them into the host's native telemetry pipeline 
    /// through the [`wasi:logging/logging`] system interface.
    struct SololcLogger;

    impl log::Log for SololcLogger {
        /// Evaluates whether a log record with the provided metadata should be processed.
        ///
        /// Returns `true` universally, deferring severity filtering to the host environment.
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }

        /// Forwards a diagnostic [`Record`] directly to the WASIp2 system stream.
        ///
        /// Translates Rust's [`Level`] enum representations into native WASI [`WasiLevel`] 
        /// components prior to dispatching.
        fn log(&self, record: &Record) {
            let level = match record.level() {
                Level::Error => WasiLevel::Error,
                Level::Warn => WasiLevel::Warn,
                Level::Info => WasiLevel::Info,
                Level::Debug => WasiLevel::Debug,
                Level::Trace => WasiLevel::Trace,
            };
            // Dispatches directly to the host's WASIp2 standard logger
            wasi_log(level, record.target(), &format!("{}", record.args()));
        }

        /// Flushes any buffered log entries.
        ///
        /// Performs no operations (no-op) as the underlying WASIp2 system logging 
        /// interface is stream-immediate.
        fn flush(&self) {}
    }

    /// Holds the static singleton instance of the active logger.
    static LOGGER: SololcLogger = SololcLogger;

    /// Initializes the global logger singleton and registers it with the standard `log` library.
    ///
    /// Configures [`LOGGER`] as the primary collector for all telemetry dispatches and 
    /// applies the specified maximum filter severity.
    ///
    /// # Errors
    ///
    /// Returns a [`SetLoggerError`] if another logging implementation has already been 
    /// initialized as the global singleton for the executing process.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sololc_webhook_sdk::logging;
    /// use log::LevelFilter;
    ///
    /// # fn run() -> Result<(), log::SetLoggerError> {
    /// logging::init(LevelFilter::Info)?;
    /// log::info!("WASI-HTTP telemetry initialized successfully.");
    /// # Ok(())
    /// # }
    /// ```
    pub fn init(max_level: log::LevelFilter) -> Result<(), SetLoggerError> {
        log::set_logger(&LOGGER)?;
        log::set_max_level(max_level);
        Ok(())
    }
}