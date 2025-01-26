fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs"); // 仅在 build.rs 更改时重新运行
    println!("Running build script...");
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .out_dir("src/proto")
        .compile_protos(
            &["src/proto/echo.proto",
                "src/proto/helloworld.proto"],
            &["src/proto/"]
        )?;
    Ok(())
}
