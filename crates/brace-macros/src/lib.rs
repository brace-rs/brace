use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_str};

use brace_hook_gen::args::Args;
use brace_hook_gen::{declaration, registration};

#[proc_macro_attribute]
pub fn hook(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);
    let krate = match parse_str("brace") {
        Ok(path) => path,
        Err(err) => return TokenStream::from(err.to_compile_error()),
    };

    TokenStream::from(match args {
        Args::None => declaration::expand(krate, parse_macro_input!(input)),
        Args::With(path, weight) => {
            registration::expand(krate, path, weight, parse_macro_input!(input))
        }
    })
}
