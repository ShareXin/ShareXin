# ShareXin  

[![Crates.io](https://img.shields.io/crates/v/sharexin.svg?)](https://crates.io/crates/sharexin)  
[![Github Stars](https://img.shields.io/github/stars/thebitstick/ShareXin.svg?)](https://github.com/thebitstick/ShareXin)  
[![Crates.io Downloads](https://img.shields.io/crates/d/sharexin.svg?)](https://crates.io/crates/sharexin)  

![Mastodon](https://raw.githubusercontent.com/thebitstick/ShareXin/master/ui-mastodon.png)
![Twitter](https://raw.githubusercontent.com/thebitstick/ShareXin/master/ui-twitter.png)  

## Requirements  
* maim  
* slop  
* imagemagick  
* [t](https://github.com/sferik/t) (for now)  
* [toot](https://github.com/ihabunek/toot) (for now)  

## Dependencies for compiling  
* gtk3  
* cairo  
* libnotify  
* pango  
* gdk-pixbuf2  
* atk  

### Binary works out of the box on  
- Ubuntu 16.04 and up  
- Fedora 25 and up  

## Features  
* Uploads to Twitter and Mastodon  
* Allows taking screenshots and saving them to files  
* Notifications via libnotify  
* GUI works with Wayland and X11  
* Screenshotting works with X11  
* Saves screenshots to folder in Pictures dir  

## Installation (via Github)  
1. `git clone https://github.com/ShareXin/`  
2. Mac Users: `brew install gtk+3`  
2. `cargo install`  
3. Login to Twitter and/or Mastodon using `t` and/or `toot`  
4. Explore `--help`  

## Installation (via Crates.io)  
1. Mac Users: `brew install gtk+3`  
1. `cargo install sharexin`  
2. Login to Twitter and/or Mastodon using `t` and/or `toot`  
3. Explore `--help`  

## `--help`  

```rust

sharexin 0.3.6 (2017 Jul 28)

Usage: sharexin <options> [destination] <image options>

Options:
  -h, --help	Display this help message and exit
  -V, --version	Print version info and exit
  -U, --upgrade	Check for new updates

Image Options:
  area		Grab an area of the screen instead of the entire screen
  window	Grab the current window instead of the entire screen
  full		Gran the entire screen

Destinations:
  toot		Upload to Mastodon (uses "toot")
  tweet		Upload to Twitter (uses "t")
  file		Only save file

Examples:
  sharexin toot
  sharexin tweet full
  sharexin file window
  sharexin toot area
```  

## Language support  

#### English  
#### Français by [@Eleoryth](https://twitter.com/Eleoryth)  
#### Español  
#### Esperanto  
#### 简体中文  
#### 繁體中文  
#### 日本語  
#### 한국어  
#### Deutsche by [@qwertxzy](https://twitter.com/qwertxzy)  

## Known issues  
#### If `t` won't send your tweet  
Check your $PATH. Your terminal may be able to launch it, but your WM/DE may not.  
If you have a WM like i3, you can add it to your PATH in your `.xprofile`.  

#### If `sharexin` isn't found  
Add `$HOME/.cargo/bin` to your `$PATH`.  

#### If `t` takes forever to send a tweet  
Remember that it's only a Ruby app...  

#### No command line parameters work on Mac
I can't test Mac at the moment, all I know is that it compiles.    

## Changelog  
#### [0.3.6] - 2017-07-28  
- Character count now turns red when over the limit  
- You can't send a toot or tweet if it's over the limit  
- Double the ImageMagick shadow  
- No longer using Pipers crate  
- Changed those weird "mort" and "morti" variables  
- Commented most of the code, cleaned up  

#### [0.3.5] - 2017-07-27  
- Added button to check your image  
- Added upgrade checker  
- Added character count  
- Beautified code  
- New command line args  

#### [0.3.4] - 2017-07-26  
- Window screenshot adds shadow  
- Separated main.rs function gui()  

#### [0.3.3] - 2017-07-26  
- New UI thanks to Glade  
- Notifications now have language support  
- Pressing Control+Return sends your message  
- Native Buttons, language changes!  

#### [0.3.2] - 2017-07-25  
- Cleaned help messages, used GNU coreutil messages for some items  

#### [0.3.1] - 2017-07-25  
- Separated main.rs into {main, help, file, send}.rs, less scrolling  

#### [0.3.0] - 2017-07-25  
- Die Deutsche Sprache!  
- Falsche Kompilierungsdatum  

#### [0.2.9] - 2017-07-25  
- Multaj Lingovj! Added French, Spanish, Esperanto, and Japanese translations for --help!  

#### [0.2.8] - 2017-07-25  
- Maim replacing Gnome-screenshot  
- Shadow added to area screenshots using ImageMagick  
- Notification adds back tweet text  

#### [0.2.7] - 2017-07-24  
- Notifications via libnotify (bye bye dbus)  
- Username gotten by $USER var, rather than an entire library (thanks std!)  

#### [0.2.6] - 2017-07-23  
- Forgot to update the version # for 0.2.5 from 0.2.4 and Crates wouldn't allow a reupload so....  

#### [0.2.5] - 2017-07-23  
- Better word wrap  
- Better temp dir  
- Notification actually shows image now  

#### [0.2.4] - 2017-07-21  
- Added version info  
- Made --help prettier  

#### [0.2.3] - 2017-07-21  
- Send button now says Toot or Tweet depending on where you're going  
- TextView no longer accepts tabs  

#### [0.2.2] - 2017-07-21  
- TextView now word wraps  
- Ability to simply just tweet without an image  
- Mort  

#### [0.2.1] - 2017-07-20  
- Centered window (why isn't .set_position() IN THE DOCS)  

#### [0.2.0] - 2017-07-20  
- Uh, if you're haven problems with t not loadin', check your $PATH  

#### [0.1.0] - 2017-07-19  
- Bug fixes and improvements  

#### [0.0.0] - 2017-07-19  
##### Added  
- First commit  