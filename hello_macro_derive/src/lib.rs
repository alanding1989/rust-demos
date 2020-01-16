use proc_macro::TokenStream;

use syn::{parse, DeriveInput};

use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn macro_derive(input: TokenStream) -> TokenStream {
    // 构建Rust代码的抽象语法树，以便对其进行操作
    let ast = syn::parse(input).unwrap();

    // 构建 trait 实现
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Macro)]
pub fn derive(input: TokenStream) -> TokenStream {
    // 构建Rust代码的抽象语法树，以便对其进行操作
    let ast: DeriveInput = syn::parse(input).unwrap();

    // 为结构体实现方法
    let name = &ast.ident;
    let new_ts = quote! {
        impl #name {
            fn proc_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    new_ts.into()
}
