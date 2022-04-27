use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq)]
pub struct Package {
    root: Module,
}

impl Package {
    pub fn from_dir(dir: impl AsRef<Path>) -> anyhow::Result<Self> {
        let mut root = Module::new(Vec::new());

        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            let name = path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("empty file name: {:?}", path))?
                .to_string_lossy();

            let mut iter = name.trim_end_matches(".rs").split('.').peekable();
            if iter.peek().is_none() {
                return Err(anyhow::anyhow!("empty file name: {:?}", path));
            }

            let mut module_path = Vec::new();
            let mut module = &mut root;
            loop {
                let name: Name = iter.next().expect("already checked").into();
                module_path.push(name.clone());
                module = module
                    .modules
                    .entry(name.clone())
                    .or_insert_with(|| Module::new(module_path.clone()));
                if iter.peek().is_none() {
                    break module.file = Some(File::new(name, path.clone()));
                }
            }
        }

        Ok(Self { root })
    }

    pub fn generate(&self, out_dir: impl Into<PathBuf>) -> anyhow::Result<()> {
        self.root.generate(out_dir.into())
    }
}

#[derive(Debug, PartialEq)]
struct Module {
    module_path: Vec<Name>,
    modules: BTreeMap<Name, Module>,
    file: Option<File>,
}

impl Module {
    fn new(module_path: Vec<Name>) -> Self {
        Self { module_path, modules: Default::default(), file: Default::default() }
    }

    fn is_well_known_type(&self) -> bool {
        self.module_path.len() == 2
            && self.module_path[0].unescaped() == "google"
            && self.module_path[1].unescaped() == "protobuf"
    }

    fn feature(&self) -> Option<String> {
        self.file.as_ref().map(|_| {
            self.module_path.iter().map(|name| name.unescaped()).collect::<Vec<_>>().join("-")
        })
    }

    fn features(&self) -> BTreeSet<String> {
        let mut set = BTreeSet::from_iter(self.feature());
        for (_, module) in &self.modules {
            set.append(&mut module.features());
        }
        set
    }

    fn generate(&self, out_dir: PathBuf) -> anyhow::Result<()> {
        let mut items = Items::default();

        for (name, module) in &self.modules {
            if !module.is_well_known_type() {
                items.push(Item::Module(name.escaped().into(), module.features()));
                module.generate(out_dir.join(name.unescaped()))?;
            }
        }

        if let Some(ref file) = self.file {
            items.push(Item::Items(file.read()?));
        }

        let name = if self.module_path.is_empty() { "lib.rs" } else { "mod.rs" };
        items.write(&out_dir, name)?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Name(String);

impl Name {
    fn escaped(&self) -> &str {
        &self.0
    }

    fn unescaped(&self) -> &str {
        self.0.trim_start_matches("r#")
    }
}

impl<T: Into<String>> From<T> for Name {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

#[derive(Default)]
struct Items(Vec<Item>);

impl Items {
    fn push(&mut self, item: Item) {
        self.0.push(item);
    }

    fn print(&self) -> anyhow::Result<String> {
        let mut output = String::new();
        let mut tmp = String::new();

        for item in &self.0 {
            match item {
                Item::Module(name, features) => {
                    tmp.clear();

                    writeln!(&mut tmp, "#[cfg(any(")?;
                    for feature in features {
                        writeln!(&mut tmp, "feature = \"{}\",", feature)?;
                    }
                    writeln!(&mut tmp, "))]")?;
                    writeln!(&mut tmp, "pub mod {};\n", name.escaped())?;

                    // https://github.com/hyperium/tonic/issues/890
                    let file = syn::parse_file(&tmp)?;
                    let pretty_code = prettyplease::unparse(&file);
                    output.push_str(&pretty_code);
                }
                Item::Items(code) => output.write_str(code)?,
            }
        }

        Ok(output)
    }

    fn write(&self, out_dir: &Path, file_name: &str) -> anyhow::Result<()> {
        if self.0.is_empty() {
            return Ok(());
        }
        fs::create_dir_all(out_dir)?;
        fs::write(out_dir.join(file_name), self.print()?).map_err(Into::into)
    }
}

#[derive(Debug, PartialEq)]
enum Item {
    Module(Name, BTreeSet<String>),
    Items(String),
}

#[derive(Debug, PartialEq)]
struct File {
    name: Name,
    path: PathBuf,
}

impl File {
    fn new(name: impl Into<Name>, path: impl Into<PathBuf>) -> Self {
        Self { name: name.into(), path: path.into() }
    }

    fn read(&self) -> anyhow::Result<String> {
        fs::read_to_string(&self.path).map_err(Into::into)
    }
}
