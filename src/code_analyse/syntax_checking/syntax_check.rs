fn check_syntax(code: String) {
    let symbols = code.split(" ");
    for (index, symbol) in symbols.iter().enumerate() {
        if CIQ_SYNTAX_KEYWORDS.iter().rposition(|&keyword| keyword == symbol) == Some() {
            println!("Keyword")
        }
    }
}
