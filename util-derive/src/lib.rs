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
                    Self {
                        #(#fields: self.#fields #operation rhs.#fields),*
                    }
                )
            },
            syn::Fields::Unnamed(fields) => {
                let fields: Vec<_> = (0..fields.unnamed.len()).map(syn::Index::from).collect();
                quote!(
                    Self(
                        #(self.#fields #operation rhs.#fields),*
                    )
                )
            },
            syn::Fields::Unit => panic!("Unit Structs cannot derive the {} Macro.", operation_name),
        }
    } else {
        panic!("Only Structs can derive the {} Macro.", operation_name)
    };

    let fn_name = syn::Ident::new(operation_name.to_string().to_lowercase().as_str(), proc_macro2::Span::call_site());

    quote!(
        impl<T : Into<#name>> std::ops::#operation_name<T> for #name {
            type Output = Self;

            fn #fn_name(self, rhs: T) -> Self::Output {
                let rhs: Self = rhs.into();
                #fields
            }
        }
    )
}

/// Derive macro for the impl of the trait [`std::ops::Add`] for types that
/// implement [`Into<T>`] where T = the struct. This is a naïve implementation
/// which adds each field together of the LHS and RHS.
/// 
/// *Note: This proc macro is restricted to only named and unnamed structs.*
/// 
/// ## Example
/// ```
/// use util_derive::Add;
/// 
/// #[derive(Add)]
/// struct Foo {
///     x: f64,
///     y: f64,
/// }
/// 
/// let a = Foo {
///     x: 10.0,
///     y: 20.0,
/// };
/// 
/// let b = Foo {
///     x: 30.0,
///     y: 40.0,
/// };
/// 
/// let sum = a + b;
/// 
/// assert_eq!(sum.x, 40.0);
/// assert_eq!(sum.y, 60.0);
/// ```
/// Will expand to:
/// ```
/// struct Foo {
///     x: f64,
///     y: f64,
/// }
/// 
/// impl<T : Into<Foo>> std::ops::Add<T> for Foo {
///     type Output = Self;
///     
///     fn add(self, rhs: T) -> Self::Output {
///         let rhs: Self = rhs.into();
///         Self {
///             x: self.x + rhs.x,
///             y: self.y + rhs.y,
///         }
///     }
/// }
/// ```
#[proc_macro_derive(Add)]
pub fn add_macro(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();

    TokenStream::from(derive_macro(ast, quote!(Add), quote!(+)))
}

/// Derive macro for the impl of the trait [`std::ops::Sub`] for types that
/// implement [`Into<T>`] where T = the struct. This is a naïve implementation
/// which subtracts each field together of the LHS and RHS.
/// 
/// *Note: This proc macro is restricted to only named and unnamed structs.*
/// 
/// ## Example
/// ```
/// use util_derive::Sub;
/// 
/// #[derive(Sub)]
/// struct Foo {
///     x: f64,
///     y: f64,
/// }
/// 
/// let a = Foo {
///     x: 40.0,
///     y: 30.0,
/// };
/// 
/// let b = Foo {
///     x: 20.0,
///     y: 10.0,
/// };
/// 
/// let diff = a - b;
/// 
/// assert_eq!(diff.x, 20.0);
/// assert_eq!(diff.y, 20.0);
/// ```
/// Will expand to:
/// ```
/// struct Foo {
///     x: f64,
///     y: f64,
/// }
/// 
/// impl<T : Into<Foo>> std::ops::Sub<T> for Foo {
///     type Output = Self;
///     
///     fn sub(self, rhs: T) -> Self::Output {
///         let rhs: Self = rhs.into();
///         Self {
///             x: self.x - rhs.x,
///             y: self.y - rhs.y,
///         }
///     }
/// }
/// ```
#[proc_macro_derive(Sub)]
pub fn sub_macro(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();

    TokenStream::from(derive_macro(ast, quote!(Sub), quote!(-)))
}

