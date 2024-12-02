#![feature(proc_macro_span)]

use std::fs;

use proc_macro::{Span, TokenStream};
use quote::TokenStreamExt;
use regex::Regex;
use syn::{parse::Parse, parse_macro_input, spanned::Spanned, Error, LitStr, Token, TypeBareFn};

struct MatchModule(String, TypeBareFn);

impl Parse for MatchModule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let pattern = input.parse::<LitStr>()?;
        let _ = input.parse::<Token![,]>()?;
        let sig = input.parse::<TypeBareFn>()?;
        Ok(Self(pattern.value(), sig))
    }
}

#[proc_macro]
pub fn match_module(input: TokenStream) -> TokenStream {
    let MatchModule(pattern, signature) = parse_macro_input!(input);
    let output: syn::Result<proc_macro2::TokenStream> = (|| {
        let regex = Regex::new(&pattern).map_err(|e| {
            Error::new(
                Span::call_site().into(),
                format!("cannot parse pattern: {}", e.to_string()),
            )
        })?;
        let src_file = Span::call_site().source_file().path();
        let (src_dir, errors): (Vec<_>, Vec<_>) = src_file
            .parent()
            .ok_or(Error::new(
                Span::call_site().into(),
                "cannot find module parent dir",
            ))?
            .read_dir()
            .map_err(|e| {
                Error::new(
                    Span::call_site().into(),
                    format!("cannot read module dir: {}", e.to_string()),
                )
            })?
            .partition(Result::is_ok);

        let src_dir: Vec<_> = src_dir.into_iter().map(Result::unwrap).collect();
        let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
        if errors.len() > 0 {
            let msg = errors
                .into_iter()
                .map(|e| e.to_string())
                .collect::<Box<[_]>>()
                .join("\n");
            return syn::Result::Err(syn::Error::new(
                Span::call_site().into(),
                format!("cannot read module files: {}", msg),
            ));
        }

        let mut mod_names: Vec<String> = vec![];
        // scan for each file
        for file in src_dir {
            // get raw metadata
            if let Ok(metadata) = file.metadata() {
                let path = if metadata.is_symlink() {
                    fs::read_link(file.path()).map_err(|e| {
                        Error::new(
                            Span::call_site().into(),
                            format!("cannot follow module file symlink: {}", e.to_string()),
                        )
                    })?
                } else {
                    file.path()
                };
                if let Ok(metadata) = path.metadata() {
                    if metadata.is_file() && path.extension().map(|e| e == "rs").unwrap_or_default()
                    {
                        let mod_name = path
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .strip_suffix(".rs")
                            .unwrap();
                        if regex.is_match(mod_name) && !["mod", "lib", "main"].contains(&mod_name) {
                            mod_names.push(mod_name.to_owned());
                        }
                    }
                }
            }
        }

        let mod_ids = mod_names
            .iter()
            .map(|e| syn::Ident::new(&e, Span::call_site().into()));

        let mod_stmt = quote::quote! {
            #( mod #mod_ids; )*
        };

        let (mod_exports, errors): (Vec<_>, Vec<_>) = mod_names
            .iter()
            .map(|e| syn::parse_str::<syn::Path>(format!("{}::{}", e, e).as_str()))
            .partition(Result::is_ok);
        let mod_exports: Vec<_> = mod_exports.into_iter().map(Result::unwrap).collect();
        let errors: Vec<_> = errors
            .into_iter()
            .map(|e| unsafe { e.unwrap_err_unchecked() })
            .collect();
        if errors.len() > 0 {
            let msg = errors
                .into_iter()
                .map(|e| e.to_string())
                .collect::<Box<[_]>>()
                .join("\n");
            return syn::Result::Err(syn::Error::new(
                Span::call_site().into(),
                format!("cannot parse identifier: {}", msg),
            ));
        }

        let export_stmt = quote::quote_spanned! { mod_stmt.span() =>
            use std::collections::HashMap;
            const EXPORTS: &[(&str, #signature)] = &[
                #( (#mod_names, #mod_exports as #signature) ),*
            ];
        };
        let mut stmt = proc_macro2::TokenStream::new();
        stmt.append_all(mod_stmt);
        stmt.append_all(export_stmt);
        // import mods
        Ok(stmt)
    })();

    output.unwrap_or_else(syn::Error::into_compile_error).into()
}
