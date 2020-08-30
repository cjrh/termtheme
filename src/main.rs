use colors_transform::{Color, Rgb};
use gio;
use gio::prelude::SettingsExtManual;
use gio::SettingsExt;
use yansi;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use json;

#[derive(structopt::StructOpt)]
struct Args {
    #[structopt(short, long)]
    theme: String,
}

#[paw::main]
fn main(args: Args) {
    println!("{}", &args.theme);

    let profiles_list = "org.gnome.Terminal.ProfilesList";
    let settings = gio::Settings::new(&profiles_list);
    let default_terminal_profile: String = settings.get("default");
    println!("{:?}", &default_terminal_profile);

    let schema = "org.gnome.Terminal.Legacy.Profile";
    let path = format!(
        "/org/gnome/terminal/legacy/profiles:/:{}/",
        &default_terminal_profile
    );
    let terminal_profile = gio::Settings::with_path(
        &schema,
        &path, // "/org/gnome/terminal/legacy/profiles:/:5db53530-b692-4860-95df-0fef50b1d52c/",
    );
    println!("{:?}", &terminal_profile);
    let palette = &terminal_profile.get_strv("palette");
    println!("palette {:?}", &palette);

    for c in palette {
        println!("{}", &c)
    }
    println!();

    for c in palette {
        let rgb = &c.parse::<Rgb>().unwrap();
        println!(
            "{}",
            yansi::Paint::rgb(
                rgb.get_red() as u8,
                rgb.get_green() as u8,
                rgb.get_blue() as u8,
                &c
            )
        );
    }

    let vs = palette
        .iter()
        .map(|g| g.to_string())
        .collect::<Vec<String>>();
    dump_theme(&vs, "beep.gnome-terminal-theme");

    println!("Loading the file again:");
    let loaded = load_theme("beep.gnome-terminal-theme");
    println!("{:?}", &loaded);
}

fn dump_theme(theme: &Vec<String>, filename: &str) {
    let path = Path::new(filename);
    let mut file = File::create(&path).unwrap();
    let data = theme.join("\t");
    file.write_all(data.as_bytes()).unwrap();
}

fn load_theme(filename: &str) -> Vec<String> {
    let path = Path::new(filename);
    let mut data = String::new();
    let mut file = File::open(&path).unwrap();
    file.read_to_string(&mut data).unwrap();
    data.split("\t").map(str::to_string).collect()
}
