use proc_macro::TokenStream;
use proc_macro2::{Ident, Punct, Spacing, Span};
use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, LitInt};

#[proc_macro]
pub fn repeat_ops(item: TokenStream) -> TokenStream {
    let repeat_times = parse_macro_input!(item as LitInt);
    let ident = Ident::new("possible_ops", Span::call_site());
    // Generate the code for declaring the constant variable.

    let n = repeat_times.base10_parse::<usize>().unwrap();

    //    let v = iproduct!(repeat_ops!(3);).collect_vec();

    let mut tokens = quote! {let v = iproduct!(#ident,)};
    tokens.append(Punct::new(',', Spacing::Joint));
    for _ in 0..n {
        tokens.extend(quote!(#ident,));
        tokens.append(Punct::new(',', Spacing::Joint));
    }
    tokens.append(Punct::new(')', Spacing::Joint));
    tokens.into()
}
