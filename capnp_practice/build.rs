fn main() {
    capnpc::CompilerCommand::new()
        .file("./hello.capnp")
        .run()
        .expect("Compiling schema");
}
