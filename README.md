# ShareXin  

![](https://raw.githubusercontent.com/thebitstick/ShareXin/master/ui.png)

#### Requirements
* gnome-screenshot  
* gtk3  
* [t](https://github.com/sferik/t)  
* [toot](https://github.com/ihabunek/toot)  

#### Features
* Uploads to Twitter and Mastodon
* Notifications
* Works with Wayland (On Gnome only) and X11 (On everything since the 90s)

#### Installation
1. `git clone https://github.com/ShareXin/`
2. `cargo install`
3. Login to Twitter and/or Mastodon using t and/or toot
4. Explore `--help`

#### Troubleshooting
- If 't' won't send your tweet, check your $PATH. Your terminal may be able to launch it, but your WM/DE may not.

### Changelog
#### [0.2.1] - 2017-07-20
- Centered window (why isn't .set_position() IN THE DOCS)

#### [0.2.0] - 2017-07-20
- Uh, if you're haven problems with t not loadin', check your $PATH

#### [0.1.0] - 2017-07-19
- Bug fixes and improvements

#### [0.0.0] - 2017-07-19
#### Added
- First commit
