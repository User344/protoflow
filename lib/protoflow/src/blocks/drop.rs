// This is free and unencumbered software released into the public domain.

use protoflow::derive::Block;
use protoflow::{Block, BlockResult, BlockRuntime, InputPort, Message};

/// A block that simply discards all messages it receives.
#[derive(Block, Clone)]
pub struct Drop<T: Message> {
    #[input]
    pub input: InputPort<T>,
}

impl<T: Message> Drop<T> {
    pub fn new(input: InputPort<T>) -> Self {
        Self { input }
    }
}

impl<T: Message> Block for Drop<T> {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            drop(message);
            self.input.close()?;
        }
        self.input.close()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Drop;
    use crate::{transports::MockTransport, System};

    #[test]
    fn instantiate_drop_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(Drop::<i32>::new(s.input()));
        });
    }
}
