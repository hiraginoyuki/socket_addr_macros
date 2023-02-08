//! [The Rust Standard Library documentation] suggests that, for an argument of type
//! [`ToSocketAddrs`] you should directly pass a string literal, which is parsed at runtime and
//! thus potentially causing a panic. To solve this, [`socket_addr!`] and [`socket_addr_dyn!`] macros
//! do the parsing at compile-time and when the address is invalid, cause a compile error.
//!
//! [`ToSocketAddrs`]: std::net::ToSocketAddrs
//! [`socket_addr!`]: ./macro.socket_addr.html
//! [`socket_addr_dyn!`]: ./macro.socket_addr_dyn.html
//! [The Rust Standard Library documentation]: https://doc.rust-lang.org/stable/std/net/struct.TcpListener.html#examples
//!
//! # Example
//!
//! ```no_run
//! use socket_addr_macros::socket_addr;
//!
//! use std::io::Write;
//! use std::net::TcpListener;
//!
//! fn main() {
//!     let listener = TcpListener::bind(socket_addr!(127.0.0.1:8080)).unwrap();
//!
//!     while let Ok((mut conn, _)) = listener.accept() {
//!         conn.write(b"hello").unwrap();
//!     }
//! }
//! ```

pub(crate) mod ext;
use ext::*;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use std::net::SocketAddr;

/// Parses an IPv4 or IPv6 address at compile-time and returns a [`SocketAddr`].
///
/// [`SocketAddr`]: std::net::SocketAddr
///
/// # Examples
///
/// ```
#[doc = concat!("use ", module_path!(), "::socket_addr;")]
/// use std::net::{SocketAddr, Ipv4Addr, Ipv6Addr};
///
/// let addr_v4: SocketAddr = socket_addr!(1.1.1.1:53);
/// assert_eq!(addr_v4.ip(), Ipv4Addr::new(1, 1, 1, 1));
/// assert_eq!(addr_v4.port(), 53);
///
/// let addr_v6: SocketAddr = socket_addr!([2606:4700:4700::1111]:53);
/// assert_eq!(addr_v6.ip(), Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1111));
/// assert_eq!(addr_v6.port(), 53);
/// ```
/// ```compile_fail
#[doc = concat!("use ", module_path!(), "::socket_addr;")]
///
/// _ = socket_addr!(1.2.3.4.5.6:123456);
/// ```
#[proc_macro]
pub fn socket_addr(input: TokenStream) -> TokenStream {
    TokenStream2::from(input)
        .stringify()
        .parse::<SocketAddr>()
        .map(|addr| addr.to_const_literal())
        .err_as_compile_error()
        .into()
}

/// Parses an IPv4 or IPv6 address at compile-time and returns either a [`SocketAddrV4`] or a
/// [`SocketAddrV6`] depending on the input.
///
/// [`SocketAddrV4`]: std::net::SocketAddrV4
/// [`SocketAddrV6`]: std::net::SocketAddrV6
///
/// # Examples
///
/// ```
#[doc = concat!("use ", module_path!(), "::socket_addr_dyn;")]
/// use std::net::{SocketAddrV4, SocketAddrV6, Ipv4Addr, Ipv6Addr};
///
/// let addr_v4: SocketAddrV4 = socket_addr_dyn!(1.1.1.1:53);
/// assert_eq!(addr_v4.ip(), &Ipv4Addr::new(1, 1, 1, 1));
/// assert_eq!(addr_v4.port(), 53);
///
/// let addr_v6: SocketAddrV6 = socket_addr_dyn!([2606:4700:4700::1111]:53);
/// assert_eq!(addr_v6.ip(), &Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1111));
/// assert_eq!(addr_v6.port(), 53);
/// ```
/// ```compile_fail
#[doc = concat!("use ", module_path!(), "::socket_addr_dyn;")]
///
/// _ = socket_addr_dyn!(1.2.3.4.5.6:123456);
/// ```
#[proc_macro]
pub fn socket_addr_dyn(input: TokenStream) -> TokenStream {
    TokenStream2::from(input)
        .stringify()
        .parse::<SocketAddr>()
        .map(|addr| match addr {
            SocketAddr::V4(addr) => addr.to_const_literal(),
            SocketAddr::V6(addr) => addr.to_const_literal(),
        })
        .err_as_compile_error()
        .into()
}
