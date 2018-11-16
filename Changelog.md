#### [0.6.4] - 2018-03-30
- Wow this project keeps going
- Added Portuguese translation

#### [0.6.3] - 2017-10-19
- Added support for Ubuntu 17.10 Gnome Desktop (congrats on upgrading to the future)

#### [0.6.2] - 2017-09-01
- System tray support (not for Mac)
- New slogan, guaranteed to work on Linux, FreeBSD, and macOS only
- Makefile and PKGBUILD
- Better command line parsing with clap-rs
- ShareXin library? I guess?

#### [0.6.1] - 2017-08-26
- XDG Directory for Pictures folder (switched my system to French, directories changed)

#### [0.6.0] - 2017-08-25
- macOS tested, Imgur works, `t` and `toot` should work if you have them
- Updated "upgrade" and error messages
- Figured out libcurl issue on some older systems
- Bug fixes

#### [0.5.9] - 2017-08-20
- ImageMagick integrated
- `killall vim` finally removed (kinda)!

#### [0.5.8] - 2017-08-20
- Imgur link now copied to clipboard _(should work on Wayland?)_
- Better desktop recognition
- Bug fixes

#### [0.5.7] - 2017-08-19
- Less repetitive code
- Area screenshots (except for KDE and Mac) use two separate files,
the frozen fullscreen and the part of it you select
- Fixed bug where Error 29 would always print (kind of a bug)

#### [0.5.6] - 2017-08-19
- Removes temporary file after sending it
- Budgie Desktop support
- Ubuntu Unity Desktop support
- Theoretical Mac support

#### [0.5.5] - 2017-08-18
- Added Polish translation
- Character count acts like ShareX, counts down from 140 or 500 (depending on destination)
- "File Saved" notification now includes image saved

#### [0.5.4] - 2017-08-17
- Just a Cargo fix, ignore

#### [0.5.3] - 2017-08-15
- Now packaging `t` (should work)
- Added `auth` option for Twitter and Mastodon

#### [0.5.2] - 2017-08-15
- GTk fixes
- Notification for error messages
- Bug fixes
- Now using AppImage for releases

#### [0.5.1] - 2017-08-14
- No more wildcards in getting modules
- Separated `language.rs`
- Now using scrot rather than maim
- `.kill()` exists for `Command`, no longer using killall feh
- Added Fatal error messages
- Cinnamon desktop support

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
