# orb-messages

Defines the communication protocol between the Jetson and microcontrollers.

Protobuf stands for
[Protocol Buffers](https://developers.google.com/protocol-buffers), a
serializing protocol developed by Google, aimed to be language and platform
neutral as the message definition allows us to generate de/serializers for most
programming languages. We are using the
[proto3](https://developers.google.com/protocol-buffers/docs/proto3) version.

Some constraints on the messages are allowed when generating the C
de/serializers with [nanopb](https://github.com/nanopb/nanopb) (small C
library), see `mcu_messaging_*.options` files. Make sure to follow these
constraints in any other languages.

## 🔐 Public and private definitions

We don't want to publicly share the definitions of messages that are used to
communicate tamper events, because it could reveal information about what the
microcontroller is detecting. Instead, for these sensitive messages we provide 
publicly available stubs, where the message contents are empty.

The directory structure looks like this:

```shell
orb-messages
├── messages         # protobuf definitions
│   └── private_stub # stubs for the closed source definitions
├── src              # Rust library
└── zephyr           # builds the library for Zephyr
```

## 📦 Messages

All the messages are wrapped into a versioned `McuMessage` to ensure versions
support before further decoding of the message.

The `McuMessage` encapsulates messages depending on the direction of the
message:

- `JetsonToMcu`: Jetson to main MCU. Commands to be executed by the main MCU.
- `McuToJetson`: Main MCU to Jetson. Data retrieved by the main MCU.
- `JetsonToSec`: Jetson to security MCU. Commands to be executed by the security
  MCU.
- `SecToJetson`: Security MCU to Jetson. Data retrieved by the security MCU.

All the `JetsonTo*` messages are acknowledged by the recipient using an
`*ToJetson` message with an `Ack` payload.

Commands and data are documented into the `.proto` files.

## 📝 Usage

We first need to install `protoc`, the Protobuf compiler. `protoc` must either
be present in `$PATH` or alternatively be reachable by setting the `PROTOC`
environment variable to its path.

- macOS
  ```shell
  brew install protobuf
  ```
- Debian-based OS
  ```shell
  apt-get install protobuf-compiler
  ```
- Nix
  ```shell
  nix shell nixpkgs#protobuf
  ```

Then generate the encoder/decoder in the language you want.

### Formatting

We use pre-commit to run checks on the proto files:

```shell
pre-commit install -c .pre-commit-config.yaml
```

## 🪪 License

Unless otherwise specified, all code in this repository is dual-licensed under
either:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0, with LLVM Exceptions
  ([LICENSE-APACHE](LICENSE-APACHE))

at your option. This means you may select the license you prefer to use.

Any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
