mod parse;
mod render;

use proc_macro::TokenStream;
use render::render;

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let mut ast = syn::parse_macro_input!(input);

    let debug_def = parse::DebugDef::try_new_from(&mut ast);

    render(debug_def)
}
