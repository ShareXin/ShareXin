# ShareXin  

![](https://raw.githubusercontent.com/thebitstick/ShareXin/master/ui.png)

#### Requirements
* gnome-screenshot  
* gtk3  
* cairo  
* [t](https://github.com/sferik/t)  
* [toot](https://github.com/ihabunek/toot)  

#### Features
* Uploads to Twitter and Mastodon  
* Allows taking screenshots and saving them to files  
* Notifications  
* Works with Wayland (On Gnome only)  
* Works with X11 (On everything since the 90s)  
* Saves screenshots to folder in Pictures dir

#### Installation (via Github)
1. `git clone https://github.com/ShareXin/`
2. `cargo install`
3. Login to Twitter and/or Mastodon using t and/or toot
4. Explore `--help`

#### Installation (via Crates.io)
1. `cargo install sharexin`
2. Login to Twitter and/or Mastodon using t and/or toot
3. Explore `--help`

#### Troubleshooting
- If `t` won't send your tweet, check your $PATH. Your terminal may be able to launch it, but your WM/DE may not.
- If `sharexin` isn't found, add `$HOME/.cargo/bin` to your `$PATH`
- If `t` takes forever to send a tweet, remember that it's only a Ruby app...

#### Changelog
### [0.2.4] - 2017-07-21
- Added version info
- Made --help prettier

### [0.2.3] - 2017-07-21
- Send button now says Toot or Tweet depending on where you're going
- TextView no longer accepts tabs

### [0.2.2] - 2017-07-21
- TextView now word wraps
- Ability to simply just tweet without an image
- Mort

### [0.2.1] - 2017-07-20
- Centered window (why isn't .set_position() IN THE DOCS)

### [0.2.0] - 2017-07-20
- Uh, if you're haven problems with t not loadin', check your $PATH

### [0.1.0] - 2017-07-19
- Bug fixes and improvements

### [0.0.0] - 2017-07-19
### Added
- First commit
