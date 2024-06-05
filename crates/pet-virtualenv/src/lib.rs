// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::fs;

use pet_core::{
    python_environment::{PythonEnvironment, PythonEnvironmentBuilder, PythonEnvironmentCategory},
    Locator, LocatorResult,
};
use pet_utils::env::PythonEnv;

pub fn is_virtualenv(env: &PythonEnv) -> bool {
    if env.prefix.is_none() {
        return false;
    }
    if let Some(bin) = env.executable.parent() {
        // Check if there are any activate.* files in the same directory as the interpreter.
        //
        // env
        // |__ activate, activate.*  <--- check if any of these files exist
        // |__ python  <--- interpreterPath

        // if let Some(parent_path) = PathBuf::from(env.)
        // const directory = path.dirname(interpreterPath);
        // const files = await fsapi.readdir(directory);
        // const regex = /^activate(\.([A-z]|\d)+)?$/i;
        if fs::metadata(bin.join("activate")).is_ok()
            || fs::metadata(bin.join("activate.bat")).is_ok()
        {
            return true;
        }

        // Support for activate.ps, etc.
        if let Ok(files) = std::fs::read_dir(bin) {
            for file in files.filter_map(Result::ok).map(|e| e.path()) {
                if file
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .starts_with("activate")
                {
                    return true;
                }
            }
            return false;
        }
    }

    false
}

pub struct VirtualEnv {}

impl VirtualEnv {
    pub fn new() -> VirtualEnv {
        VirtualEnv {}
    }
}
impl Default for VirtualEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl Locator for VirtualEnv {
    fn from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        if is_virtualenv(env) {
            let mut name = None;
            if let Some(filename) = &env.prefix {
                name = filename.to_str().map(|f| f.to_string());
            }

            Some(
                PythonEnvironmentBuilder::new(PythonEnvironmentCategory::VirtualEnv)
                    .name(name)
                    .executable(Some(env.executable.clone()))
                    .version(env.version.clone())
                    .prefix(env.prefix.clone())
                    .build(),
            )
        } else {
            None
        }
    }

    fn find(&self) -> Option<LocatorResult> {
        // There are no common global locations for virtual environments.
        // We expect the user of this class to call `is_compatible`
        None
    }
}