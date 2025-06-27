// SPDX-License-Identifier: CC0-1.0

use anyhow::{anyhow, Result};
use bitcoin::script;

/// The stack used during script execution.
pub struct Stack {
    // FIXME: I can't work out how to separate items on the stack
    // without using a matrix e.g., how does one read a scriptint from
    // a long string of bytes?
    items: Vec<Vec<u8>>,
}

impl Stack {
    /// Creates a new empty stack.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    /// Pushes bytes onto the stack.
    pub fn push(&mut self, bytes: &[u8]) {
        self.items.push(bytes.to_vec());
    }

    /// Pushes number onto the stack.
    pub fn push_num(&mut self, x: i64) {
        let mut buf = [0_u8; 8];
        let written = script::write_scriptint(&mut buf, x);
        self.push(&buf[0..written]);
    }

    /// Pushes boolean onto the stack.
    pub fn push_bool(&mut self, x: bool) {
        if x {
            self.push(&[0x01]) 
        } else {
            self.push(&[])
        }
    }

    /// Pops the top item from the stack.
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<Vec<u8>> {
        self.items.pop()
    }

    /// Pops the top item from the stack returning it if it is a scriptint.
    ///
    /// # Returns
    ///
    /// Returns an error if stack is empty or scriptint parsing fails.
    pub fn pop_num(&mut self) -> Result<i64> {
        let item = self.items.pop().ok_or_else(|| anyhow!("called pop on an empty stack"))?;
        Ok(script::read_scriptint_non_minimal(&item)?)
    }

    /// Returns the number of items on the stack.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the stack is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Peeks at the top item without removing it.
    pub fn top(&self) -> Option<&Vec<u8>> {
        self.items.last()
    }

    /// Returns `true` if stack is non-empty and top item is non-zero.
    ///
    /// This is the definition of valid for a Bitcoin script after execution.
    pub fn is_true(&self) -> bool {
        match self.top() {
            Some(top) => script::read_scriptbool(&top),
            None => false,
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
