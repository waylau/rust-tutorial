use proc_macro::TokenStream;

#[proc_macro]
pub fn print_message(input: TokenStream) -> TokenStream {
    let message = input.to_string();
    let output = format!("println!(\"Rust! {}\");", message);
    output.parse().unwrap()
}
