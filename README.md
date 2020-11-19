# Rust P2P

Pre-requirement: run https://github.com/danitrod/p2p-file-index-server/ and get the URL

To run, create a `.env` file in the root, having the following values:

```
HOST=<your IP or 0.0.0.0>
SERVER_URL=<url for running indexing server>
PORT=<port>
SENHA=<password for connecting to indexing server>
```

Then run with `cargo run`.
