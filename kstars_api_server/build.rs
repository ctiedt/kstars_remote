use tonic_prost_build::configure;

fn main() {
    configure()
        .compile_protos(&["proto/kstars.proto"], &["proto"])
        .unwrap();
}
