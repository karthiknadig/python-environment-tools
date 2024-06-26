// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use manager::EnvManager;
use pet_python_utils::env::PythonEnv;
use python_environment::PythonEnvironment;
use reporter::Reporter;

pub mod arch;
pub mod manager;
pub mod os_environment;
pub mod python_environment;
pub mod reporter;
pub mod telemetry;

#[derive(Debug, Clone)]
pub struct LocatorResult {
    pub managers: Vec<EnvManager>,
    pub environments: Vec<PythonEnvironment>,
}

pub trait Locator: Send + Sync {
    /**
     * Given a Python environment, this will convert it to a PythonEnvironment that can be supported by this locator.
     * If an environment is not supported by this locator, this will return None.
     *
     * Note: The returned environment could have some missing information.
     * This is because the `from` will do a best effort to get the environment information without spawning Python.
     */
    fn from(&self, env: &PythonEnv) -> Option<PythonEnvironment>;
    /**
     * Finds all environments specific to this locator.
     */
    fn find(&self, reporter: &dyn Reporter);
}