/// Derive macro for the impl of the trait [`std::ops::Mul`] for types that
/// implement [`Into<T>`] where T = the struct. This is a naïve implementation
/// which multiplies each field together of the LHS and RHS.
/// 
/// *Note: This proc macro is restricted to only named and unnamed structs.*
/// 
/// ## Example
/// ```
/// use util_derive::Mul;
/// 
/// #[derive(Mul)]
/// struct Foo {
///     x: f64,
///     y: f64,
/// }
/// 
/// let a = Foo {
///     x: 10.0,
///     y: 20.0,
/// };
/// 
/// let b = Foo {
///     x: 30.0,
///     y: 40.0,
/// };
/// 
/// let prod = a * b;
/// 
/// assert_eq!(prod.x, 300.0);
/// assert_eq!(prod.y, 800.0);
/// ```
/// Will expand to:
/// ```
/// struct Foo {
///     x: f64,
///     y: f64,
/// }
/// 
/// impl<T : Into<Foo>> std::ops::Mul<T> for Foo {
///     type Output = Self;
///     
///     fn mul(self, rhs: T) -> Self::Output {
///         let rhs: Self = rhs.into();
///         Self {
///             x: self.x * rhs.x,
///             y: self.y * rhs.y,
///         }
///     }
/// }
/// ```
#[proc_macro_derive(Mul)]
pub fn mul_macro(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();

    TokenStream::from(derive_macro(ast, quote!(Mul), quote!(*)))
}

/// Derive macro for the impl of the trait [`std::ops::Div`] for types that
/// implement [`Into<T>`] where T = the struct. This is a naïve implementation
/// which divides each field together of the LHS and RHS.
/// 
/// *Note: This proc macro is restricted to only named and unnamed structs.*
/// 
/// ## Example
/// ```
/// use util_derive::Div;
/// 
/// #[derive(Div)]
/// struct Foo {
///     x: f64,
///     y: f64,
/// }
/// 
/// let a = Foo {
///     x: 40.0,
///     y: 30.0,
/// };
/// 
/// let b = Foo {
///     x: 20.0,
///     y: 10.0,
/// };
/// 
/// let quo = a / b;
/// 
/// assert_eq!(quo.x, 2.0);
/// assert_eq!(quo.y, 3.0);
/// ```
/// Will expand to:
/// ```
/// struct Foo {
///     x: f64,
///     y: f64,
/// }
/// 
/// impl<T : Into<Foo>> std::ops::Div<T> for Foo {
///     type Output = Self;
///     
///     fn div(self, rhs: T) -> Self::Output {
///         let rhs: Self = rhs.into();
///         Self {
///             x: self.x / rhs.x,
///             y: self.y / rhs.y,
///         }
///     }
/// }
/// ```
#[proc_macro_derive(Div)]
pub fn div_macro(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast: syn::DeriveInput = syn::parse_str(&s).unwrap();

    TokenStream::from(derive_macro(ast, quote!(Div), quote!(/)))
}

/// Derive macro for the impl of the trait [`std::ops::Add`], [`std::ops::Sub`]
/// [`std::ops::Mul`], and [`std::ops::Div`] for types that implement
/// [`Into<T>`] where T = the struct. This is a naïve implementation
/// which does the operation on each field together of the LHS and RHS.
/// 
/// *Note: This proc macro is restricted to only named and unnamed structs.*
/// 
/// This is equivalent to using each of this crate's proc macros. 
/// 
/// ## Example
/// ```
/// use util_derive::PartialOps;
/// 
/// #[derive(PartialOps)]
/// struct Foo {
///     x: f64,
///     y: f64,
/// }
/// ```
/// Is the same as:
/// ```
/// use util_derive::{Add, Sub, Div, Mul};
/// 
/// #[derive(Add, Sub, Div, Mul)]
/// struct Foo {
///     x: f64,
///     y: f64,
/// }
/// ```
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