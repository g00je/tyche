use proc_macro2::TokenStream;
use quote_into::quote_into;

pub fn array_index(
    arr: &Option<Vec<usize>>, gen_out: &impl Fn(TokenStream) -> TokenStream,
) -> TokenStream {
    let arr = match arr {
        Some(a) => a,
        None => {
            return gen_out(TokenStream::new());
        }
    };

    fn inner(
        arr: &Vec<usize>, idx: &mut Vec<usize>, lvl: usize,
        gen_out: &impl Fn(TokenStream) -> TokenStream,
    ) -> TokenStream {
        if arr.is_empty() {
            let mut s = TokenStream::new();
            for i in idx {
                quote_into! {s += [#i]}
            }
            return gen_out(s);
        }

        let mut arr = arr.clone();
        let mut s = TokenStream::new();

        quote_into! {s +=
            [
                #{(0..arr.remove(0)).for_each(|i| {
                    idx[lvl] = i;
                    quote_into!{s += #(inner(&arr, idx, lvl+1, gen_out)),}
                })}
            ]
        }

        s
    }

    let mut idx = vec![0; arr.len()];
    inner(arr, &mut idx, 0, gen_out)
}
