use serde::*;
use std::fs::File;
use std::env;

pub mod errors;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Steps {
    uses{uses: String},
    name{name: String, run: String},
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    name: String,
    on: String,
    jobs: Jobs
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Jobs {
    build: Build,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Build {
    runs_on: String,
    steps: Vec<Steps>,
}

#[cfg(test)]
mod tests {

    use crate::Workflow;
    use crate::errors::FileError;
    use crate::Build;
    use crate::Jobs;
    use crate::Steps;
    use std::fs::File;
    use std::io::{Write, Read};

    #[test]
    fn test_struct() -> Result<(), FileError> {

        let path = "src/resources/rust.yml";

        let mut contents = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut contents)?;

        let original_workflow = Workflow {
            name: "Rust".to_string(),
            on: "push".to_string(),
            jobs: Jobs {
                build: Build {runs_on: String::from("ubuntu-latest"),
                    steps: vec![Steps::uses { uses: String::from("actions/checkout@v1") },
                                Steps::name{name : String::from("Build"),
                                    run: String::from("cargo build --verbose")},
                                Steps::name{ name : String::from("Run tests"),
                                    run: String::from("cargo test --verbose")}]}
            }
        };

        let s = serde_yaml::to_string(&original_workflow)?;

        let string = s.replace("\"on\"", "on").trim().to_string();

        let deserialized_workflow : Workflow = serde_yaml::from_str::<Workflow>(&string)?;

        let mut new_file = File::create("src/resources/output.yml")?;
        new_file.write_all(string.as_bytes())?;

        assert_eq!(original_workflow, deserialized_workflow);

        Ok(())
    }
}
