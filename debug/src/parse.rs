use helper::take_first_matched_attribute_from_list;
use quote::spanned::Spanned;

mod keyword {
    syn::custom_keyword!(debug);
}

pub(crate) struct DebugDef {
    name: syn::Ident,
    fields: Vec<DebugField>,
}

impl DebugDef {
    pub fn get_name(&self) -> &syn::Ident {
        &self.name
    }
    pub fn fields(&self) -> &Vec<DebugField> {
        &self.fields
    }
}

#[derive(Debug)]
pub(crate) struct DebugField {
    pub _f_span: proc_macro2::Span,
    pub f_name: syn::Ident,
    // pub f_type: syn::Type,
    // pub f_is_optional: bool,
    // in case if either it's option or it's each vec
    // pub f_inner_type: Option<syn::Ident>,
    pub debug_fmt: Option<syn::Lit>,
}

impl DebugDef {
    pub fn try_new_from(derive_input: &mut syn::DeriveInput) -> syn::Result<Self> {
        // check if the target is struct or else return with valid error
        let target_struct = if let syn::Data::Struct(ref mut target_struct) = derive_input.data {
            target_struct
        } else {
            return Err(syn::Error::new_spanned(
                derive_input,
                "`CustomDerive` attribute can only be applied on struct only",
            ));
        };

        // check if it contains named fields only
        let named_struct_fields = if let syn::Fields::Named(syn::FieldsNamed {
            brace_token: _,
            ref mut named,
        }) = target_struct.fields
        {
            named
        } else {
            return Err(syn::Error::new_spanned(
                derive_input,
                "`CustomDerive` attribute can only be applied on struct with all named fields.",
            ));
        };

        let mut debug_fields = vec![];

        for named_field in named_struct_fields.iter_mut() {
            let f_span = &named_field.__span();
            // let f_is_optional = is_option(&named_field.ty);
            // let mut f_inner_type = None;
            // let f_type = &named_field.ty;
            let f_name = &named_field.ident.clone().ok_or_else(|| {
                syn::Error::new_spanned(named_field.clone(), "Must have an Identifier")
            })?;

            // let mut is_each = false;
            let mut debug_fmt = None;
            let attr: Option<DebugAttr> =
                take_first_matched_attribute_from_list(&mut named_field.attrs, "debug")?;

            if let Some(DebugAttr::Debug(_, df)) = attr {
                debug_fmt = Some(df)
            }

            debug_fields.push(DebugField {
                _f_span: f_span.clone(),
                f_name: f_name.clone(),
                debug_fmt,
            });
        }

        // replace me with original type
        Ok(DebugDef {
            name: derive_input.ident.clone(),
            fields: debug_fields,
        })
    }
}

/// Allowed Debug attribute
#[derive(Debug)]
enum DebugAttr {
    Debug(proc_macro2::Span, syn::Lit),
}

impl syn::parse::Parse for DebugAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![#]>()?;
        let content;
        syn::bracketed!(content in input);

        let debug_span = content.parse::<keyword::debug>()?.span;
        content.parse::<syn::Token![=]>()?;
        let string_literal = content.parse::<syn::Lit>()?;
        Ok(DebugAttr::Debug(debug_span, string_literal))
    }
}
