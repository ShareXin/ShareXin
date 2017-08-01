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
    //gets command args
    let args: Vec<_> = env::args().collect();
    //if at least one arg is passed (the command itself counts?? wtf)
    if args.len() >= 2 {
        //check the first arg
        match args[1].as_ref() {
            "-V" | "--version" | "version" => println!("sharexin {}", VERSION),
            "-U" | "--upgrade" | "upgrade" => help::upgrade(),
            "toot" => {
                if args.len() == 3 {
                    //if first arg is for mastodon, check if its an acceptable arg
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
                //if its only toot, then no image will be taken
                else {gui(mastodon, false);}
            },
            //if first arg is twitter
            "tweet" => {
                if args.len() == 3 {
                    //check any more args
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
                //or just tweet
                else {gui(twitter, false);}
            },
            //if first arg is imgur
            "imgur" => {
                if args.len() == 3 {
                    match args[2].as_ref() {
                        //check for args
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
            //if first arg is file
            "file" => {
                if args.len() == 3 {
                    match args[2].as_ref() {
                        //check for args
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
            //or else give them help, you cant save nothing.
            _ => help::help()
        }
    }
    //if no args at all
    else {help::help();}
}