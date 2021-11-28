use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use toml_edit::{value, Array, Document};

pub struct Manifest {
    path: PathBuf,
    document: Document,
}

impl Manifest {
    pub fn read(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref().to_owned();
        let file = fs::read_to_string(&path)?;
        let document = file.parse::<Document>()?;
        Ok(Self { path, document })
    }

    pub fn write(&self) -> anyhow::Result<()> {
        fs::write(&self.path, &self.document.to_string()).map_err(Into::into)
    }

    pub fn set_features(&mut self, features: BTreeMap<String, BTreeSet<String>>) -> &Self {
        self.document["features"]["default"] = value(Array::new());
        for (feature, depends_features) in features {
            let array = depends_features.into_iter().collect::<Array>();
            self.document["features"][feature] = value(array);
        }
        self
    }

    pub fn list_feature(&self) -> anyhow::Result<Vec<String>> {
        let features = match self.document["features"].as_table() {
            Some(features) => features.into_iter().map(|(k, _)| k.to_string()).collect::<Vec<_>>(),
            None => Vec::new(),
        };
        Ok(features)
    }
}
