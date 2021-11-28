use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{manifest::Manifest, package::Package, proto::Protos};

mod manifest;
mod package;
mod proto;

#[derive(Clone)]
struct Opt {
    input_proto_dir: PathBuf,
    output_package_dir: PathBuf,
    tmp: PathBuf,
}

impl Opt {
    fn manifest_path(&self) -> PathBuf {
        package::join_path(self.output_package_dir.clone(), "Cargo.toml")
    }

    fn output_package_src(&self) -> PathBuf {
        package::join_path(self.output_package_dir.clone(), "src")
    }
}

fn main() -> anyhow::Result<()> {
    let opt = Opt {
        input_proto_dir: PathBuf::from("xtask/proto"),
        output_package_dir: PathBuf::from("google-api-proto"),
        tmp: PathBuf::from("xtask/tmp"),
    };

    match env::args().nth(1) {
        Some(command) => match command.as_str() {
            "all" => {
                // clean
                let _ = cleanup(&opt.tmp, &[]);
                let _ = cleanup(&opt.output_package_src(), &[]);
                // generate
                generate(opt.clone())?;
                // test
                test(opt.manifest_path())
            }
            "generate" => generate(opt),
            "test" => test(opt.manifest_path()),
            "clean" => {
                let _ = cleanup(&opt.tmp, &[]);
                let _ = cleanup(&opt.output_package_src(), &[]);
                Ok(())
            }
            _ => help(),
        },
        _ => help(),
    }
}

fn help() -> anyhow::Result<()> {
    Ok(()) // TODO: impl
}

fn cleanup(path: impl AsRef<Path>, exclude: &'static [&'static str]) -> anyhow::Result<()> {
    if !path.as_ref().exists() {
        return fs::create_dir(path).map_err(Into::into);
    }

    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        match path.components().rev().next().and_then(|c| c.as_os_str().to_str()) {
            Some(path) if exclude.contains(&path) => {}
            _ => {
                if path.metadata()?.file_type().is_dir() {
                    fs::remove_dir_all(path)?
                } else {
                    fs::remove_file(path)?
                }
            }
        }
    }

    Ok(())
}

fn generate(opt: Opt) -> anyhow::Result<()> {
    let protos = Protos::from_dir(&opt.input_proto_dir)?;
    Manifest::read(opt.manifest_path())?.set_features(protos.feature_names()?).write()?;

    let mut config = prost_build::Config::new();
    config.btree_map(&["."]).bytes(&["."]).protoc_arg("--experimental_allow_proto3_optional");
    tonic_build::configure()
        .build_server(false)
        .format(false)
        .out_dir(&opt.tmp)
        .compile_with_config(config, &protos.proto_paths(), &[&opt.input_proto_dir])?;

    Package::from_dir(&opt.tmp)?.generate(opt.output_package_src())?;

    tonic_build::fmt(opt.output_package_src().to_str().unwrap());

    Ok(())
}

fn test(manifest_path: impl AsRef<Path>) -> anyhow::Result<()> {
    let features = Manifest::read(manifest_path)?.list_feature()?;
    let count = features.len();
    for (index, feature) in features.into_iter().enumerate() {
        println!("feature({:>3}/{:>3}): {}", index + 1, count, feature);
        let state = Command::new("cargo")
            .arg("build")
            .arg("--package")
            .arg("google-api-proto")
            .arg("--features")
            .arg(&feature)
            .status()?;
        if !state.success() {
            return Err(anyhow::anyhow!("build failre: feature={}", feature));
        }
    }
    Ok(())
}
