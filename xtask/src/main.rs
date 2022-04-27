use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{manifest::Manifest, package::Package, proto::Protos};

mod manifest;
mod package;
mod proto;
mod utils;

#[derive(Clone)]
struct Opt {
    input_proto_dir: PathBuf,
    output_crate_dir: PathBuf,
    temp_dir: PathBuf,
}

impl Opt {
    fn manifest_path(&self) -> PathBuf {
        self.output_crate_dir.join("Cargo.toml")
    }

    fn output_package_src(&self) -> PathBuf {
        self.output_crate_dir.join("src")
    }
}

fn main() -> anyhow::Result<()> {
    let opt = Opt {
        input_proto_dir: PathBuf::from("xtask/proto"),
        output_crate_dir: PathBuf::from("google-api-proto"),
        temp_dir: PathBuf::from("xtask/temp"),
    };

    let mut args = env::args().skip(1);
    while let Some(target) = args.next() {
        match target.as_str() {
            "all" => {
                clean(&opt)?;
                generate(&opt)?;
                test(opt.manifest_path())?;
            }
            "generate" => generate(&opt)?,
            "test" => test(opt.manifest_path())?,
            "clean" => clean(&opt)?,
            _ => help()?,
        }
    }

    Ok(())
}

fn help() -> anyhow::Result<()> {
    Ok(()) // TODO: impl
}

fn clean(opt: &Opt) -> anyhow::Result<()> {
    let _ = utils::cleanup(&opt.temp_dir, &[]);
    let _ = utils::cleanup(opt.output_package_src(), &[]);
    Ok(())
}

fn generate(opt: &Opt) -> anyhow::Result<()> {
    let protos = Protos::from_dir(&opt.input_proto_dir)?;
    Manifest::read(opt.manifest_path())?.overwrite_features(protos.feature_names()?).write()?;

    let mut config = prost_build::Config::new();
    config.btree_map(&["."]).bytes(&["."]).protoc_arg("--experimental_allow_proto3_optional");
    tonic_build::configure().build_server(false).out_dir(&opt.temp_dir).compile_with_config(
        config,
        &protos.proto_paths(),
        &[&opt.input_proto_dir],
    )?;

    Package::from_dir(&opt.temp_dir)?.generate(opt.output_package_src())?;

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
