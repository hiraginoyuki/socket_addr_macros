Macros that can check and parse a SocketAddr at compile-time.

## Examples

```rust
use socket_addr_macros::socket_addr;

use std::net::TcpListener;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind(socket_addr!(127.0.0.1:8080)).unwrap();

    while let Ok((mut conn, _)) = listener.accept() {
        conn.write(b"hello").unwrap();
    }
}
```
