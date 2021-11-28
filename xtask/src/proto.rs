use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    str::Lines,
};

#[derive(Debug, PartialEq)]
pub struct Protos {
    protos: Vec<Proto>,
}

impl Protos {
    pub fn from_dir(dir: impl AsRef<Path>) -> anyhow::Result<Self> {
        let mut protos = Vec::new();
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.file_stem().filter(|&name| name == "google" || name == "grafeas").is_some() {
                protos.append(&mut Self::from_path(path)?);
            }
        }
        Ok(Self { protos })
    }

    fn from_path(path: impl AsRef<Path>) -> anyhow::Result<Vec<Proto>> {
        let mut protos = Vec::new();
        let path = path.as_ref();
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let path = entry?.path();
                protos.append(&mut Self::from_path(path)?);
            }
        }
        if path.is_file() && path.extension().filter(|&x| x == "proto").is_some() {
            protos.push(Proto::parse(path)?);
        }
        return Ok(protos);
    }

    pub fn proto_paths(&self) -> Vec<&Path> {
        let mut paths = Vec::new();
        for proto in &self.protos {
            paths.push(proto.path.as_path());
        }
        paths
    }

    // map<feature name, set of dependent feature names>
    pub fn feature_names(&self) -> anyhow::Result<BTreeMap<String, BTreeSet<String>>> {
        let mut map = BTreeMap::new();
        for proto in &self.protos {
            let feature = proto.package.as_feature();
            let entry = map.entry(feature.clone()).or_insert_with(BTreeSet::new);
            for import in &proto.imports {
                if !import.is_well_known_type() {
                    let depends_feature = import.as_feature(&self.protos)?;
                    if feature != depends_feature {
                        entry.insert(depends_feature);
                    }
                }
            }
        }
        // To avoid using `include!` to bundle proto files, the build will fail
        // if you do not also include the dependencies of the parent module.
        let mut parent_dependencies = Vec::new();
        for (feature, _) in &map {
            let mut feature_segments = feature.split('-').collect::<Vec<_>>();
            while let Some(_) = feature_segments.pop() {
                let parent_feature = feature_segments.join("-");
                if map.contains_key(&parent_feature) {
                    parent_dependencies.push((feature.to_string(), parent_feature));
                }
            }
        }
        for (feature, parent_feature) in parent_dependencies {
            map.get_mut(&feature).unwrap().insert(parent_feature);
        }
        Ok(map)
    }
}

#[derive(Debug, PartialEq)]
struct Proto {
    path: PathBuf,
    package: Package,
    imports: Vec<Import>,
}

impl Proto {
    fn parse(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let file = fs::read_to_string(path)?;
        let mut lines = file.lines();

        let package = Package::parse(&mut lines)?;
        let mut imports = Vec::new();
        loop {
            match Import::parse(&mut lines) {
                Ok(import) => imports.push(import),
                Err(_) => break,
            }
        }

        Ok(Self { path: path.to_owned(), package, imports })
    }
}

#[derive(Debug, PartialEq)]
struct Package {
    // ex. google.rpc
    raw_name: String,
}

impl Package {
    // https://developers.google.com/protocol-buffers/docs/reference/proto3-spec#package
    // package = "package" fullIdent ";"
    fn parse(lines: &mut Lines<'_>) -> anyhow::Result<Self> {
        const PACKAGE: &str = "package";

        for line in lines {
            let line = line.trim_start(); // remove white spaces
            if line.starts_with(PACKAGE) {
                let package = line
                    .chars()
                    .skip(PACKAGE.len())
                    .take_while(|&c| c != ';')
                    .collect::<String>()
                    .trim()
                    .to_owned();
                return Ok(Self { raw_name: package });
            }
        }

        Err(anyhow::anyhow!("package declaration not found"))
    }

    // https://doc.rust-lang.org/cargo/reference/features.html#features
    // crates.io requires feature names to only contain ASCII letters, digits, _, or -.
    fn as_feature(&self) -> String {
        self.raw_name.split('.').collect::<Vec<_>>().join("-")
    }
}

#[derive(Debug, PartialEq)]
struct Import {
    // ex. google/rpc.proto
    raw_path: String,
}

impl Import {
    // https://developers.google.com/protocol-buffers/docs/reference/proto3-spec#import_statement
    // import = "import" [ "weak" | "public" ] strLit ";"
    fn parse(lines: &mut Lines<'_>) -> anyhow::Result<Self> {
        const IMPORT: &str = "import";

        for line in lines {
            let line = line.trim_start(); // remove white spaces
            if line.starts_with(IMPORT) {
                let import = line
                    .chars()
                    .skip(IMPORT.len())
                    .skip_while(|&c| c != '\'' && c != '"')
                    .skip(1)
                    .take_while(|&c| c != '\'' && c != '"') // checked by protoc
                    .collect::<String>()
                    .trim()
                    .to_owned();
                return Ok(Self { raw_path: import });
            }
        }

        Err(anyhow::anyhow!("import statement not found"))
    }

    fn is_well_known_type(&self) -> bool {
        self.raw_path.starts_with("google/protobuf")
    }

    fn as_feature(&self, protos: &[Proto]) -> anyhow::Result<String> {
        for proto in protos {
            if proto.path.ends_with(&self.raw_path) {
                return Ok(proto.package.as_feature());
            }
        }
        Err(anyhow::anyhow!("not found imported file: {:?}", self.raw_path))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_package_parse() -> anyhow::Result<()> {
        assert_eq!(
            Package::parse(&mut "package mechiru;".lines())?,
            Package { raw_name: "mechiru".into() }
        );
        assert_eq!(
            Package::parse(&mut "package mechiru.storage;".lines())?,
            Package { raw_name: "mechiru.storage".into() }
        );
        assert!(Package::parse(&mut "mechiru.storage;".lines()).is_err());
        Ok(())
    }

    #[test]
    fn test_import_parse() -> anyhow::Result<()> {
        let mut lines = r#"import "mechiru/common1.proto";
                                  import weak "mechiru/common2.proto";
                                  import public "mechiru/common3.proto";"#
            .lines();
        assert_eq!(Import::parse(&mut lines)?, Import { raw_path: "mechiru/common1.proto".into() });
        assert_eq!(Import::parse(&mut lines)?, Import { raw_path: "mechiru/common2.proto".into() });
        assert_eq!(Import::parse(&mut lines)?, Import { raw_path: "mechiru/common3.proto".into() });
        assert!(Import::parse(&mut lines).is_err());
        Ok(())
    }
}
