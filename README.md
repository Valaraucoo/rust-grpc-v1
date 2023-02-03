### gRPC and Rust using tonic framework

Based on [_Let's build a gRPC server and client in Rust with tonic_](https://www.thorsten-hans.com/grpc-services-in-rust-with-tonic/).

#### How to run

To run server:
```bash
cd server && cargo run
```

Then run client:
```bash
cd client && cargo run
```

Tonic automatically generates the client and server code (stubs) from the proto file (`voting.proto`). In each `server` and `client` directory, there is a `build.rs` file that generates the code.
