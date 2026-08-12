//! qconnect-app
//!
//! Application adapter that composes qconnect core + protocol + transport.

mod app;
mod error;
mod events;
mod state;

pub use app::QconnectApp;
pub use error::QconnectAppError;
pub use events::{NoOpEventSink, QconnectAppEvent, QconnectEventSink};
pub use qconnect_core::{
    evaluate_remote_queue_admission, resolve_handoff_intent, AdmissionDecision, HandoffIntent,
    QConnectQueueState, QConnectRendererState, QueueVersion, RendererCommand, TrackOrigin,
};
pub use qconnect_protocol::{QueueCommandType, RendererReport, RendererReportType};
pub use state::QconnectRuntimeState;
