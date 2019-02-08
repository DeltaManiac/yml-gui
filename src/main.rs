#[macro_use]
extern crate serde_derive;
use std::fs::File;
use svg::node::element::*;
use svg::Document;

const RECT_WIDTH: u32 = 150;
const RECT_HEIGHT: u32 = 200;
const RECT_PADDING: u32 = 10;
const TEXT_PADDING_LEFT: u32 = RECT_PADDING + 20;
const TEXT_PADDING_TOP: u32 = RECT_PADDING + 20;

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

fn load_yml() -> Manifest {
    let f = File::open("sample/zookeeper.yml").expect("Couldnt open file");
    let m: Manifest = serde_yaml::from_reader(f).unwrap();
    m
}

fn draw_svg(manifest: &Manifest) {
    let background = Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "black ");

    let mut document = Document::new()
        .set("manifest", (0, 0, 100, 70))
        .add(background);

    for (idx, instance) in (&manifest.instance_groups).iter().enumerate() {
        let group = Group::new();
        let instance_rect = Rectangle::new()
            .set(
                "x",
                10 + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32),
            )
            //.set("y", 10 + RECT_HEIGHT * (idx as u8) + RECT_PADDING)
            .set("y", RECT_PADDING)
            .set("width", RECT_WIDTH)
            .set("height", RECT_HEIGHT)
            .set("fill", "lightblue");

        let instance_name_text = Text::new()
            .set("fill", "yellow")
            .set(
                "x",
                TEXT_PADDING_TOP + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32),
            )
            .set("y", TEXT_PADDING_LEFT)
            .add(svg::node::Text::new(instance.name.clone()));

        let mut zones: String = "[ ".to_string();
        for az in &instance.azs {
            zones.push_str(&az[..]);
            zones.push_str(" ");
        }
        zones.push_str("]");
        let instance_azs_text = Text::new()
            .set("fill", "yellow")
            .set(
                "x",
                TEXT_PADDING_LEFT + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32),
            )
            .set("y", TEXT_PADDING_TOP * 2)
            .add(svg::node::Text::new(zones.clone()));
        document = document
            .add(group.add(instance_rect).add(instance_name_text))
            .add(instance_azs_text);
    }
    svg::save("image.svg", &document).unwrap();
}

fn main() {
    //load_yml();
    let manifest = load_yml();
    draw_svg(&manifest);
}
