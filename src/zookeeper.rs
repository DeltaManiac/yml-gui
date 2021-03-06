use crate::ui::State;
use imgui::*;

#[derive(Debug)]
pub struct ZookeeperSettings {
    item_horizontal_spacing: f32,
    item_vertical_spacing: f32,
    text_padding: [f32; 2],
    image_padding: [f32; 4],
    image_color: [f32; 3],
    errand_inset: [f32; 2],
    errand_color: [f32; 3],
}
impl Default for ZookeeperSettings {
    fn default() -> Self {
        ZookeeperSettings {
            item_horizontal_spacing: 10.,
            item_vertical_spacing: 10.,
            text_padding: [1.; 2],
            image_padding: [1.; 4],
            image_color: [1., 0., 0.],
            errand_inset: [2.5, 2.5],
            errand_color: [0., 0., 1.],
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
    lifecycle: Option<String>,
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
    let zk = state.zookeeper_settings.as_mut().unwrap();
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
                    ui.slider_float2(
                        im_str!("Horizontal and Vertical Text Padding"),
                        &mut zk.text_padding,
                        -15.0,
                        25.0,
                    )
                    .build();
                    ui.slider_float2(
                        im_str!("Horizontal and Vertical Errand Inset"),
                        &mut zk.errand_inset,
                        0.,
                        15.0,
                    )
                    .build();
                    ui.color_edit(im_str!("Item Color"), &mut zk.image_color)
                        .build();
                    ui.color_edit(im_str!("Errand Color"), &mut zk.errand_color)
                        .build();
                });
            let draw_list = ui.get_window_draw_list();
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
    let mut x = pos.0 + zk.item_horizontal_spacing;
    let mut y = pos.1 + zk.item_vertical_spacing;
    let size: (f32, f32) = (
        (real_estate.0 / 2 as f32) - zk.item_horizontal_spacing,
        //+ zk.item_vertical_spacing),
        140.0,
    );
    for instance in (&manifest.instance_groups).iter() {
        let mut line_height = ui
            .calc_text_size(&ImString::new(instance.name.clone()), true, x + size.0)
            .y;
        draw_list
            .add_rect((x, y), (x + size.0, y + size.1), zk.image_color)
            .filled(true)
            .build();
        ui.set_cursor_screen_pos((x + zk.text_padding[0], y + zk.text_padding[1]));
        ui.with_text_wrap_pos(x + size.0, || ui.text(ImString::new(instance.name.clone())));
        draw_list
            .add_line(
                (x, y + line_height),
                (x + size.0, y + line_height),
                [0.0, 0.0, 0.],
            )
            .thickness(1.)
            .build();
        //Release
        let release = manifest
            .releases
            .iter()
            .find(|&x| x.name == instance.jobs[0].release)
            .unwrap();
        let mut txt = String::new();
        txt.push_str(&release.name);
        txt.push('(');
        txt.push_str(&release.version);
        txt.push(')');
        txt.push('/');
        txt.push_str(&instance.name);
        ui.set_cursor_screen_pos((x + zk.text_padding[0], y + line_height + zk.text_padding[1]));
        ui.with_text_wrap_pos(x + size.0, || ui.text(ImString::new(txt.clone())));
        line_height += ui
            .calc_text_size(&ImString::new(txt.clone()), true, x + size.0)
            .y;
        //AZS
        let mut zones: String = "[ ".to_string();
        for az in &instance.azs {
            zones.push_str(&az[..]);
            zones.push_str(" ");
        }
        zones.push_str("]");
        //line_height += ui
        //    .calc_text_size(&ImString::new(zones.clone()), true, x + size.0)
        //    .y;
        ui.set_cursor_screen_pos((x + zk.text_padding[0], y + line_height + zk.text_padding[1]));
        ui.with_text_wrap_pos(x + size.0, || ui.text(ImString::new(zones.clone())));
        line_height += ui
            .calc_text_size(&ImString::new(zones.clone()), true, x + size.0)
            .y;

        //StemCELLL
        let stemcell = manifest
            .stemcells
            .iter()
            .find(|&x| x.alias == instance.stemcell)
            .unwrap();
        let mut se = String::new();
        se.push_str(&stemcell.os.clone().unwrap());
        se.push(':');
        se.push_str(&stemcell.version.clone());
        ui.set_cursor_screen_pos((x + zk.text_padding[0], y + line_height + zk.text_padding[1]));
        ui.with_text_wrap_pos(x + size.0, || ui.text(ImString::new(se.clone())));
        line_height += ui
            .calc_text_size(&ImString::new(se.clone()), true, x + size.0)
            .y;

        //Start drawing border
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
        //End Drawing Border
        //Start drawing errand border
        if instance.lifecycle.is_some() {
            if instance.lifecycle.clone().unwrap_or(String::from("none")) == "errand" {
                let inset: (f32, f32) = (zk.errand_inset[0], zk.errand_inset[1]);
                draw_list
                    .add_line(
                        (x + inset.0, y + inset.1),
                        (x + inset.0, y + size.1 - inset.1),
                        zk.errand_color,
                    )
                    .build();
                draw_list
                    .add_line(
                        (x - inset.0 + size.0, y + inset.1),
                        (x - inset.0 + size.0, y + size.1 - inset.1),
                        zk.errand_color,
                    )
                    .build();
                draw_list
                    .add_line(
                        (x + inset.0, y - 1. + inset.1),
                        (x + size.0 - inset.0, y - 1. + inset.1),
                        zk.errand_color,
                    )
                    .build();
                draw_list
                    .add_line(
                        (x + inset.0, y + size.1 - inset.1),
                        (x + size.0 - inset.0, y + size.1 - inset.1),
                        zk.errand_color,
                    )
                    .build();
            }
        }
        //End drawing errand border
        x += size.0 + zk.item_horizontal_spacing;
        if x >= real_estate.0 {
            x = pos.0 + zk.item_horizontal_spacing;
            y = y + 140.0 + zk.item_vertical_spacing;
        }
    }
}
