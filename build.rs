use std::fs::File;
use std::io::Write;

fn main() {
    // collect .proto
    let mut files = Vec::new();
    for entry in std::fs::read_dir("protos").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().unwrap().to_str().unwrap() != "proto" {
            continue;
        }
        let filename = path.file_name().unwrap().to_str().unwrap();
        files.push(filename.to_owned());
    }

    // compile .proto
    let proto_root = "protos";
    protoc_grpcio::compile_grpc_protos(
        files.as_slice(),
        &[proto_root],
        "protos/src",
        None
    ).expect("Failed to compile gRPC definitions!");

    // generate lib.rs
    let mut lib_rs = File::create("protos/src/lib.rs").unwrap();
    for entry in std::fs::read_dir("protos/src").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename == "lib.rs" {
            continue;
        }
        lib_rs.write(format!("pub mod {};\n", filename.trim_end_matches(".rs")).as_ref()).unwrap();
    }

}
