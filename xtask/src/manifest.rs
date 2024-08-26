use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use toml_edit::{table, value, Array, DocumentMut};

pub struct Manifest {
    path: PathBuf,
    document: DocumentMut,
}

impl Manifest {
    pub fn read(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref().to_owned();
        let file = fs::read_to_string(&path)?;
        let document = file.parse::<DocumentMut>()?;
        Ok(Self { path, document })
    }

    pub fn write(&self) -> anyhow::Result<()> {
        fs::write(&self.path, self.document.to_string()).map_err(Into::into)
    }

    pub fn overwrite_features(&mut self, features: BTreeMap<String, BTreeSet<String>>) -> &Self {
        let mut table = table();
        table["default"] = value(Array::new());
        for (feature, depends_features) in features {
            table[feature] = value(depends_features.into_iter().collect::<Array>());
        }
        self.document["features"] = table;
        self
    }

    pub fn list_feature(&self) -> anyhow::Result<Vec<String>> {
        let features = match self.document["features"].as_table() {
            Some(features) => features.into_iter().map(|(k, _)| k.to_owned()).collect::<Vec<_>>(),
            None => Vec::new(),
        };
        Ok(features)
    }
}
