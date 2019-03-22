use crate::ui::State;
use imgui::*;

#[derive(Debug)]
pub struct ZookeeperSettings {
    item_horizontal_spacing: f32,
    item_vertical_spacing: f32,
    text_padding_left: f32,
    text_padding_right: f32,
    text_padding_top: f32,
    text_padding_bottom: f32,
    image_padding_left: f32,
    image_padding_right: f32,
    image_padding_top: f32,
    image_padding_bottom: f32,
}
impl Default for ZookeeperSettings {
    fn default() -> Self {
        ZookeeperSettings {
            item_horizontal_spacing: 10.,
            item_vertical_spacing: 10.,
            text_padding_left: 5.,
            text_padding_right: 0.,
            text_padding_top: 0.,
            text_padding_bottom: 0.,
            image_padding_left: 0.,
            image_padding_right: 0.,
            image_padding_top: 0.,
            image_padding_bottom: 0.,
        }
    }
}
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

pub fn add_rects(ui: &Ui, state: &mut State) {
    if state.yml_str.is_none() {
        return;
    }
    if state.zookeeper_settings.is_none() {
        state.zookeeper_settings = Some(ZookeeperSettings::default());
    }
    let manifest: Manifest = serde_yaml::from_str(&state.yml_str.clone().unwrap()).unwrap();
    let mut zk = state.zookeeper_settings.as_mut().unwrap();
    ui.window(im_str!("Image"))
        //.title_bar(false)
        .position(ui.get_cursor_screen_pos(), ImGuiCond::FirstUseEver)
        //      .movable(false)
        .collapsible(false)
        //.always_auto_resize(true)
        //   .resizable(false)
        .scroll_bar(false)
        .menu_bar(true)
        .size((1024f32, 768f32), ImGuiCond::FirstUseEver)
        .build(|| {
            if ui.small_button(im_str!("Config...")) {
                ui.open_popup(im_str!("Config"))
            }
            ui.window(im_str!("Config"))
                .always_auto_resize(true)
                .collapsible(false)
                .build(|| {
                    ui.separator();
                    ui.slider_float(
                        im_str!("Vertical spacing"),
                        &mut zk.item_vertical_spacing,
                        1.0,
                        20.0,
                    )
                    .build();
                    ui.slider_float(
                        im_str!("Horizontal spacing"),
                        &mut zk.item_horizontal_spacing,
                        1.0,
                        20.0,
                    )
                    .build();
                    //.build();
                });
            let mut draw_list = ui.get_window_draw_list();
            draw_image(&manifest, ui, &draw_list, zk);
        });
}

fn draw_image(
    manifest: &Manifest,
    ui: &Ui,
    draw_list: &WindowDrawList,
    zk: &mut ZookeeperSettings,
) {
    let pos = ui.get_cursor_screen_pos();
    let real_estate = ui.get_content_region_avail();
    //let real_estate = ui.get_window_content_region_max();

    let mut x = pos.0 + zk.item_horizontal_spacing;
    let mut y = pos.1 + zk.item_vertical_spacing;
    let size: (f32, f32) = (
        (real_estate.0 / manifest.instance_groups.len() as f32) - zk.item_horizontal_spacing,
        //+ zk.item_vertical_spacing),
        140.0,
    );
    for (dx, instance) in (&manifest.instance_groups).iter().enumerate() {
        let a = ui.calc_text_size(&ImString::new(instance.name.clone()), true, x + size.0);
        draw_list
            .add_rect((x, y), (x + size.0, y + size.1), [0.8, 0.2, 0.5])
            .filled(true)
            .build();
        ui.set_cursor_screen_pos((x + 5., y));
        ui.with_text_wrap_pos(x + size.0, || ui.text(ImString::new(instance.name.clone())));
        draw_list
            .add_line((x, y), (x, y + size.1), [1., 1., 1.])
            .build();
        draw_list
            .add_line((x + size.0, y), (x + size.0, y + size.1), [1., 1., 1.])
            .build();
        draw_list
            .add_line((x, y - 1.), (x + size.0, y - 1.), [1., 1., 1.])
            .build();
        draw_list
            .add_line((x, y + size.1), (x + size.0, y + size.1), [1., 1., 1.])
            .build();
        draw_list
            .add_line((x, y + a.y), (x + size.0, y + a.y), [0.0, 0.0, 0.])
            .thickness(5.)
            .build();
        x += size.0 + zk.item_horizontal_spacing;
    }
}

// fn draw_svg(manifest: &Manifest) {
//     let background = Rectangle::new()
//         .set("width", "100%")
//         .set("height", "100%")
//         .set("fill", "white");

//     let mut document = Document::new()
//         .set("manifest", (0, 0, 100, 70))
//         .add(background);

//     for (idx, instance) in (&manifest.instance_groups).iter().enumerate() {
//         let group = Group::new();
//         let instance_rect = Rectangle::new()
//             .set(
//                 "x",
//                 10 + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32),
//             )
//             //.set("y", 10 + RECT_HEIGHT * (idx as u8) + RECT_PADDING)
//             .set("y", RECT_PADDING)
//             .set("width", RECT_WIDTH)
//             .set("height", RECT_HEIGHT)
//             .set("fill", "lightblue")
//             .set("stroke","black");

//         let instance_name_text = Text::new()
//             .set("fill", "black")
//             .set(
//                 "x",
//                 TEXT_PADDING_TOP + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32),
//             )
//             .set("y", TEXT_PADDING_LEFT)
//             .add(svg::node::Text::new(instance.name.clone()));

//         let mut zones: String = "[ ".to_string();
//         for az in &instance.azs {
//             zones.push_str(&az[..]);
//             zones.push_str(" ");
//         }
//         zones.push_str("]");
//         let seperator = Line::new()
//             .set("fill", "black")
//             .set(
//                 "x1",
//                 10 + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32),
//             )
//             .set("y1", TEXT_PADDING_TOP * 1 + TEXT_PADDING_TOP / 4)
//             .set(
//                 "x2",
//                 10 + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32) + RECT_WIDTH,
//             )
//             .set("y2", TEXT_PADDING_TOP * 1 + TEXT_PADDING_TOP / 4)
//             .set("stroke","black");
//         let seperator2 = Line::new()
//             .set("fill", "black")
//             .set(
//                 "x1",
//                 10 + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32),
//             )
//             .set("y1", TEXT_PADDING_TOP * 1 + TEXT_PADDING_TOP / 4 +3)
//             .set(
//                 "x2",
//                 10 + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32) + RECT_WIDTH,
//             )
//             .set("y2", TEXT_PADDING_TOP * 1 + TEXT_PADDING_TOP / 4 + 3)
//             .set("stroke","black");
//             //.set("pathLength", RECT_WIDTH);
//         let instance_azs_text = Text::new()
//             .set("fill", "black")
//             .set(
//                 "x",
//                 TEXT_PADDING_LEFT + RECT_WIDTH * (idx as u32) + RECT_PADDING * (idx as u32),
//             )
//             .set("y", TEXT_PADDING_TOP * 2)
//             .add(svg::node::Text::new(zones.clone()));
//         document = document
//             .add(group.add(instance_rect).add(instance_name_text))
//             .add(seperator)
//             .add(seperator2)
//             .add(instance_azs_text);
//     }
//     svg::save("image.svg", &document).unwrap();// }
