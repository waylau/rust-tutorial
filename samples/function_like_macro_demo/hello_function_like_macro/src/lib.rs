use proc_macro::TokenStream;

#[proc_macro]
pub fn print_hello(_input: TokenStream) -> TokenStream {
    let output = "println!(\"hi, macro!\");";
    output.parse().unwrap()
}

#[proc_macro]
pub fn print_message(input: TokenStream) -> TokenStream {
    let message = input.to_string();
    let output = format!("println!(\"{}!\");", message);
    output.parse().unwrap()
}
