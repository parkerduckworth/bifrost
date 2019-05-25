use std::io::{self, Write};
use std::path::PathBuf;
use subprocess::Result as SubprocResult;

use crate::core::docker::{self, manifest_parser::ManifestConfig};
use crate::core::workspace;
use crate::util::BifrostResult;

pub fn run(ws: &workspace::WorkSpace) -> BifrostResult<()> {
    let (out, err) = run_docker(ws)?;

    output_cmd_result(out, err);
    
    Ok(())
}

fn run_docker(ws: &workspace::WorkSpace) -> SubprocResult<(Vec<String>, Vec<String>)> {
    let mut manifest_path: PathBuf = ws.config.cwd.clone();
    manifest_path.push("Bifrost");
    manifest_path.set_extension("toml");

    let mut docker_env = docker::DockerEnv::new(ws);
    let docker_config = ManifestConfig::new(&manifest_path);

    docker_env.collect_commands(docker_config);

    Ok(docker_env.exec_shell()?)
}

fn output_cmd_result(out_vec: Vec<String>, err_vec: Vec<String>) {
    println!("OUTPUT");

    for line in out_vec.iter() {
        io::stdout()
            .write_all(&line.as_bytes())
            .expect("Unable to write to stdout");   
    }

    io::stdout().write_all(b"\n\n").unwrap();

    println!("ERRORS");

    for line in err_vec.iter() {
        io::stdout()
            .write_all(&line.as_bytes())
            .expect("Unable to write to stdout");   
    }
}
