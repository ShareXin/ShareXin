use language;
use upgrade;
use twitter;
use mastodon;
use tray;
use sharexin::*;
use yaml_rust::YamlLoader;

use clap::{Arg, App, SubCommand, AppSettings};

pub fn cmd() {

    let file = language::loader();
    let locators = YamlLoader::load_from_str(file).unwrap();
    let locator = &locators[0]["Help"];
    let help = &locator["Help"].as_str().unwrap();
    let version = &locator["Version"].as_str().unwrap();
    let upgrade = &locator["Upgrade"].as_str().unwrap();
    let tray = &locator["Tray"].as_str().unwrap();
    let area = &locator["Area"].as_str().unwrap();
    let window = &locator["Window"].as_str().unwrap();
    let full = &locator["Full"].as_str().unwrap();
    let file = &locator["File"].as_str().unwrap();
    let mastodon = &locator["Toot"].as_str().unwrap();
    let twitter = &locator["Tweet"].as_str().unwrap();
    let imgur = &locator["Imgur"].as_str().unwrap();
    let twitter_auth = &locator["Twitter"]["Auth"].as_str().unwrap();
    let mastodon_auth = &locator["Mastodon"]["Auth"].as_str().unwrap();

    let mut sharexin = App::new("sharexin")
        .version(crate_version!())
        .author(crate_authors!())
        .about("ShareX for Linux, FreeBSD, and macOS")
        .help_message(help.to_owned())
        .version_message(version.to_owned())
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::ColoredHelp)
        .version_short("v")
        .arg(
            Arg::with_name("upgrade")
                .short("U")
                .long("upgrade")
                .help(upgrade)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("tray")
                .short("t")
                .long("tray")
                .help(tray)
                .takes_value(false),
        )
        .subcommand(
            SubCommand::with_name("toot")
                .about(mastodon.to_owned())
                .arg(
                    Arg::with_name("file")
                        .help(file)
                        .short("f")
                        .long("file")
                        .help(file)
                        .takes_value(true),
                )
                .subcommand(SubCommand::with_name("auth").about(
                    mastodon_auth.to_owned(),
                ))
                .subcommand(SubCommand::with_name("area").about(area.to_owned()))
                .subcommand(SubCommand::with_name("window").about(window.to_owned()))
                .subcommand(SubCommand::with_name("full").about(full.to_owned())),
        )
        .subcommand(
            SubCommand::with_name("tweet")
                .about(twitter.to_owned())
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
                .about(imgur.to_owned())
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

    if matches.is_present("upgrade") {
        upgrade::upgrade();
    } else if matches.is_present("tray") {
        tray::tray();
    }

    match matches.subcommand() {
        ("toot", Some(toot_matches)) => {
            match toot_matches.subcommand_name() {
                Some("area") => toot_area(),
                Some("window") => toot_window(),
                Some("full") => toot_full(),
                Some("auth") => mastodon::auth(),
                _ => {
                    if toot_matches.is_present("file") {
                        if let Some(file) = toot_matches.value_of("file") {
                            toot_file(file.to_owned());
                        }
                    } else {
                        toot();
                    }
                }
            }
        }
        ("tweet", Some(tweet_matches)) => {
            match tweet_matches.subcommand_name() {
                Some("area") => tweet_area(),
                Some("window") => tweet_window(),
                Some("full") => tweet_full(),
                Some("auth") => twitter::auth(),
                _ => {
                    if tweet_matches.is_present("file") {
                        if let Some(file) = tweet_matches.value_of("file") {
                            tweet_file(file.to_owned());
                        }
                    } else {
                        tweet();
                    }
                }
            }
        }
        ("imgur", Some(imgur_matches)) => {
            match imgur_matches.subcommand_name() {
                Some("area") => imgur_area(),
                Some("window") => imgur_window(),
                Some("full") => imgur_full(),
                _ => {
                    if imgur_matches.is_present("file") {
                        if let Some(file) = imgur_matches.value_of("file") {
                            imgur_file(file.to_owned());
                        }
                    } else {
                        sharexin.print_help().unwrap();
                    }
                }
            }
        }
        _ => sharexin.print_help().unwrap(),
    }
}
