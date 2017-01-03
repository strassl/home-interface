## Cross-compilation
Use the docker image from [here](https://github.com/Ragnaroek/rust-on-raspberry-docker).

OpenSSL can be found [here](http://archive.raspbian.org/raspbian/pool/main/o/openssl/).

```bash
docker run --volume ./:/home/cross/project --volume ./deb-deps:/home/cross/deb-deps ragnaroek/rust-raspberry:1.14.0 build --release
```

The result can be found in `target/arm-unknown-linux-gnueabihf/release/home-interface`
