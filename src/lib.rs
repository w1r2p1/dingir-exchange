#![allow(dead_code)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::let_and_return)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::single_char_pattern)]
#![allow(clippy::await_holding_refcell_ref)] // FIXME

pub mod matchengine;
pub use matchengine::{asset, controller, dto, history, market, persist, sequencer, server};
pub mod storage;
pub use storage::{database, models, sqlxextend};
pub mod config;
pub mod message;
pub mod restapi;
pub mod types;
pub mod utils;
