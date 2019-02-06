#[macro_use]
extern crate serde_derive;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Serialize, Deserialize,  Debug)]
struct Instance {
    name: String,
    azs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Features {
    converge_variable: Option<bool>,
    randomize_az_placement: Option<bool>,
    use_dns_address: Option<bool>,
    use_tmpfs_job_config: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Stemcell {
    os: String,
    version: String,
}
#[derive(Serialize, Deserialize,  Debug)]
struct Release {
    name: String,
    version: String,
    url: Option<String>,
    sha1: Option<String>,
    stemcell: Option<Stemcell>,
}

#[derive(Serialize, Deserialize,  Debug)]
struct Manifest {
    instance_groups: Vec<Instance>,
    name: String,
    features: Option<Features>,
}

fn load_yml() -> io::Result<()> {
    let mut f = File::open("sample/zookeeper.yml")?;
    let c: Manifest = serde_yaml::from_reader(f).unwrap();

    dbg!(&c);

    Ok(())
}

fn main() {
    load_yml();
}
