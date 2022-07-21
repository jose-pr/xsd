use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Result;
use syn::{Expr, ExprAssign, Field, ItemStruct, Token};

#[derive(Debug)]
struct AttrArray {
    pub elems: Punctuated<Expr, Token![,]>,
}
impl AttrArray {
    pub fn span(&self) -> proc_macro2::Span {
        self.elems.span()
    }
}
impl IntoIterator for AttrArray {
    type Item = Expr;

    type IntoIter = <Punctuated<Expr, Token![,]> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.into_iter()
    }
}

impl Parse for AttrArray {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut elems = Punctuated::new();

        while !input.is_empty() {
            let first: Expr = input.parse()?;
            elems.push_value(first);
            if input.is_empty() {
                break;
            }
            let punct = input.parse()?;
            elems.push_punct(punct);
        }

        Ok(AttrArray { elems })
    }
}

fn id_field() -> Result<Field> {
    syn::Field::parse_named.parse2(quote! {
    ///The ID of this element. The id value must be of type ID and be unique within the document containing this element.
    pub id: Option<crate::schemas::xsd::Id>
    })
}

fn annotation_field() -> Result<Field> {
    syn::Field::parse_named.parse2(quote! {
    ///Annotation about this xsd node.
    pub annotation: Option<crate::schemas::xsd::Annotation>
    })
}

fn get_args(args: TokenStream) -> syn::Result<HashMap<syn::Ident, proc_macro2::TokenStream>> {
    let args = syn::parse::<AttrArray>(args)?;
    let mut defaults = HashMap::<syn::Ident, proc_macro2::TokenStream>::new();
    for arg in args {
        match arg {
            Expr::Assign(ExprAssign {
                attrs: _,
                left,
                eq_token: _,
                right,
            }) => match *left {
                Expr::Path(expr) => {
                    if let Some(name) = expr.path.get_ident() {
                        defaults.insert(name.clone(), right.into_token_stream());
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(defaults)
}

fn new_fn(
    fields: &syn::Fields,
    defaults: &HashMap<syn::Ident, proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    let mut field_name = vec![];
    let mut field_type = vec![];
    let mut field_value = vec![];
    for field in fields {
        let mut ty = field.ty.clone();
        let id = field.ident.clone().unwrap();
        field_name.push(id.clone());
        let val = if let Some(val) = defaults.get(&id) {
            ty = syn::Field::parse_named
                .parse2(quote! {
                    #id:Option<#ty>
                })
                .unwrap()
                .ty;
            quote! {
                #id.unwrap_or(#val)
            }
        } else {
            quote!(#id)
        };
        field_value.push(val);
        field_type.push(ty);
    }
    quote! {
        #[inline]
        pub fn new(#(#field_name: #field_type,)*) -> Self {
            Self {
                #(#field_name: #field_value,)*
            }

        }
    }
}
fn _xsdnode(args: TokenStream, input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let mut input = syn::parse::<ItemStruct>(input)?;
    let defaults = get_args(args)?;

    if let syn::Fields::Named(ref mut fields) = input.fields {
        fields.named.push(id_field()?);
        fields.named.push(annotation_field()?);
    }

    let name = &input.ident;
    let new = new_fn(&input.fields, &defaults);
    let mut gen_bounds = vec![];
    let mut gen_ident = vec![];

    for gen in input.generics.params.iter() {
        match gen {
            syn::GenericParam::Type(gen) => {
                gen_bounds.push(gen.clone());

                gen_ident.push(gen.ident.clone())
            }
            syn::GenericParam::Lifetime(_) => {}
            syn::GenericParam::Const(_) => {}
        }
    }

    Ok(quote! {
        #input
        impl<#(#gen_bounds),*>  #name <#(#gen_ident),*> {
            #new
        }
    })
}
fn _with_id(args: TokenStream, input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let mut input = syn::parse::<ItemStruct>(input)?;
    let defaults = get_args(args)?;

    if let syn::Fields::Named(ref mut fields) = input.fields {
        fields.named.push(id_field()?);
    }

    let name = &input.ident;
    let new = new_fn(&input.fields, &defaults);

    Ok(quote! {
        #input
        impl  #name {
            #new
        }
    })
}

#[proc_macro_attribute]
pub fn with_id(args: TokenStream, input: TokenStream) -> TokenStream {
    match _with_id(args, input) {
        Ok(stream) => stream,
        Err(er) => er.into_compile_error(),
    }
    .into()
}

#[proc_macro_attribute]
pub fn xsdnode(args: TokenStream, input: TokenStream) -> TokenStream {
    match _xsdnode(args, input) {
        Ok(stream) => stream,
        Err(er) => er.into_compile_error(),
    }
    .into()
}

fn _xsdattr(args: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let args = syn::parse::<AttrArray>(args)?;
    let mut name = None;
    let mut ns: Option<proc_macro2::TokenStream> = None;

    for arg in args.elems.iter() {
        match arg {
            Expr::Assign(expr) => match expr.left {
                _ => {}
            },
            Expr::Lit(expr) => match &expr.lit {
                syn::Lit::Str(lit) => {
                    if name.is_none() {
                        name = Some(lit.value())
                    } else if ns.is_none() {
                        ns = Some(lit.to_token_stream())
                    }
                }
                _ => {}
            },
            _ => {
                if name.is_some() && ns.is_none() {
                    ns = Some(arg.to_token_stream())
                }
            }
        }
    }

    let name = name.unwrap();
    let convert = convert_case::Converter::new().to_case(convert_case::Case::ScreamingSnake);
    let ns = ns.unwrap_or_else(|| syn::LitStr::new("", args.span()).to_token_stream());
    let const_name = syn::Ident::new(&convert.convert(&name), args.span());
    let name = syn::LitStr::new(&name, args.span());
    Ok(quote! {
        pub const #const_name: StaticQname = StaticQname{ namespace:#ns, name:#name };
    })
}
#[proc_macro]
pub fn const_qname(args: TokenStream) -> TokenStream {
    match _xsdattr(args) {
        Ok(stream) => stream,
        Err(er) => er.into_compile_error(),
    }
    .into()
}
