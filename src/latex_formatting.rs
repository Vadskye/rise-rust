pub fn latexify(text: String) -> String {
    return text.replace("<", "{").replace(">", "}")
}
