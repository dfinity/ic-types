# DEPRECATED

This is no longer supported. It's original contents has been moved to other projects.

* `Principal`: [`candid`](https://github.com/dfinity/candid/blob/28a9fe8e898dc7662de2a13204b6f06bce536e14/rust/candid/src/types/principal.rs).
* `HashTree`: [`agent-rs`](https://github.com/dfinity/agent-rs/tree/c9f9497dc8ced64b76250d76967f4f61a56d8262/ic-certification).

# IC Types

## Contributing
Please follow the guidelines in the [CONTRIBUTING.md](.github/CONTRIBUTING.md) document.

## Goal
This library contains typings and utility functions dealing with the Public specification of the Internet Computer
 and the HTTP
client. It might be shared in the future but for now is separated for the purpose of testing and
development.

## Running Tests
Regular tests can be run by anyone using `cargo test`. This will not run a special version of the
tests that runs against a server implementation of the public specification, though those
tests can be run against a compatible Replica.

## References
See https://sdk.dfinity.org/docs/interface-spec/index.html
