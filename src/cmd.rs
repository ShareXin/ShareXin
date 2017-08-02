use Destination;
use std::*;
use gui::gui;
use help;
use file;
use imgur;
use VERSION;
use open;

pub fn cmd()
{
    let twitter = Destination::new(1);
    let mastodon = Destination::new(0);

    // fetches user arguments
    let args: Vec<_> = env::args().collect();

    // since the actual command itself counts as an arguments, this checks if at least one non-command argument is made
    if args.len() >= 2 {

        // checks the first arguments
        match args[1].as_ref() {
            "-V" | "--version" | "version" => println!("sharexin {}", VERSION),
            "-U" | "--upgrade" | "upgrade" => help::upgrade(),

            // if the first arg is for mastodon, it checks for an acceptable argument
            "toot" => {
                if args.len() == 3 {
                    match args[2].as_ref() {
                        "area" | "-a" => {
                            file::image(String::from("-s"));
                            gui(mastodon, true);
                        },
                        "window" | "-w" => {
                            file::image(String::from("-i"));
                            gui(mastodon, true);
                        },
                        "full" => {
                            file::image(String::new());
                            gui(mastodon, true);
                        },
                        _ => help::help()
                    }
                }
                else if args.len() >= 3 {
                    match args[2].as_ref() {
                        "open" => { 
                            if args.len() == 4 {
                                file::open(args[3].clone());
                                gui(mastodon, true);
                            }
                            else {help::help();}
                        },
                        _ => help::help()
                    }
                }
                // if no additional argument is passed, then the user only wants to post an imageless toot
                else {gui(mastodon, false);}
            },
            // if the first arg is for twitter, it checks for an acceptable argument
            "tweet" => {
                if args.len() == 3 {
                    match args[2].as_ref() {
                        "area" | "-a" => {
                            file::image(String::from("-s"));
                            gui(twitter, true);
                        },
                        "window" | "-w" => {
                            file::image(String::from("-i"));
                            gui(twitter, true);
                        },
                        "full" => {
                            file::image(String::new());
                            gui(twitter, true);
                        },
                        "auth" => println!("Not implemented yet."),
                        _ => help::help()
                    }
                }
                else if args.len() >= 3 {
                    match args[2].as_ref() {
                        "open" => { 
                            if args.len() == 4 {
                                file::open(args[3].clone());
                                gui(twitter, true);
                            }
                            else {help::help();}
                        },
                        _ => help::help()
                    }
                }
                // if no additonal argument is passed, then the user only wants to post an imageless tweet
                else {gui(twitter, false);}
            },
            // if the first arg is imgur, it checks for an acceptable argument
            "imgur" => {
                if args.len() == 3 {
                    match args[2].as_ref() {
                        "area" | "-a" => {
                            file::image(String::from("-s"));
                            imgur::send();
                            },
                        "window" | "-w" => {
                            file::image(String::from("-i"));
                            imgur::send();
                        },
                        "full" => {
                            file::image(String::new());
                            imgur::send();
                        },
                        _ => help::help()
                    }
                }
                else if args.len() >= 3 {
                    match args[2].as_ref() {
                        "open" => {
                            if args.len() == 4 {
                                file::open(args[3].clone());
                                imgur::send();
                            }
                            else {help::help();}
                        },
                        _ => help::help()
                    }
                }
                else {
                    help::help();
                }
            },
            // if the first arg is file, it checks for an acceptable argument
            "file" => {
                if args.len() == 3 {
                    match args[2].as_ref() {
                        "area" | "-a" => file::image(String::from("-s")),
                        "window" | "-w" => file::image(String::from("-i")),
                        "full" => file::image(String::new()),
                        _ => help::help()
                    }
                }
                else if args.len() >= 3 {
                    match args[2].as_ref() {
                        "open" => { 
                            if args.len() == 4 {
                                file::open(args[3].clone());
                                match open::that(args[3].clone()) {
                                    Ok(ok) => println!("Opening: {}{:?}", args[3], ok),
                                    Err(e) => panic!("Could not open. {:?}", e),
                                };
                            }
                            else {help::help();}
                        },
                        _ => help::help()
                    }
                }
            },
            // if no argument is passed, then show user help
            _ => help::help()
        }
    }
    // if only the command is passed, or if somehow there are no arguments, then show user help
    else {help::help();}
}