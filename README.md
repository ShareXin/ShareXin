# ShareXin  

[![Crates.io](https://img.shields.io/crates/v/sharexin.svg?)](https://crates.io/crates/sharexin)
[![Github Stars](https://img.shields.io/github/stars/thebitstick/ShareXin.svg?)](https://github.com/thebitstick/ShareXin)
[![Crates.io Downloads](https://img.shields.io/crates/d/sharexin.svg?)](https://crates.io/crates/sharexin)  

## Screenshots
![Mastodon](https://raw.githubusercontent.com/thebitstick/ShareXin/master/ui-mastodon.png)
![Twitter](https://raw.githubusercontent.com/thebitstick/ShareXin/master/ui-twitter.png)  

## Requirements  
* **Rust Nightly**  
* Unix-like system  
* xdg (probably already installed)  
* scrot (on non-Gnome/KDE x11 desktops)  
* imagemagick  
* openssl  
* [t](https://github.com/sferik/t) (for now)  
* [toot](https://github.com/ihabunek/toot) (for now)  

## Features  
* Uploads to Twitter and Mastodon and Imgur  
* Allows taking screenshots and saving them to files  
* Notifications via dbus  
* GUI works with GTK  
* Screenshotting works with X11 and Wayland (on supported desktops)  
* Saves screenshots to a folder in your Pictures directory  

## `--help`  

```bash
sharexin 0.5.0
Usage: sharexin <options> [destination] [image options] [FILE]

Options:
  -h, --help	Display this help message and exit
  -V, --version	Print version info and exit
  -U, --upgrade	Check for new updates

Image Options:
  area		Grab an area of the screen instead of the entire screen
  window	Grab the current window instead of the entire screen
  full		Grab the entire screen
  file		Use a file

Destinations:
  toot		Upload to Mastodon (uses "toot")
  tweet		Upload to Twitter (uses "t")
  imgur		Upload to Imgur

Examples:
  sharexin toot
  sharexin tweet full
  sharexin toot area
  sharexin imgur file [FILE]
```  

## Language support  

### Now accepting language template files in this [form](https://goo.gl/forms/rNx4yAB9KM2fDXDG3)  

#### English  
#### Français by [@Eleoryth](https://twitter.com/Eleoryth)  
#### Español  
#### Esperanto  
#### 简体中文  
#### 繁體中文  
#### 日本語  
#### 한국어  
#### Deutsch by [@qwertxzy](https://twitter.com/qwertxzy)  

## Compiling

### Dependencies for compiling  
* gtk3  
* cairo  
* libnotify  
* pango  
* gdk-pixbuf2  
* atk  
* openssl  

### Ubuntu dependencies  
* libgtk-3-dev  
* libcairo2-dev  
* libnotify-dev  
* libpango1.0-dev  
* libgdk-pixbuf2.0-dev  
* libatk1.0-dev  
* libssl1.0-dev  
* libssl-dev  

### Fedora dependencies  
* gtk3-devel  
* cairo-devel  
* libnotify-devel  
* pango-devel  
* gdk-pixbuf2-devel  
* atk-devel  
* openssl-devel  

### FreeBSD dependencies  
* openssl-devel
* gmake  
* gcc  

### Compling tested on  
- Ubuntu 17.04  
- Fedora 26  
- Arch Linux with i3  
- FreeBSD with Xfce  
- TrueOS  

### Dependency installation on Fedora  
`dnf install gtk3-devel cairo-devel libnotify-devel pango-devel gdk-pixbuf2-devel atk-devel openssl-devel`  

### Dependency installation on Ubuntu  
`apt install libgtk-3-dev libcairo2-dev libnotify-dev libpango1.0-dev libgdk-pixbuf2.0-dev libatk1.0-dev libssl1.0-dev libssl-dev`  

### Dependency installation on FreeBSD or TrueOS  
`pkg install openssl-devel gmake gcc`

### Compiling (via Github)  
1. `git clone https://github.com/thebitstick/ShareXin/`  
2. `cargo install`  
3. Login to Twitter and/or Mastodon using `t` and/or `toot`  
4. Explore `--help`  

### Compiling (via Crates.io)  
1. `cargo install sharexin`  
2. Login to Twitter and/or Mastodon using `t` and/or `toot`  
3. Explore `--help`  

## Changelog  
#### [0.5.1] - 2017-08-14  
- No more wildcards in getting modules  
- Separated `language.rs`  
- Now using scrot rather than maim  
- `.kill()` exists for `Command`, no longer using killall feh  
- Added Fatal error messages  

#### [0.5.0] - 2017-08-13  
- Rewrite of Language message system, now using templates, extremely easy to add more languages  
- Bug fixes  

#### [0.4.9] - 2017-08-12  
- Lots of optimizations  
- Actual bug fixes  
- Better error messages  
- `format!` is a thing  
- `gnome-screenshot` has native shadow effects for windows  
- Really dumb [seg](https://github.com/thebitstick/ShareXin/blob/db4b202d30eecb160b2e4db4fbd4f03f918ba4da/src/language.rs#L25) [fault](https://github.com/thebitstick/ShareXin/blob/db4b202d30eecb160b2e4db4fbd4f03f918ba4da/src/error.rs#L38)  
- Less code, actually  
- Less comments, cause "comments are bad"  

#### [0.4.8] - 2017-08-11  
- Stderr used for errors  
- Returns for String methods rather than variables  
- Completely remade the help function, universal syntax  
- Bug fixes  

#### [0.4.7] - 2017-08-09  
- Character count now shows when you've hit the limit and when you've passed
the limit  
- Rather than crashing, if notify-rust is unable to show you a notification,
it'll display an error message and exit nicely  
- Added French error messages  
- Bug fixes  

#### [0.4.6] - 2017-08-09  
- Error messages now translated  
- When uploading an image to Twitter, the character limit is reduced to 117, just like on ShareX  
- Less panicking  
- Less repetitiveness  
- t and toot now show notification if command fails to run (API troubles or no internet), rather than just showing you the "Sent" notification  
- Bug fixes  
- **Known issue**: In order to get t working correctly without a terminal, it must `killall vim`, so consider it a feature-bug  

#### [0.4.5] - 2017-08-07  
- BSD support, tested on FreeBSD with Xfce  
- Rather than panicking, ShareXin exits with an error message  
- Cleaner command line parsing  
- Error messages now multi-lingual, that is if the error doesn't include $LANG  
- Bug fixes  

#### [0.4.4] - 2017-08-04  
- Untested Mac notifications  
- Shadows only added to Window screenshots  
- Mac screenshot support coming soon  
- Bug fixes? Cleaner code? Maybe  

#### [0.4.3] - 2017-08-03  
- Bug fixes  
- Cleaner code  

#### [0.4.1] - 2017-08-02  
- Partial Wayland support for Gnome, Plasma, and Sway  
- Gnome-Screenshot used on Gnome  
- Spectacle used on Plasma/KDE  
- Swaygrab used on Sway (i3-clone)  

#### [0.4.0] - 2017-08-01  
- Cursor hidden in all screenshots  
- Removed double shadow  
- Bug fixes  

#### [0.3.9] - 2017-07-31  
- Bug fixes  

#### [0.3.8] - 2017-07-30  
- Better struct management  
- Imgur support! Opens image in browser  
- Changed date variable (had to change it manually everytime  

#### [0.3.7] - 2017-07-29  
- Custom error handles  
- Added experimental `open` Image Option, lets you select an image (animated or not) or possibly a video and send to a destination  
- Update checker implemented properly  
- Fixed service variables, now using structs  
- Readying code for more destinations  
- `auth` option not working at this current moment in time due to immature twitter apis available for Rust  
- `imgur` option hints at what will be the next Destination  

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
