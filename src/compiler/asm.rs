pub mod x86_64 {
    use crate::compiler::ast::Node;

    trait IntoX86_64Instructions {
        fn into_x86_64_string(&self) -> String;
        fn unref_to_rax(&self) -> String;
    }

    impl IntoX86_64Instructions for Node {
        fn into_x86_64_string(&self) -> String {
            match self {
                Node::Block { statements } => statements
                    .iter()
                    .map(|node| node.into_x86_64_string() + "\npop rax")
                    .collect::<Vec<_>>()
                    .join("\n"),
                Node::Integer { value } => format!("push {}", value),
                Node::LocalVariable {
                    identifier: _,
                    offset: _,
                } => format!(
                    "{}\n\
                     pop rax\n\
                     mov rax, [rax]\n\
                     push rax\n",
                    self.unref_to_rax(),
                ),
                Node::OperatorAdd { lhs, rhs } => format!(
                    "{}\n\
                     {}\n\
                     pop rdi\n\
                     pop rax\n\
                     add rax, rdi\n\
                     push rax",
                    lhs.into_x86_64_string(),
                    rhs.into_x86_64_string(),
                ),
                Node::OperatorSub { lhs, rhs } => format!(
                    "{}\n\
                     {}\n\
                     pop rdi\n\
                     pop rax\n\
                     sub rax, rdi\n\
                     push rax",
                    lhs.into_x86_64_string(),
                    rhs.into_x86_64_string(),
                ),
                Node::OperatorMul { lhs, rhs } => format!(
                    "{}\n\
                     {}\n\
                     pop rdi\n\
                     pop rax\n\
                     imul rax, rdi\n\
                     push rax",
                    lhs.into_x86_64_string(),
                    rhs.into_x86_64_string(),
                ),
                Node::OperatorDiv { lhs, rhs } => format!(
                    "{}\n\
                     {}\n\
                     pop rdi\n\
                     pop rax\n\
                     cqo\n\
                     idiv rax, rdi\n\
                     push rax",
                    lhs.into_x86_64_string(),
                    rhs.into_x86_64_string(),
                ),
                Node::OperatorLt { lhs, rhs } => format!(
                    "{}\n\
                     {}\n\
                     pop rdi\n\
                     pop rax\n\
                     cmp rax, rdi\n\
                     setl al\n\
                     movzb rax, al\n\
                     push rax",
                    lhs.into_x86_64_string(),
                    rhs.into_x86_64_string(),
                ),
                Node::OperatorLtEq { lhs, rhs } => format!(
                    "{}\n\
                     {}\n\
                     pop rdi\n\
                     pop rax\n\
                     cmp rax, rdi\n\
                     setle al\n\
                     movzb rax, al\n\
                     push rax",
                    lhs.into_x86_64_string(),
                    rhs.into_x86_64_string(),
                ),
                Node::OperatorEq { lhs, rhs } => format!(
                    "{}\n\
                     {}\n\
                     pop rdi\n\
                     pop rax\n\
                     cmp rax, rdi\n\
                     sete al\n\
                     movzb rax, al\n\
                     push rax",
                    lhs.into_x86_64_string(),
                    rhs.into_x86_64_string(),
                ),
                Node::OperatorNe { lhs, rhs } => format!(
                    "{}\n\
                     {}\n\
                     pop rdi\n\
                     pop rax\n\
                     cmp rax, rdi\n\
                     setne al\n\
                     movzb rax, al\n\
                     push rax",
                    lhs.into_x86_64_string(),
                    rhs.into_x86_64_string(),
                ),
                Node::OperatorAssign { lhs, rhs } => format!(
                    "{}\n\
                     {}\n\
                     pop rdi\n\
                     pop rax\n\
                     mov [rax], rdi\n\
                     push rdi",
                    lhs.unref_to_rax(),
                    rhs.into_x86_64_string(),
                ),
            }
        }

        fn unref_to_rax(&self) -> String {
            match self {
                Node::LocalVariable {
                    identifier: _,
                    offset,
                } => format!(
                    "mov rax, rbp\n\
                     sub rax, {}\n\
                     push rax",
                    offset
                ),
                _ => panic!("Unexpected node for left value"),
            }
        }
    }

    pub fn into_asm_string(node: &Node) -> String {
        format!(
            ".intel_syntax noprefix
.global main
main:
    push rbp
    mov rbp, rsp
    sub rsp, 208
    {}
    mov rsp, rbp
    pop rbp
    ret",
            (node.into_x86_64_string()).replace("\n", "\n    ")
        )
    }
}
