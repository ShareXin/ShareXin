mod dialog;
mod image;
mod imgur;
mod mastodon;
mod notification;
mod text;
mod twitter;

use clap::{crate_authors, crate_version, App, AppSettings, Arg, SubCommand};
use screenshot_rs::ScreenshotKind;
use yaml_rust::YamlLoader;

#[derive(Copy, Clone)]
pub enum ServiceKind {
    Twitter,
    Mastodon,
    Imgur,
}

#[derive(Copy, Clone, PartialEq)]
pub enum MessageKind {
    Image,
    Text,
}

fn main() {
    // Individual parts the help menu
    let locators = YamlLoader::load_from_str(&text::loader()).unwrap();
    let locator = &locators[0]["Help"];
    let help = &locator["Help"].as_str().unwrap();
    let version = &locator["Version"].as_str().unwrap();
    let area = &locator["Area"].as_str().unwrap();
    let window = &locator["Window"].as_str().unwrap();
    let full = &locator["Full"].as_str().unwrap();
    let file = &locator["File"].as_str().unwrap();
    let mastodon = &locator["Toot"].as_str().unwrap();
    let twitter = &locator["Tweet"].as_str().unwrap();
    let imgur = &locator["Imgur"].as_str().unwrap();
    let twitter_auth = &locator["Twitter"]["Auth"].as_str().unwrap();
    let mastodon_auth = &locator["Mastodon"]["Auth"].as_str().unwrap();

    // Build help menu with clap.rs
    let mut sharexin = App::new("sharexin")
        .version(crate_version!())
        .author(crate_authors!())
        .about("ShareX for Linux and FreeBSD")
        .help_message(help.to_owned())
        .version_message(version.to_owned())
        .setting(AppSettings::DisableHelpSubcommand)
        .version_short("v")
        .subcommand(
            SubCommand::with_name("toot")
                .version(crate_version!())
                .author(crate_authors!())
                .about(mastodon.to_owned())
                .help_message(help.to_owned())
                .version_message(version.to_owned())
                .version_short("v")
                .arg(
                    Arg::with_name("file")
                        .help(file)
                        .short("f")
                        .long("file")
                        .help(file)
                        .takes_value(true),
                )
                .subcommand(SubCommand::with_name("auth").about(mastodon_auth.to_owned()))
                .subcommand(SubCommand::with_name("area").about(area.to_owned()))
                .subcommand(SubCommand::with_name("window").about(window.to_owned()))
                .subcommand(SubCommand::with_name("full").about(full.to_owned())),
        )
        .subcommand(
            SubCommand::with_name("tweet")
                .version(crate_version!())
                .author(crate_authors!())
                .about(twitter.to_owned())
                .help_message(help.to_owned())
                .version_message(version.to_owned())
                .version_short("v")
                .arg(
                    Arg::with_name("file")
                        .help(file)
                        .short("f")
                        .long("file")
                        .help(file)
                        .takes_value(true),
                )
                .subcommand(SubCommand::with_name("auth").about(twitter_auth.to_owned()))
                .subcommand(SubCommand::with_name("area").about(area.to_owned()))
                .subcommand(SubCommand::with_name("window").about(window.to_owned()))
                .subcommand(SubCommand::with_name("full").about(full.to_owned())),
        )
        .subcommand(
            SubCommand::with_name("imgur")
                .version(crate_version!())
                .author(crate_authors!())
                .about(imgur.to_owned())
                .help_message(help.to_owned())
                .version_message(version.to_owned())
                .version_short("v")
                .arg(
                    Arg::with_name("file")
                        .help(file)
                        .short("f")
                        .long("file")
                        .help(file)
                        .takes_value(true),
                )
                .subcommand(SubCommand::with_name("area").about(area.to_owned()))
                .subcommand(SubCommand::with_name("window").about(window.to_owned()))
                .subcommand(SubCommand::with_name("full").about(full.to_owned())),
        );
    let matches = sharexin.clone().get_matches();

    match matches.subcommand() {
        ("toot", Some(toot_matches)) => match toot_matches.subcommand_name() {
            Some("area") => {
                image::image(ScreenshotKind::Area);
                dialog::dialog(ServiceKind::Mastodon, MessageKind::Image);
            }
            Some("window") => {
                image::image(ScreenshotKind::Window);
                dialog::dialog(ServiceKind::Mastodon, MessageKind::Image);
            }
            Some("full") => {
                image::image(ScreenshotKind::Full);
                dialog::dialog(ServiceKind::Mastodon, MessageKind::Image);
            }
            Some("auth") => mastodon::auth(),
            _ => {
                if toot_matches.is_present("file") {
                    if let Some(file) = toot_matches.value_of("file") {
                        image::file(file.to_string());
                        dialog::dialog(ServiceKind::Mastodon, MessageKind::Image);
                    }
                } else {
                    dialog::dialog(ServiceKind::Mastodon, MessageKind::Text);
                }
            }
        },
        ("tweet", Some(tweet_matches)) => match tweet_matches.subcommand_name() {
            Some("area") => {
                image::image(ScreenshotKind::Area);
                dialog::dialog(ServiceKind::Twitter, MessageKind::Image);
            }
            Some("window") => {
                image::image(ScreenshotKind::Window);
                dialog::dialog(ServiceKind::Twitter, MessageKind::Image);
            }
            Some("full") => {
                image::image(ScreenshotKind::Full);
                dialog::dialog(ServiceKind::Twitter, MessageKind::Image);
            }
            Some("auth") => twitter::auth(),
            _ => {
                if tweet_matches.is_present("file") {
                    if let Some(file) = tweet_matches.value_of("file") {
                        image::file(file.to_string());
                        dialog::dialog(ServiceKind::Twitter, MessageKind::Image);
                    }
                } else {
                    dialog::dialog(ServiceKind::Twitter, MessageKind::Text);
                }
            }
        },
        ("imgur", Some(imgur_matches)) => match imgur_matches.subcommand_name() {
            Some("area") => {
                image::image(ScreenshotKind::Area);
                imgur::send();
            }
            Some("window") => {
                image::image(ScreenshotKind::Window);
                imgur::send();
            }
            Some("full") => {
                image::image(ScreenshotKind::Full);
                imgur::send();
            }
            _ => {
                if imgur_matches.is_present("file") {
                    if let Some(file) = imgur_matches.value_of("file") {
                        image::file(file.to_string());
                        imgur::send();
                    }
                } else {
                    sharexin.print_help().unwrap();
                }
            }
        },
        _ => dialog::dialog(ServiceKind::Twitter, MessageKind::Text),
    }
}
