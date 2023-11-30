pub fn compile(text: String) -> String {
    let text = text.trim();

    format!(
        "\
.intel_syntax noprefix
.global main
main:
    mov rax, {}
    ret",
        text.parse::<i32>().unwrap()
    )
}
