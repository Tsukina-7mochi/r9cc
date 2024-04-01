use crate::compiler::asm::x86_64;
use crate::compiler::error::Result;
use crate::compiler::parser::Parser;

pub fn compile(text: &str) -> Result<String> {
    let text = text.trim();
    let node = Parser::new(text).parse()?;
    let result = x86_64::into_asm_string(&node);

    Ok(result)
}
