// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
pub mod sys {
    pub trait SysBlocks {}
    pub enum SysBlockConfig {}
}

#[cfg(feature = "std")]
pub mod sys {
    use super::{
        prelude::{vec, Box, Cow, Named, String, Vec},
        types::ByteSize,
        BlockConnections, BlockInstantiation, InputPortName, OutputPortName, System,
    };
    use protoflow_core::Block;

    pub trait SysBlocks {
        fn read_dir(&mut self) -> ReadDir;
        fn read_env(&mut self) -> ReadEnv;
        fn read_file(&mut self) -> ReadFile;
        fn read_socket(&mut self) -> ReadSocket;
        fn read_stdin(&mut self) -> ReadStdin;
        fn write_file(&mut self) -> WriteFile;
        fn write_socket(&mut self) -> WriteSocket;
        fn write_stderr(&mut self) -> WriteStderr;
        fn write_stdout(&mut self) -> WriteStdout;
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum SysBlockTag {
        ReadDir,
        ReadEnv,
        ReadFile,
        ReadSocket,
        ReadStdin,
        WriteFile,
        WriteSocket,
        WriteStderr,
        WriteStdout,
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Clone, Debug)]
    pub enum SysBlockConfig {
        ReadDir {
            path: InputPortName,
            output: OutputPortName,
        },

        ReadEnv {
            name: InputPortName,
            output: OutputPortName,
        },

        ReadFile {
            path: InputPortName,
            output: OutputPortName,
        },

        ReadSocket {
            output: OutputPortName,
            config: ReadSocketConfig,
        },

        ReadStdin {
            output: OutputPortName,
            buffer_size: Option<ByteSize>,
        },

        WriteFile {
            path: InputPortName,
            input: InputPortName,
            flags: Option<WriteFlags>,
        },

        WriteSocket {
            input: InputPortName,
            config: WriteSocketConfig,
        },

        WriteStderr {
            input: InputPortName,
        },

        WriteStdout {
            input: InputPortName,
        },
    }

    impl Named for SysBlockConfig {
        fn name(&self) -> Cow<str> {
            use SysBlockConfig::*;
            Cow::Borrowed(match self {
                ReadDir { .. } => "ReadDir",
                ReadEnv { .. } => "ReadEnv",
                ReadFile { .. } => "ReadFile",
                ReadSocket { .. } => "ReadSocket",
                ReadStdin { .. } => "ReadStdin",
                WriteFile { .. } => "WriteFile",
                WriteSocket { .. } => "WriteSocket",
                WriteStderr { .. } => "WriteStderr",
                WriteStdout { .. } => "WriteStdout",
            })
        }
    }

    impl BlockConnections for SysBlockConfig {
        fn output_connections(&self) -> Vec<(&'static str, Option<OutputPortName>)> {
            use SysBlockConfig::*;
            match self {
                ReadDir { output, .. }
                | ReadEnv { output, .. }
                | ReadFile { output, .. }
                | ReadSocket { output, .. }
                | ReadStdin { output, .. } => {
                    vec![("output", Some(output.clone()))]
                }
                WriteFile { .. } | WriteSocket { .. } | WriteStderr { .. } | WriteStdout { .. } => {
                    vec![]
                }
            }
        }
    }

    impl BlockInstantiation for SysBlockConfig {
        fn instantiate(&self, system: &mut System) -> Box<dyn Block> {
            use SysBlockConfig::*;
            match self {
                ReadDir { .. } => Box::new(super::ReadDir::with_system(system)),
                ReadEnv { .. } => Box::new(super::ReadEnv::<String>::with_system(system)),
                ReadFile { .. } => Box::new(super::ReadFile::with_system(system)),
                ReadSocket { config, .. } => {
                    Box::new(super::ReadSocket::with_system(system, Some(config.clone())))
                }
                ReadStdin { buffer_size, .. } => {
                    Box::new(super::ReadStdin::with_system(system, *buffer_size))
                }
                WriteFile { flags, .. } => Box::new(super::WriteFile::with_system(system, *flags)),
                WriteSocket { config, .. } => Box::new(super::WriteSocket::with_system(
                    system,
                    Some(config.clone()),
                )),
                WriteStderr { .. } => Box::new(super::WriteStderr::with_system(system)),
                WriteStdout { .. } => Box::new(super::WriteStdout::with_system(system)),
            }
        }
    }

    mod read_dir;
    pub use read_dir::*;

    mod read_env;
    pub use read_env::*;

    mod read_file;
    pub use read_file::*;

    mod read_socket;
    pub use read_socket::*;

    mod read_stdin;
    pub use read_stdin::*;

    mod write_file;
    pub use write_file::*;

    mod write_socket;
    pub use write_socket::*;

    mod write_stderr;
    pub use write_stderr::*;

    mod write_stdout;
    pub use write_stdout::*;
}

pub use sys::*;
