use super::error::Result;
use super::parser::{Node, Parser};

trait IntoX86_64String {
    fn into_x86_64_string(&self) -> String;
}

impl IntoX86_64String for Node {
    fn into_x86_64_string(&self) -> String {
        match self {
            Node::Integer { value } => format!("push {}", value),
            Node::OperatorAdd { lhs, rhs } => {
                format!(
                    "{}\n\
                     {}\n\
                     pop rdi\n\
                     pop rax\n\
                     add rax, rdi\n\
                     push rax",
                    lhs.into_x86_64_string(),
                    rhs.into_x86_64_string(),
                )
            }
            Node::OperatorSub { lhs, rhs } => {
                format!(
                    "{}\n\
                     {}\n\
                     pop rdi\n\
                     pop rax\n\
                     sub rax, rdi\n\
                     push rax",
                    lhs.into_x86_64_string(),
                    rhs.into_x86_64_string(),
                )
            }
            Node::OperatorMul { lhs, rhs } => {
                format!(
                    "{}\n\
                     {}\n\
                     pop rdi\n\
                     pop rax\n\
                     imul rax, rdi\n\
                     push rax",
                    lhs.into_x86_64_string(),
                    rhs.into_x86_64_string(),
                )
            }
            Node::OperatorDiv { lhs, rhs } => {
                format!(
                    "{}\n\
                     {}\n\
                     pop rdi\n\
                     pop rax\n\
                     cqo\n\
                     idiv rax, rdi\n\
                     push rax",
                    lhs.into_x86_64_string(),
                    rhs.into_x86_64_string(),
                )
            }
        }
    }
}

pub fn compile(text: &str) -> Result<String> {
    let text = text.trim();
    let node = Parser::new(text).parse()?;

    let result = format!(
        ".intel_syntax noprefix
.global main
main:
    {}
    pop rax
    ret",
        (node.into_x86_64_string()).replace("\n", "    \n")
    );

    Ok(result)
}
