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
        .set("fill", "white");

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
            .set("fill", "lightblue")
            .set("stroke","black");

        let instance_name_text = Text::new()
            .set("fill", "black")
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
        let seperator = Line::new()
            .set("fill", "black")
            .set(
                "x1",
                10 + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32),
            )
            .set("y1", TEXT_PADDING_TOP * 1 + TEXT_PADDING_TOP / 4)
            .set(
                "x2",
                10 + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32) + RECT_WIDTH,
            )
            .set("y2", TEXT_PADDING_TOP * 1 + TEXT_PADDING_TOP / 4)
            .set("stroke","black");
        let seperator2 = Line::new()
            .set("fill", "black")
            .set(
                "x1",
                10 + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32),
            )
            .set("y1", TEXT_PADDING_TOP * 1 + TEXT_PADDING_TOP / 4 +3)
            .set(
                "x2",
                10 + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32) + RECT_WIDTH,
            )
            .set("y2", TEXT_PADDING_TOP * 1 + TEXT_PADDING_TOP / 4 + 3)
            .set("stroke","black");
            //.set("pathLength", RECT_WIDTH);
        let instance_azs_text = Text::new()
            .set("fill", "black")
            .set(
                "x",
                TEXT_PADDING_LEFT + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32),
            )
            .set("y", TEXT_PADDING_TOP * 2)
            .add(svg::node::Text::new(zones.clone()));
        document = document
            .add(group.add(instance_rect).add(instance_name_text))
            .add(seperator)
            .add(seperator2)
            .add(instance_azs_text);
    }
    svg::save("image.svg", &document).unwrap();
}

fn draw_svg_2(manifest:&Manifest){
    
}


fn calculate(canvas_width:u16,canvas_height:u16,instance_height:u16, instance_width:u16,instance_count:u16, instance_padding:u16) {
    let canvas_estate = canvas_height*canvas_width;
    let instance_estate = instance_height*instance_height + (4*instance_padding );
    let canvas_max_instance_count = canvas_estate/instance_estate;
    let canvas_max_row_count = canvas_height/(instance_height + 2*instance_padding);
    let canvas_max_col_count = canvas_width/(instance_width + 2*instance_padding);

    println!("==============");
    println!("Instance Dimensions:");
    println!("Height: {}",instance_height);
    println!("Width: {}",instance_width);
    println!("Size: {}",instance_estate);
    println!("\nCanvas Dimensions:");
    println!("Height: {}",canvas_height);
    println!("Width: {}",canvas_width);
    println!("Size: {}",canvas_estate);
    println!("\nMax Fit Count: {}",canvas_max_instance_count);
    println!("Max Row Count: {}",canvas_max_row_count);
    println!("Max Col Count: {}",canvas_max_col_count);

    println!("\nInstance Count: {}",instance_count);
    println!("==============");

}
fn main() {
    calculate(100,100,100,100,1,0);
    calculate(200,200,100,100,1,0);

    calculate(100,100,100,100,1,10);
    calculate(200,200,100,100,1,10);
    //load_yml();
    //let manifest = load_yml();
    //draw_svg(&manifest);
    println!("image generated at : image.svg")
}
