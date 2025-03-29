use std::path::PathBuf;

use crate::placeholder::Placeholder;
use crate::store_path::StorePath;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SingleDerivedPath {
    Opaque(StorePath),
    Built(SingleDerivedPathBuilt),
}

impl SingleDerivedPath {
    pub fn store_path(&self) -> StorePath {
        match self {
            SingleDerivedPath::Opaque(store_path) => store_path.clone(),
            SingleDerivedPath::Built(built_path) => built_path.drv_path.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SingleDerivedPath::Opaque(store_path) => store_path.to_string(),
            SingleDerivedPath::Built(built_path) => built_path.to_string(),
        }
    }

    pub fn to_input(&self) -> PathBuf {
        match self {
            SingleDerivedPath::Opaque(store_path) => store_path.path().clone(),
            SingleDerivedPath::Built(built_path) => built_path.placeholder(),
        }
    }
}

/// A single derived path that is built from a derivation.
/// Built derived paths are a pair of a derivation and an output name.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SingleDerivedPathBuilt {
    pub drv_path: StorePath,
    pub output: String,
}

impl SingleDerivedPathBuilt {
    pub fn placeholder(&self) -> PathBuf {
        Placeholder::ca_output(&self.drv_path, &self.output).render()
    }

    pub fn to_string(&self) -> String {
        format!("{}^{}", &self.drv_path.to_string(), &self.output)
    }
}
