use crate::ast::Program;
use failure::Fallible;
use nom::IResult;
use std::io::Read;

fn do_parse(input: &str) -> IResult<&str, Program> {
    // Ok((input, Program { body: vec![] }))
    match serde_json::from_str(&input) {
        Ok(program) => Ok((input, program)),
        _ => panic!("ass2"),
    }
}
impl Program {
    /// Parses source code from the given input, into an abstract syntax tree.
    pub fn parse(source: &mut impl Read) -> Fallible<Self> {
        let mut source_buffer = String::new();
        source.read_to_string(&mut source_buffer)?;
        match do_parse(&source_buffer) {
            Ok((_, program)) => Ok(program),
            _ => panic!("ass"),
        }
    }
}
