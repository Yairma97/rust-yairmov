fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs"); // 仅在 build.rs 更改时重新运行
    println!("Running build script...");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("src/proto")
        .compile_protos(
            &["src/proto/helloworld.proto"],
            &["src/proto/"]
        )?;
    Ok(())
}
