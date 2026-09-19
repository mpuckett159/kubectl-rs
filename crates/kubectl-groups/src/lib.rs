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
    pub fn load() -> Result<Self, serde_yaml::Error> {
        let file = File::open("/Users/msalazar/.kube/groups.yaml").unwrap();
        serde_yaml::from_reader(file)
    }
}
