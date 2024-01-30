# Messaging interface for the microcontrollers

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

We don't want to share publicly the definitions of the sensitive messages that
are used to communicate tamper events. Thus, we have two sets of definitions:

- Public definitions in the root directory, that are shared with the
  [public repository](https://www.github.com/worldcoin/open-orb-mcu-messaging).
- Private definitions in `private/` that are not shared with the public
  repository.

```shell
orb-mcu-messaging
├── nanopb  # (private) nanopb C library, allows testing from CI
├── private # (private) protobuf definitions
├── src     # Rust library
└── zephyr  # builds the library for Zephyr
```

`nanopb` is used in the private repository to perform checks in CI.

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

Then generate the encoder/decoder in the language you want.

Here is an example with the Python language,
[more info here](https://developers.google.com/protocol-buffers/docs/reference/python-generated):

```shell
protoc --python_out=. orb-mcu-messaging/mcu_messaging_main.proto orb-mcu-messaging/mcu_messaging_common.proto
```

Here is an example of manually encoded a message. The text format for writing a
message for direct encoding by `protoc` is specified
[here](https://protobuf.dev/reference/protobuf/textformat-spec/). What follows
is an example of trying to encode an McuMessage going from the main MCU to the
Jetson.

```shell
output=$(mktemp -t encoded-msg.XXX)
protoc --encode McuMessage orb-mcu-messaging/mcu_messaging_main.proto > "$output" <<EOF
version: VERSION_0
m_message {
    gnss_partial {
        counter: 4294967295
        nmea_part: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    }
}
EOF
printf "Encoded file ($output) contents:\n"
hexdump -C "$output"
printf "Encoded file size: %d\n" "$(wc -c < "$output")"
```

### Formatting

We use pre-commit to run checks on the proto files:

```shell
pre-commit install -c .pre-commit-config.yaml
```

To ensure code will pass in CI:

```shell
make
```
