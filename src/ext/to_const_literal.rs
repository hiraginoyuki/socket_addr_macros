use proc_macro2::TokenStream;
use quote::quote;

use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, Ipv4Addr, Ipv6Addr};

pub trait ToConstLiteral {
    fn to_const_literal(&self) -> TokenStream;
}

impl ToConstLiteral for Ipv4Addr {
    fn to_const_literal(&self) -> TokenStream {
        let octets = self.octets().into_iter();

        quote! {
            ::std::net::Ipv4Addr::new(#(#octets),*)
        }
    }
}

impl ToConstLiteral for Ipv6Addr {
    fn to_const_literal(&self) -> TokenStream {
        let segments = self.segments().into_iter();

        quote! {
            ::std::net::Ipv6Addr::new(#(#segments),*)
        }
    }
}

impl ToConstLiteral for SocketAddrV4 {
    fn to_const_literal(&self) -> TokenStream {
        let addr = self.ip().to_const_literal();
        let port = self.port();

        quote! {
            ::std::net::SocketAddrV4::new(
                #addr,
                #port,
            )
        }
    }
}

impl ToConstLiteral for SocketAddrV6 {
    fn to_const_literal(&self) -> TokenStream {
        let addr = self.ip().to_const_literal();
        let port = self.port();
        let flowinfo = self.flowinfo();
        let scope_id = self.scope_id();

        quote! {
            ::std::net::SocketAddrV6::new(
                #addr,
                #port,
                #flowinfo,
                #scope_id,
            )
        }
    }
}

impl ToConstLiteral for SocketAddr {
    fn to_const_literal(&self) -> TokenStream {
        match self {
            SocketAddr::V4(addr) => {
                let addr = addr.to_const_literal();
                quote! { ::std::net::SocketAddr::V4(#addr) }
            }
            SocketAddr::V6(addr) => {
                let addr = addr.to_const_literal();
                quote! { ::std::net::SocketAddr::V6(#addr) }
            }
        }

    }
}

// impl ToConstLiteral for SocketAddr {
//     fn to_const_literal(&self) -> TokenStream {
//         let (variant, addr) = match self {
//             SocketAddr::V4(addr) => ("V4", addr.to_const_literal()),
//             SocketAddr::V6(addr) => ("V6", addr.to_const_literal()),
//         };
// 
//         let variant = Ident::new(variant, Span::call_site());
// 
//         quote! {
//             ::std::net::SocketAddr::#variant(#addr)
//         }
//     }
// }
