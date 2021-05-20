#[macro_use] extern crate magic_crypt;
extern crate proc_macro2;

use proc_macro::{TokenStream, TokenTree};
use magic_crypt::MagicCryptTrait;
use quote::quote;
use proc_macro2::Literal;
use random_string::Charset;


const CHARSET_S: &'static str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890";

#[proc_macro]
pub fn strenc_initialize(_: TokenStream) -> TokenStream {
    let internal_module = quote! {
        pub mod internal_strenc {
            use magic_crypt::MagicCryptTrait;

            pub fn decrypt_bytes(encrypted: &[u8], key: &str) -> String {
                let dec = new_magic_crypt!(key, 256).decrypt_bytes_to_bytes(encrypted).unwrap();
                String::from_utf8(dec).unwrap()
            }
        }
    };

    internal_module.into()
}

#[proc_macro]
pub fn enc(tokens: TokenStream) -> TokenStream {
    let mut target = String::new();
    for token in tokens.clone() {
        target = match token {
            TokenTree::Literal(lit) => lit.to_string(),
            _ => "".to_owned()
        }
    }

    if target.is_empty() {
        return tokens;
    }

    target = String::from(&target[1..target.len() - 1]);

    let key = random_string::generate(
        32,
        &Charset::new(CHARSET_S).unwrap()
    ).to_string();

    let bytes = new_magic_crypt!(&key, 256).encrypt_bytes_to_bytes(target.as_bytes());
    let encrypted = Literal::byte_string(&bytes);

    let result = quote! {
        crate::internal_strenc::decrypt_bytes(#encrypted, #key)
    };

    result.into()
}
