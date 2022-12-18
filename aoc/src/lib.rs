use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let path = match &parse_macro_input!(args as AttributeArgs)[..] {
        [NestedMeta::Lit(Lit::Int(day))] => format!("../../input/{}.txt", day.token()),
        _ => panic!("Expected one integer argument"),
    };

    let tokens = quote! {
        const INPUT: &str = include_str!(#path);

        fn main() {
            let now = ::std::time::Instant::now();
            let a = part1(INPUT);
            let elapsed = now.elapsed();
            println!("Part 1 ({}):\n{}\n", duration_string(elapsed), a);

            let now = ::std::time::Instant::now();
            let b = part2(INPUT);
            let elapsed = now.elapsed();
            println!("Part 2 ({}):\n{}", duration_string(elapsed), b);
        }

        fn duration_string(d: ::std::time::Duration) -> String {
            match d.as_millis() {
                e if e >= 1000 => format!("{:.2}s", d.as_secs_f64()),
                e if e > 0 => format!("{}ms", e),
                _ => format!("{}μs", d.as_micros()),
            }
        }
    };

    TokenStream::from(tokens)
}
