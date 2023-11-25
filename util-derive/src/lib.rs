use proc_macro::TokenStream;

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
use proc_macro2::TokenStream as Tokens;

fn derive_macro(ast: syn::DeriveInput, operation_name: Tokens, operation: Tokens) -> Tokens {
    let name = ast.ident;
    let fields = if let syn::Data::Struct(f) = ast.data {
        match f.fields {
            syn::Fields::Named(fields) => {
                let fields: Vec<_> = fields.named.iter().map(|f| f.ident.clone()).collect();
                quote!(
                    #name {
                        #(#fields: self.#fields #operation rhs.#fields),*
                    }
                )
            },
            syn::Fields::Unnamed(fields) => {
                let fields: Vec<_> = fields.unnamed.iter().enumerate().map(|(i, _)| syn::Index::from(i)).collect();
                quote!(
                    #name(
                        #(self.#fields #operation rhs.#fields),*
                    )
                )
            },
            syn::Fields::Unit => panic!("Unit Structs cannot derive the {} Macro.", operation_name.to_string()),
        }
    } else {
        panic!("Only Structs can derive the {} Macro.", operation_name.to_string())
    };

    let fn_name = syn::Ident::new(operation_name.to_string().to_lowercase().as_str(), proc_macro2::Span::call_site());

    quote!(
        impl<T : Into<#name>> std::ops::#operation_name<T> for #name {
            type Output = Self;

            fn #fn_name(self, rhs: T) -> Self::Output {
                let rhs: #name = rhs.into();
                #fields
            }
        }
    )
}

#[proc_macro_derive(Add)]
pub fn add_macro(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();

    TokenStream::from(derive_macro(ast, quote!(Add), quote!(+)))
}

#[proc_macro_derive(Sub)]
pub fn sub_macro(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();

    TokenStream::from(derive_macro(ast, quote!(Sub), quote!(-)))
}

#[proc_macro_derive(Mul)]
pub fn mul_macro(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();

    TokenStream::from(derive_macro(ast, quote!(Mul), quote!(*)))
}

#[proc_macro_derive(Div)]
pub fn div_macro(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();

    TokenStream::from(derive_macro(ast, quote!(Div), quote!(/)))
}

#[proc_macro_derive(PartialOps)]
pub fn partial_ops(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();

    let [add, sub, mul, div] = [
        derive_macro(ast.clone(), quote!(Add), quote!(+)),
        derive_macro(ast.clone(), quote!(Sub), quote!(-)),
        derive_macro(ast.clone(), quote!(Mul), quote!(*)),
        derive_macro(ast,         quote!(Div), quote!(/))
    ];

    TokenStream::from(
        quote! {
            #add
            #sub
            #mul
            #div
        }
    )
}