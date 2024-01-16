pub mod token;

// this is mainly for defining our tokens and shouldn't be used anywhere outside the tokenizer and
// lexer modules since it's leaky
pub fn char_to_str(input: char) -> &'static str {
    Box::leak(input.to_string().into_boxed_str())
}
