use colors_transform::{Color, Rgb};
use gio;
use gio::prelude::SettingsExtManual;
use gio::SettingsExt;
use yansi;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use json;

const KEYS: &'static [&'static str] = &[
    "terminal.ansiBlack",
    "terminal.ansiBlue",
    "terminal.ansiCyan",
    "terminal.ansiGreen",
    "terminal.ansiMagenta",
    "terminal.ansiRed",
    "terminal.ansiWhite",
    "terminal.ansiYellow",
    "terminal.ansiBrightBlack",
    "terminal.ansiBrightBlue",
    "terminal.ansiBrightCyan",
    "terminal.ansiBrightGreen",
    "terminal.ansiBrightMagenta",
    "terminal.ansiBrightRed",
    "terminal.ansiBrightWhite",
    "terminal.ansiBrightYellow",
];

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
    let foreground_color = &terminal_profile.get_string("foreground-color").unwrap();
    let background_color = &terminal_profile.get_string("background-color").unwrap();
    let palette = &terminal_profile.get_strv("palette");
    println!("foreground-color {:?}", &foreground_color);
    println!("background-color {:?}", &background_color);
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

    println!("{}", "dumping to vscode");
    dump_theme_vscode(&vs, "beep.gnome-terminal-theme.json");
    println!("{}", "loading from vscode");
    let loaded = load_theme_vscode("beep.gnome-terminal-theme.json");
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

fn dump_theme_vscode(theme: &Vec<String>, filename: &str) {
    let path = Path::new(filename);
    let mut file = File::create(&path).unwrap();
    let mut data = json::JsonValue::new_object();
    data["workbench.colorCustomizations"] = json::JsonValue::new_object();
    data["workbench.colorCustomizations"]["terminal.foreground"] =
        json::JsonValue::String(theme[0].clone());
    data["workbench.colorCustomizations"]["terminal.background"] =
        json::JsonValue::String(theme[0].clone());

    KEYS.iter().enumerate().for_each(|(i, &key)| {
        let rgb = theme[i].clone();
        let hex = rgb.parse::<Rgb>().unwrap().to_css_hex_string();
        data["workbench.colorCustomizations"][key] = json::JsonValue::String(hex);
    });
    file.write_all(data.pretty(4).as_bytes()).unwrap();
}

fn load_theme_vscode(filename: &str) -> Vec<String> {
    let path = Path::new(filename);
    let mut data = String::new();
    let mut file = File::open(&path).unwrap();
    file.read_to_string(&mut data).unwrap();

    let parsed = json::parse(&data).unwrap();
    // println!("Loaded json: {}", parsed);
    KEYS.iter()
        .map(|&color| {
            let hex = parsed["workbench.colorCustomizations"][color].to_string();
            Rgb::from_hex_str(&hex).unwrap().to_css_string()
        })
        .collect::<Vec<String>>()
}
