
#[macro_export]
macro_rules! logy {
    ($lvl:expr, $($arg:tt)*) => {
        #[cfg(feature = $lvl)]
        if file!().starts_with(env!("CARGO_MANIFEST_DIR")) {
            println!("[{}:{}:{}:{}]{}",
                $lvl,
                env!("CARGO_CRATE_NAME"), 
                {
                    const LEN: usize = env!("CARGO_MANIFEST_DIR").len();
                    &(file!()[LEN..])
                },
                line!(), 
                format!($($arg)*)
            )
        } else {
            println!("[{}:{}:{}]{}",
                $lvl,
                file!(),
                line!(), 
                format!($($arg)*)
            )
        }
    };
}
#[test]
fn test(){
    logy!("errors", "Foo{}", "Bar");
}
/*
#[macro_export]
macro_rules! logy {
    ($lvl:expr, $($arg:tt)*) => {
        #[cfg(feature = $lvl)]
        println!("[{}:{}:{}]{}",
            $lvl,
            innerlogy!(file!(), env!("CARGO_MANIFEST_DIR"), env!("CARGO_CRATE_NAME")),
            line!(), 
            format!($($arg)*)
        )
    };
}

/// innerlogy!() is pure chad
use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, ExprLit, Lit};
use quote::quote;

#[proc_macro]
pub fn innerlogy(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with syn::parse_macro_input::parse::<syn::punctuated::Punctuated<Expr, syn::Token![,]>>);
    let parts: Vec<String> = args.iter().map(|arg| {
        if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = arg {
            s.value()
        } else {
            panic!("Expected string literals")
        }
    }).collect();

    let (file_path, manifest_dir, crate_name) = match parts.as_slice() {
        [f, m, c] => (f, m, c),
        _ => panic!("Expected exactly three arguments: file!(), CARGO_MANIFEST_DIR, CARGO_CRATE_NAME"),
    };

    let output = if let Some(stripped) = file_path.strip_prefix(manifest_dir) {
        format!("{}:{}", crate_name, stripped)
    } else {
        file_path.to_owned()
    };

    quote!(#output).into()
}


*/