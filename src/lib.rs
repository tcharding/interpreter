// SPDX-License-Identifier: CC0-1.0

//! # Bitcoin Script Interpreter

pub mod interpreter;
mod stack;

use anyhow::Result;
use bitcoin::ScriptBuf;

use crate::interpreter::Interpreter;

/// Executes the script `script_sig |  script_pubkey`.
///
/// A Bitcoin script is valid if after execution the stack is non-empty and non-zero.
///
/// # Returns
///
/// - `Ok(top_of_stack)` if script terminated successfully.
/// - `Err()` if something in the script triggered failure.
pub fn execute(script_sig: ScriptBuf, script_pubkey: ScriptBuf) -> Result<bool> {
    let script = join_parts(script_sig, script_pubkey);
    let mut interpreter = Interpreter::new(script);
    interpreter.execute_script()
}

/// Checks if the script `script_sig |  script_pubkey` is valid.
///
/// A Bitcoin script is valid if after execution the stack is non-empty and non-zero.
pub fn is_valid(script_sig: ScriptBuf, script_pubkey: ScriptBuf) -> bool {
    let script = join_parts(script_sig, script_pubkey);
    let mut interpreter = Interpreter::new(script);
    interpreter.script_is_valid()
}

/// Joins the script sig and script pubkey together into a single
/// script ready for execution.
fn join_parts(script_sig: ScriptBuf, script_pubkey: ScriptBuf) -> ScriptBuf {
    ScriptBuf::from_bytes(script_sig.into_bytes().into_iter().chain(script_pubkey.into_bytes()).collect())
}
