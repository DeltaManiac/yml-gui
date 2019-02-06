#[macro_use]
extern crate serde_derive;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Job {
    name: String,
    release: String,
    // consumes:Option<String>
    // provides:Option<String>
    // properties:Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Network {
    name: String,
    static_ips: Option<Vec<String>>,
    default: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug)]
struct VMResource {
    cpu: i16,
    ram: i16,
    ephemeral_disk_size: i16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Instance {
    name: String,
    azs: Vec<String>,
    instances: i16,
    jobs: Vec<Job>,
    vm_type: String,
    vm_resources: Option<VMResource>,
    stemcell: String,
    persistent_disk: Option<i16>,
    persistent_disk_type: Option<String>,
    networks: Vec<Network>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Feature {
    converge_variable: Option<bool>,
    randomize_az_placement: Option<bool>,
    use_dns_address: Option<bool>,
    use_tmpfs_job_config: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stemcell {
    alias: String,
    os: Option<String>,
    version: String,
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Release {
    name: String,
    version: String,
    url: Option<String>,
    sha1: Option<String>,
    //stemcell: Option<Stemcell>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Manifest {
    instance_groups: Vec<Instance>,
    name: String,
    features: Option<Feature>,
    releases: Vec<Release>,
    stemcells: Vec<Stemcell>,
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
