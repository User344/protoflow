// This is free and unencumbered software released into the public domain.

#![no_std]

pub mod prelude; // FIXME

pub use prost::Message;

mod block;
pub use block::*;

mod block_descriptor;
pub use block_descriptor::*;

mod block_error;
pub use block_error::*;

pub mod blocks;

/// Derive macros are available if the crate was built with a
/// `features = ["derive"]` configuration.
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub mod derive {
    pub use protoflow_derive::*;
}

mod feature;
pub use feature::*;

mod function_block;
pub use function_block::*;

mod input_port;
pub use input_port::*;

mod output_port;
pub use output_port::*;

mod port;
pub use port::*;

mod port_descriptor;
pub use port_descriptor::*;

mod port_error;
pub use port_error::*;

mod port_state;
pub use port_state::*;

mod runtime;
pub use runtime::*;

pub mod runtimes;

mod scheduler;
pub use scheduler::Scheduler;

mod system;
pub use system::*;

mod transport;
//pub use transport::*;

pub mod transports;

pub mod types {
    pub use prost_types::*;
}
