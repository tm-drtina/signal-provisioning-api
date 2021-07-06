//
// Copyright 2020 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

fn main() {
    let protos = [
        "src/proto/device_messages.proto",
        "src/proto/device_name.proto",
        "src/proto/sub_protocol.proto",
    ];
    prost_build::compile_protos(&protos, &["src"]).expect("Protobufs in src are valid");
    for proto in &protos {
        println!("cargo:rerun-if-changed={}", proto);
    }
}
