// SPDX-License-Identifier: CC0-1.0

//! Provides a type and logic to executed Bitcoin scripts.
//!
//! This is currently just a POC of the interpreter design. Does not
//! have any transaction data so can only handle opcodes that do not
//! require the transaction i.e., arithmetic.

use anyhow::{bail, Result};
use bitcoin::opcodes::all::*;
use bitcoin::script::{ScriptBuf, ScriptExt, Instruction};

use crate::stack::Stack;

pub struct Interpreter {
    /// The stack used during script execution.
    stack: Stack,
    /// The script to interpret/execute.
    // TODO: This can be a `&Script` because we only need to call `instructions`.
    script: ScriptBuf,
}

impl Interpreter {
    /// Creates a new script interpreter.
    pub fn new(script: ScriptBuf) -> Self {
        Self {
            stack: Stack::new(),
            script,
        }
    }

    /// Checks if the script is valid.
    ///
    /// > A transaction is valid if nothing in the combined script
    /// > triggers failure and the top stack item is True (non-zero)
    /// > when the script exits.
    pub fn script_is_valid(&mut self) -> bool {
        match self.execute_script() {
            Ok(res) => res,
            Err(_) => false,
        }
    }

    /// Executes the current script.
    ///
    /// # Returns
    ///
    /// - `Ok(top_of_stack)` if script terminated successfully.
    /// - `Err()` if something in the script triggered failure.
    pub fn execute_script(&mut self) -> Result<bool> {
        for ins in self.script.clone().instructions() {
            match ins? {
                Instruction::PushBytes(ref p) => self.stack.push(p.as_bytes()),
                Instruction::Op(ref op) => {
                    match *op {
                        OP_RETURN => {
                            // OP_RETURN causes script to immediately fail.
                            bail!("OP_RETURN");
                        },
                        // Constants
                        OP_PUSHNUM_NEG1 => self.stack.push_num(-1),
                        OP_PUSHBYTES_0 => self.stack.push(&[]),
                        OP_PUSHNUM_1 => self.stack.push_num(1),
                        OP_PUSHNUM_2 => self.stack.push_num(2),
                        OP_PUSHNUM_3 => self.stack.push_num(3),
                        OP_PUSHNUM_4 => self.stack.push_num(4),
                        OP_PUSHNUM_5 => self.stack.push_num(5),
                        OP_PUSHNUM_6 => self.stack.push_num(6),
                        OP_PUSHNUM_7 => self.stack.push_num(7),
                        OP_PUSHNUM_8 => self.stack.push_num(8),
                        OP_PUSHNUM_9 => self.stack.push_num(9),
                        OP_PUSHNUM_10 => self.stack.push_num(10),
                        OP_PUSHNUM_11 => self.stack.push_num(11),
                        OP_PUSHNUM_12 => self.stack.push_num(12),
                        OP_PUSHNUM_13 => self.stack.push_num(13),
                        OP_PUSHNUM_14 => self.stack.push_num(14),
                        OP_PUSHNUM_15 => self.stack.push_num(15),
                        OP_PUSHNUM_16 => self.stack.push_num(16),
                        // Arithmetic
                        OP_ADD => self.add()?,
                        OP_SUB => todo!(),
                        // Bitwise logic
                        OP_EQUAL => self.equal()?,
                        other => panic!("{}", format!("opcode not yet supported: {}", other)),
                    }
                }
            }
        }
        Ok(self.stack.is_true())
    }

    /// Removes the top two stack items, adds them together, and
    /// pushes the result back onto the stack.
    ///
    /// # Returns
    ///
    /// Returns an error if there are not two numbers on the stack.
    fn add(&mut self) -> Result<()> {
        let a = self.stack.pop_num()?;
        let b = self.stack.pop_num()?;
        let res = a + b;
        Ok(self.stack.push_num(res))
    }

    fn equal(&mut self) -> Result<()> {
        let a = self.stack.pop_num()?;
        let b = self.stack.pop_num()?;
        let res = a == b;
        Ok(self.stack.push_bool(res))

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::script::ScriptExt;
    use bitcoin::Script;

    #[test]
    fn op_return_false_no_data() {
        let script = Script::builder().push_opcode(OP_RETURN).into_script();
        assert!(Interpreter::new(script.clone()).execute_script().is_err());
        assert!(!Interpreter::new(script).script_is_valid());
    }

    #[test]
    fn op_return_false_with_data() {
        let script = Script::builder()
            .push_opcode(OP_RETURN)
            .push_slice(&[0xab, 32]) // Just push some nonsense.
            .into_script();

        assert!(Interpreter::new(script.clone()).execute_script().is_err());
        assert!(!Interpreter::new(script).script_is_valid());
    }

    #[test]
    fn add() {
        let script = Script::builder()
            .push_int(5).unwrap()
            .push_int(2).unwrap()
            .push_int(3).unwrap()
            .push_opcode(OP_ADD)
            .push_opcode(OP_EQUAL)
            .into_script();

        assert!(Interpreter::new(script).script_is_valid());
    }
}

