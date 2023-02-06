use itertools::Itertools;
use proc_macro2::{TokenStream, TokenTree::*, Delimiter::*, Ident, Punct, Literal, Group};

pub trait Stringify {
    fn stringify(self) -> String;
}

impl Stringify for TokenStream {
    fn stringify(self) -> String {
        self
            .into_iter()
            .map(|tt| match tt {
                Group(group) => group.stringify(),
                Ident(ident) => ident.stringify(),
                Punct(punct) => punct.stringify(),
                Literal(literal) => literal.stringify(),
            })
            .join("")
    }
}

impl Stringify for Group {
    fn stringify(self) -> String {
        let inner = self.stream().stringify();

        match self.delimiter() {
            Parenthesis => format!("({inner})"),
            Brace => format!("{{{inner}}}"),
            Bracket => format!("[{inner}]"),
            None => inner,
        }
    }
}

macro_rules! impl_stringify {
    ($struct:ident) => {
        impl Stringify for $struct {
            fn stringify(self) -> String {
                self.to_string()
            }
        }
    };
}

impl_stringify!(Ident);
impl_stringify!(Punct);
impl_stringify!(Literal);
