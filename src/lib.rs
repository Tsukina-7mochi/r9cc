pub fn compile(text: String) -> String {
    let text = text.trim();
    let mut result = String::new();

    result.push_str(".intel_syntax noprefix\n");
    result.push_str(".global main\n");
    result.push_str("main:\n");
    result.push_str(&format!("   mov rax, {}\n", text.parse::<i32>().unwrap()));
    result.push_str("ret\n");

    result
}
