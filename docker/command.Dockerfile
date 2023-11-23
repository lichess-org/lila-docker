FROM messense/cargo-zigbuild:0.18.0

WORKDIR /command

ENTRYPOINT cargo zigbuild --release --target x86_64-apple-darwin
