/// # Offscale github actions
///
/// This repository defines a set of structs for the conversion and storage
/// of github actions keywords. This library stores the essential structs which are
/// intended to be reused across multiple conversion libraries.

// Imports

use serde::*;
use std::fs::File;
use std::env;

// Errors module
pub mod errors;

/// The basic Workflow struct. Takes in the following information:
/// name: The workflow name.
/// on: When the workflow action will occur (push, pull...etc)
/// jobs: The actual actions which will occur when the action is triggered.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    name: String,
    on: String,
    jobs: Jobs
}

/// Defines a set of actions which will occur. This stores all build commands
/// which will happen when the trigger occurs.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Jobs {
    build: Build,
}

/// Defines an action, or set of actions. The struct takes in the following inputs:
/// runs_on: defines the system it will run on.
/// steps: Defines what commands will be undertaken with this job.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Build {
    runs_on: String,
    steps: Vec<Steps>,
}

/// Enums which store information about which steps will be occuring.
/// So far, two steps are defined:
/// uses: defines a separate action urs which will occur.
/// name: Defines a command name, along wiht its specific cli command
/// eg: cargo run.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Steps {
    uses { uses: String },
    name { name: String, run: String },
}

/// Basic test (check 'resources' folder for intended output)
/// This test attempts to replicate the 'rust.yml' file in the resources folder.
/// Uses the specified structs to re-create that file and checks to see whether the
/// struct can be safely serialised and deserialized.
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

        // Create the workflow struct.
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

        // Serialize into yaml
        let workflow_string = serde_yaml::to_string(&original_workflow)?;

        // TEMPORARY - formats the 'on' to the correct format (not sure why it serialises differently
        // in serde.)
        let workflow_string = workflow_string.replace("\"on\"", "on").trim().to_string();

        // deserialise the string back into a Workflow struct.
        let deserialized_workflow : Workflow = serde_yaml::from_str::<Workflow>(&workflow_string)?;

        // Write new yaml file to a file (output.yml)
        let mut new_file = File::create("src/resources/output.yml")?;
        new_file.write_all(workflow_string.as_bytes())?;

        // Check that the structs are the same.
        assert_eq!(original_workflow, deserialized_workflow);

        Ok(())
    }
}
