use super::asm::x86_64;
use super::error::Result;
use super::parser::Parser;

pub fn compile(text: &str) -> Result<String> {
    let text = text.trim();
    let node = Parser::new(text).parse()?;
    let result = x86_64::into_asm_string(&node);

    Ok(result)
}
