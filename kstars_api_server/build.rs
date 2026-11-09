use tonic_prost_build::configure;

fn main() {
    configure()
        .compile_protos(&["proto/ekos.proto"], &["proto"])
        .unwrap();
}
