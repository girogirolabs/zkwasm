# ZKWASM RPC Server

Implements an RPC server on top of zkwasm to expose common operations (e.g., `setup`, `single-prove`, `single-verify`, etc.) as RPC endpoints.

## Testing

Start RPC server.

```
cargo run --bin rpc-server
```

Then run test RPC client.

```
cargo run --bin rpc-client
```
