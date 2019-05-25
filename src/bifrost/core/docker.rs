use subprocess::{self, Popen, PopenConfig, Redirection, Result as SubprocResult};
use std::path::PathBuf;

use crate::core::workspace;
use manifest_parser::ManifestConfig;

pub struct DockerEnv {
    mounted_dir: PathBuf,
    cmds: Vec<String>,
    stdout: Vec<String>,
    stderr: Vec<String>,
}

impl DockerEnv {
    pub fn new(ws: &workspace::WorkSpace) -> Self {
        let mut mounted_dir = ws.config.home_path.clone();
        mounted_dir.push(".bifrost/container:/test");

        DockerEnv { 
            mounted_dir,
            cmds: vec![],
            stdout: vec![],
            stderr: vec![]
        }
    }

    pub fn collect_commands(&mut self, config: ManifestConfig) {
        for cmd in config.command.cmds.iter() {
            self.cmds.push(cmd.to_string())
        }
    }

    pub fn exec_shell(&mut self) -> SubprocResult<(Vec<String>, Vec<String>)> {
        let md = match self.mounted_dir.to_str() {
            Some(md) => md,
            None => panic!("No mounted directory found!"),
        };

        let mut process = Popen::create(
            &["docker", "run", "--rm", "-i", "-v", md, "bifrost:0.1"],
            PopenConfig {
                stdout: Redirection::Pipe, 
                stdin: Redirection::Pipe,
                stderr: Redirection::Pipe,
                ..Default::default()
        }).expect("Popen failure");


        let mounted_cmd = format!(
            "bash -c \"cd /test/; {}; \"", 
            self.cmds.join(" && ")
        );

        let (stdout, stderr) = process
            .communicate(Some(&mounted_cmd))
            .expect("communicate failure");

        self.process_output(
            stdout.unwrap_or("".to_string()), 
            stderr.unwrap_or("".to_string())
        );
        
        Ok((self.stdout.clone(), self.stderr.clone()))
    }

    fn process_output(&mut self, stdout: String, stderr: String) {
        if !stderr.is_empty() {
             self.stderr.push(stderr);
        }

        if !stdout.is_empty() {
            self.stdout.push(stdout);
        }
    }
}

pub mod manifest_parser {
    use serde_derive::Deserialize;
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    #[derive(Deserialize, Debug)]
    pub struct ManifestConfig {
        pub project: ProjectConfig,
        pub container: ContainerConfig,
        pub workspace: WorkspaceConfig,
        pub command: CommandConfig,
    }

    #[derive(Deserialize, Debug)]
    pub struct ProjectConfig {
        pub name: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct ContainerConfig {
        pub name: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct WorkspaceConfig {
        pub name: String,
        pub ignore: Vec<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct CommandConfig {
        pub cmds: Vec<String>,
    }

    impl ManifestConfig {
        pub fn new(manifest: &PathBuf) -> ManifestConfig {
            let mut manifest = File::open(manifest).expect("Unable to open maifest");
            let mut contents = String::new();
            
            manifest.read_to_string(&mut contents).expect("Unable to read maifest");
            
            toml::from_str(&contents).expect("Unable to deserialize maifest")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use super::*;

    #[test]
    fn parse_user_config() {
        let parsed_config = setup();

        assert_eq!("docker", parsed_config.container.name);

        teardown();
    }

    fn setup() -> manifest_parser::ManifestConfig {
        create_test_file();

        manifest_parser::ManifestConfig::new(&PathBuf::from("test.toml"))
    }

    fn teardown() {
        remove_test_file();
    }

    fn create_test_file() {
        File::create("test.toml").expect("Unable to create test file");
        fs::write("test.toml", 
r#"[project]
name = "project name"

[container]
name = "docker"

[workspace]
name = "name of workspace"
ignore = ["target", ".git", ".gitignore"]

[command]
cmds = ["echo hello world"]
        "#).expect("Unable to write to test file");
    }

    fn remove_test_file() {
        fs::remove_file("test.toml").expect("Unable to remove test file");
    }
}