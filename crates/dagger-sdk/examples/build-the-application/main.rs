use dagger_sdk::HostDirectoryOpts;

fn main() -> eyre::Result<()> {
    let client = dagger_sdk::connect()?;

    let host_source_dir = client.host().directory_opts(
        "examples/build-the-application/app",
        HostDirectoryOpts {
            exclude: Some(vec!["node_modules".into(), "ci/".into()]),
            include: None,
        },
    );

    let source = client
        .container()
        .from("node:16")
        .with_mounted_directory("/src", host_source_dir.id()?);

    let runner = source
        .with_workdir("/src")
        .with_exec(vec!["npm", "install"]);

    let test = runner.with_exec(vec!["npm", "test", "--", "--watchAll=false"]);

    let build_dir = test
        .with_exec(vec!["npm", "run", "build"])
        .directory("./build");

    let _ = build_dir.export("./build");

    let entries = build_dir.entries();

    println!("build dir contents: \n {:?}", entries);

    Ok(())
}
