// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use pet_core::{
    python_environment::{PythonEnvironment, PythonEnvironmentBuilder, PythonEnvironmentCategory},
    Locator, LocatorResult,
};
use pet_utils::{env::PythonEnv, pyvenv_cfg::PyVenvCfg};

fn is_venv_internal(env: &PythonEnv) -> Option<bool> {
    // env path cannot be empty.
    Some(
        PyVenvCfg::find(env.executable.parent()?).is_some()
            || PyVenvCfg::find(&env.prefix.clone()?).is_some(),
    )
}
pub fn is_venv(env: &PythonEnv) -> bool {
    if let Some(result) = is_venv_internal(env) {
        result
    } else {
        false
    }
}
pub struct Venv {}

impl Venv {
    pub fn new() -> Venv {
        Venv {}
    }
}
impl Default for Venv {
    fn default() -> Self {
        Self::new()
    }
}
impl Locator for Venv {
    fn from(&self, env: &PythonEnv) -> Option<PythonEnvironment> {
        if is_venv(env) {
            let mut name = None;
            if let Some(filename) = &env.prefix {
                name = filename.to_str().map(|f| f.to_string());
            }

            Some(
                PythonEnvironmentBuilder::new(PythonEnvironmentCategory::Venv)
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