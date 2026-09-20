use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Groups {
    pub api_version: String,
    pub kind: String,
    pub context_groups: BTreeMap<String, Vec<String>>,
}

impl Groups {
    pub fn default() -> Self {
        Groups {
            api_version: "v1alpha1".to_string(),
            kind: "Groups".to_string(),
            context_groups: BTreeMap::new(),
        }
    }
    pub fn load(path: String) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_reader(File::open(shellexpand::tilde(&path).into_owned()).unwrap())
    }
}
