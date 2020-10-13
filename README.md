# 🚇 sltunnel
A simple TLS tunneling implementation, written in Rust.

## What is TLS tunneling?
TLS tunneling is the way to transport any TCP packets via a TLS tunnel.
Thanks to TLS, we can communicate with remote server more securely, through general firewalls.

### What about sslh?
sslh is a multiplexer of packets using their header bytes.
It is not a problem in general home networks, but some network (e.g. schools or works) restricts to transport without correct TLS negotiation even in port 443.

TLS tunneling is different.
The transportation through a TLS tunnel is completely negotiated as a TLS connection.
Usually, the firewall accepts the connection even in schools or works!

## Installation
```toml
[dependencies]
sltunnel = { path = "[path to crate]" }
```

## Examples
In this case, let them to communicate between `[::]:11234` and `[::]:22334` via `[::]:33445`.

### Server
The server listens on `[::]:33445` with TLS and relays connections to `[::]:11234`.

```console
$ cd ./examples/server
$ cargo build --release
$ ./target/release/sltunnel_server [::]:33445 [::]:11234
```

### Client
The client listens on `[::]:22334` and relays connections to `[::]:33445` with TLS.

```console
$ cd ./examples/client
$ cargo build --release
$ ./target/release/sltunnel_client [::]:22334 [::]:33445
```

### Testing
When both server and client is ready, run the command to check the connection:

```console
$ nc -k -l 11223 &
$ echo "OK" > /dev/tcp/localhost/22334
```

If the console outputs "OK", it works!
