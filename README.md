# ShareXin  

[![Crates.io](https://img.shields.io/crates/v/sharexin.svg?)](https://crates.io/crates/sharexin)  
[![Github Stars](https://img.shields.io/github/stars/thebitstick/ShareXin.svg?)](https://github.com/thebitstick/ShareXin)  
[![Crates.io Downloads](https://img.shields.io/crates/d/sharexin.svg?)](https://crates.io/crates/sharexin)  

![Arc-Dark on i3](https://raw.githubusercontent.com/thebitstick/ShareXin/master/ui.png)  

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
#### English  
```
sharexin 0.3.1 (2017 Jul 25)

Usage:
    sharexin [options]
    sharexin -at
    sharexin --help
    sharexin -wm
    sharexin -m

Options:
    -h, --help		Display this help message
    -V, --version	Print version info and exit

Image Options:
    -a			Capture an area (default is Fullscreen)
    -w			Capture the current window (default is Fullscreen)
    -n			No Image will be taken, will tweet without an image

Social Options:
    -m		Upload to Mastodon (uses "toot")
    -t		Upload to Twitter (uses "t")
    -f		Only save file

```  

#### Esperanta  
```
sharexin 0.3.1 (2017 Jul 25)

Uzo:
    sharexin [opcioj]
    sharexin -at
    sharexin --help
    sharexin -wm
    sharexin -m

Opcioj:
    -h, --help		Montru la helpo mesaĝon
    -V, --version	Printversio informoj kaj eliro

Opcioj de bildo:
    -a			Kapti regionon / zono (plena ekrano defaŭlte)
    -w			Kapti la aktiva fenestro (Plena defaŭlta ekrano)
    -n			Neniu bildoj estos prenita, la alŝuto sendiĝos sen bildo

Opcioj de sociaj:
    -m		Alŝutu al Mastodon (uzante "toot")
    -t		Alŝutu al Twitter (uzante "t")
    -f		Nur konservu la dosieron
    
```  

#### Deutsche
```
sharexin 0.3.1 (2017 Jul 25)

Anwendung:
    sharexin [optionen]
    sharexin -at
    sharexin --help
    sharexin -wm
    sharexin -m
Optionen:
    -h, --help		Zeige diese Nachricht an
    -V, --version	Gebe Version aus und beende
Bildoptionen:
    -a			Nehme einen Bereich auf (standardmäßig Vollbild)
    -w			Nehme das aktuelle Fenster auf (standardmäßig Vollbild)
    -n			Es wird kein Bild aufgenommen, Tweet ohne Bild wird veröffentlicht
Optionen zu sozialen Netzwerken:
    -m		Auf Mastodon veröffentlichen (benutzt "toot")
    -t		Auf Twitter veröffentlichen (benutzt "t")
    -f		Datei nur speichern
    
```  

#### Français
```
sharexin 0.3.1 (2017 Juil 25)

Utilisation:
    sharexin [options]
    sharexin -at
    sharexin --help
    sharexin -wm
    sharexin -m

Options:
    -h, --help		Afficher le message d'aide
    -V, --version	Imprimer les informations de la version et quitter

Options d'image:
    -a			Capturer une région/zone (Plein écran par défaut)
    -w			Capturer la fenêtre active(Plein écran par défaut)
    -n			Aucune image n'est sera prise, l'upload sera envoyé sans image

Options sociales:
    -m		Upload vers Mastodon (en utilisant "toot") 
    -t		Upload vers Twitter (en utilisant "t") 
    -f		Sauvegarder le fichier uniquement
    
```

#### 日本語
```
sharexin 0.3.1 (平成29年7月25日)

使用:
    sharexin [オプション]
    sharexin -at
    sharexin --help
    sharexin -wm
    sharexin -m

オプション:
    -h, --help		標準出力に使用方法のメッセージを出力して正常終了する。
    -V, --version	標準出力にバージョン情報を出力して正常終了する。

スクリーンショットのオプション:
    -a			地域スクリーンショットのキャプチャ（全デフォルト画面）
    -w			アクティブなウィンドウをキャプチャ（全デフォルト画面）
    -n			スクリーンショットは取得されず、送信されません

ソーシャルのオプション:
    -m		マストドンにアップロード（使用して「ｔｏｏｔ」)
    -t		ツイッターにアップロード（使用して「ｔ」)
    -f		ファイルを保存のみ

```

#### Español
```    
sharexin 0.3.1 (2017 Jul 25)

Utilización:
    sharexin [opciones]
    sharexin -at
    sharexin --help
    sharexin -wm
    sharexin -m

Opciones:
    -h, --help		Mostrar el mensaje de ayuda
    -V, --version	Imprimir información de la versión y sale

Opciones de imagen:
    -a			Capturar una región (pantalla completa por defecto)
    -w			Capturar la ventana activa (pantalla completa por defecto)
    -n			No se tomarán imágenes, la carga será enviado sin imagen

Opciones de social:
    -m		Sube a Mastodon (usando "toot")
    -t		Sube a Twitter (usando "toot")
    -f		Guarde el archivo sólo

```

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

## Translations
- French by [@Eleoryth](https://twitter.com/Eleoryth)  
- German by [@qwertxzy](https://twitter.com/qwertxzy)  

## Changelog  
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