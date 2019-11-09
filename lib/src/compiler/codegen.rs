use crate::{ast::WellFormedProgram, compiler::Compiler};
use failure::Error;
use std::io::Write;

const PROLOGUE: &str = "\
section .text
global entrypoint

entrypoint:
    mov EAX, 10
";

impl Compiler<WellFormedProgram> {
    /// Generates code for the target language, and writes it to the given
    /// output destination. This is the final compiler step.
    pub fn generate_code(self, output: &mut impl Write) -> Result<(), Error> {
        output.write_all(PROLOGUE.as_bytes())?;
        Ok(())
    }
}
