use proc_macro::TokenStream;
use quote::quote;

use crate::parse::DebugDef;

struct StringWrapper(String);
impl core::fmt::Debug for StringWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", &self.0).as_str())
    }
}

pub(crate) fn render(debug_def: syn::Result<DebugDef>) -> TokenStream {
    let def = match debug_def {
        Ok(ref def) => def,
        Err(e) => return e.to_compile_error().into(),
    };

    // get all field idents
    let fields = def.fields().iter().map(|f| {
        let field_name = &f.f_name;
        let field_name_str = field_name.to_string();

        if let Some(syn::Lit::Str(debug_fmt)) = &f.debug_fmt {
            quote! { .field(#field_name_str, &StringWrapper(&std::format!(#debug_fmt, &self.#field_name)))}
        } else {
            quote! { .field(#field_name_str, &self.#field_name)}
        }
    });

    let target = def.get_name();
    let target_lit = target.to_string();

    // following is final token stream which will be rendered
    quote!(
        impl core::fmt::Debug for #target {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(#target_lit)
                #(#fields)*
                .finish()
            }
        }
    )
    .into()
}
