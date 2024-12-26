# svctl 

`svctl` is CLI for setting up and managing a fleet of (testnet|mainnet) validators.

> NOTE: work-in-progress.

## Building

Build `svctl` as a typical rust binary.

The `buf` cli needs to be be available:

    brew install buf

The protobuf plugins need to be available on your path:

    cargo install protoc-gen-prost
    cargo install protoc-gen-prost-serde
    cargo install protoc-gen-prost-crate
    cargo install protoc-gen-tonic
