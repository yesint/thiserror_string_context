extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream}, parse_macro_input, ItemEnum, LitStr, Variant
};

struct ContextAttr {
    message: Option<LitStr>,
}


impl Parse for ContextAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let message: Option<LitStr> = if input.is_empty() {
            None
        } else {
            Some(input.parse()?)
        };
        Ok(ContextAttr { message })
    }
}

#[proc_macro_attribute]
pub fn string_context(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the custom message passed to the macro
    let context_attr = parse_macro_input!(attr as ContextAttr);
    let custom_message = context_attr
        .message
        .unwrap_or_else(|| LitStr::new("{0}", proc_macro2::Span::call_site()));


    // Parse the input enum
    let input_enum = parse_macro_input!(item as ItemEnum);
    let enum_name = &input_enum.ident;
    let visibility = &input_enum.vis; // Get the visibility of the enum

    // Create the new variant with the custom message
    let new_variant: Variant = syn::parse_quote! {
        #[error(#custom_message)]
        __WithContext(String, #[source] Box<#enum_name>)
    };

    // Append the new variant to the existing variants
    let mut variants = input_enum.variants;
    variants.push(new_variant);

    let attrs = input_enum.attrs;

    // Generate the modified enum with the new variant
    let output = quote! {
        //#[derive(Error, Debug)]
        #(#attrs)*
        #visibility enum #enum_name {
            #variants
        }

        impl #enum_name {
            pub fn unwrap_context(self) -> (Option<String>,Self) {
                match self {
                    Self::__WithContext(ctx,err) => (Some(ctx),*err),
                    _ => (None,self),
                }
            }
        }

        impl<E,T,S> AddErrorContext<#enum_name, T,S> for std::result::Result<T, E>
        where
            E: Into<#enum_name>,
            S: Into<String>,
        {
            fn with_context(self, f: impl FnOnce() -> S) -> std::result::Result<T, #enum_name> {
                self.map_err(|e| #enum_name::__WithContext(f().into(), Box::new(e.into())))
            }
        }
    };

    output.into()
}