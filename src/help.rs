use std::env;
use VERSION;

pub fn help()
{
    let mut help_fr = String::from("\n");
    help_fr.push_str(VERSION);
    help_fr.push_str(" (2017 Juil 25)

Utilisation:
    sharexin [options]
    sharexin -at
    sharexin --help
    sharexin -wm
    sharexin -m

Options:
    -h, --help\t\tAfficher le message d'aide
    -V, --version\tImprimer les informations de la version et quitter

Options d'image:
    -a\t\t\tCapturer une région/zone (Plein écran par défaut)
    -w\t\t\tCapturer la fenêtre active(Plein écran par défaut)
    -n\t\t\tAucune image n'est sera prise, l'upload sera envoyé sans image

Options sociales:
    -m\t\tUpload vers Mastodon (en utilisant \"toot\") 
    -t\t\tUpload vers Twitter (en utilisant \"t\") 
    -f\t\tSauvegarder le fichier uniquement
    ");


    let mut help_eo = String::from("\n");
    help_eo.push_str(VERSION);
    help_eo.push_str(" (2017 Jul 25)

Uzo:
    sharexin [opcioj]
    sharexin -at
    sharexin --help
    sharexin -wm
    sharexin -m

Opcioj:
    -h, --help\t\tMontru la helpo mesaĝon
    -V, --version\tPrintversio informoj kaj eliro

Opcioj de bildo:
    -a\t\t\tKapti regionon / zono (plena ekrano defaŭlte)
    -w\t\t\tKapti la aktiva fenestro (Plena defaŭlta ekrano)
    -n\t\t\tNeniu bildoj estos prenita, la alŝuto sendiĝos sen bildo

Opcioj de sociaj:
    -m\t\tAlŝutu al Mastodon (uzante \"toot\")
    -t\t\tAlŝutu al Twitter (uzante \"t\")
    -f\t\tNur konservu la dosieron
    ");


    let mut help_jp = String::from("\n");
    help_jp.push_str(VERSION);
    help_jp.push_str(" (平成29年7月25日)

使用:
    sharexin [オプション]
    sharexin -at
    sharexin --help
    sharexin -wm
    sharexin -m

オプション:
    -h, --help\t\t標準出力に使用方法のメッセージを出力して正常終了する。
    -V, --version\t標準出力にバージョン情報を出力して正常終了する。

スクリーンショットのオプション:
    -a\t\t\t地域スクリーンショットのキャプチャ（全デフォルト画面）
    -w\t\t\tアクティブなウィンドウをキャプチャ（全デフォルト画面）
    -n\t\t\tスクリーンショットは取得されず、送信されません

ソーシャルのオプション:
    -m\t\tマストドンにアップロード（使用して「ｔｏｏｔ」)
    -t\t\tツイッターにアップロード（使用して「ｔ」)
    -f\t\tファイルを保存のみ
    ");


    let mut help_de = String::from("\n");
    help_de.push_str(VERSION);
    help_de.push_str(" (2017 Jul 25)

Anwendung:
    sharexin [optionen]
    sharexin -at
    sharexin --help
    sharexin -wm
    sharexin -m
Optionen:
    -h, --help\t\tZeige diese Nachricht an
    -V, --version\tGebe Version aus und beende
Bildoptionen:
    -a\t\t\tNehme einen Bereich auf (standardmäßig Vollbild)
    -w\t\t\tNehme das aktuelle Fenster auf (standardmäßig Vollbild)
    -n\t\t\tEs wird kein Bild aufgenommen, Tweet ohne Bild wird veröffentlicht
Optionen zu sozialen Netzwerken:
    -m\t\tAuf Mastodon veröffentlichen (benutzt \"toot\")
    -t\t\tAuf Twitter veröffentlichen (benutzt \"t\")
    -f\t\tDatei nur speichern
    ");


    let mut help_es = String::from("\n");
    help_es.push_str(VERSION);
    help_es.push_str(" (2017 Jul 25)

Utilización:
    sharexin [opciones]
    sharexin -at
    sharexin --help
    sharexin -wm
    sharexin -m

Opciones:
    -h, --help\t\tMostrar el mensaje de ayuda
    -V, --version\tImprimir información de la versión y sale

Opciones de imagen:
    -a\t\t\tCapturar una región (pantalla completa por defecto)
    -w\t\t\tCapturar la ventana activa (pantalla completa por defecto)
    -n\t\t\tNo se tomarán imágenes, la carga será enviado sin imagen

Opciones de social:
    -m\t\tSube a Mastodon (usando \"toot\")
    -t\t\tSube a Twitter (usando \"toot\")
    -f\t\tGuarde el archivo sólo
    ");


    let mut help = String::from("\n");
    help.push_str(VERSION);
    help.push_str(" (2017 Jul 25)

Usage:
    sharexin [options]
    sharexin -at
    sharexin --help
    sharexin -wm
    sharexin -m

Options:
    -h, --help\t\tDisplay this help message
    -V, --version\tPrint version info and exit

Image Options:
    -a\t\t\tCapture an area (default is Fullscreen)
    -w\t\t\tCapture the current window (default is Fullscreen)
    -n\t\t\tNo Image will be taken, will tweet without an image

Social Options:
    -m\t\tUpload to Mastodon (uses \"toot\")
    -t\t\tUpload to Twitter (uses \"t\")
    -f\t\tOnly save file
    ");


    let mut lang = env::var("LANG").unwrap();
    lang = lang.to_lowercase();
    if lang.contains("fr") {
        println!("{}", help_fr);
    }
    else if lang.contains("es") {
        println!("{}", help_es);
    }
    else if lang.contains("eo") {
        println!("{}", help_eo);
    }
    else if lang.contains("jp") {
        println!("{}", help_jp);
    }
    else if lang.contains("de") {
        println!("{}", help_de);
    }
    else { println!("{}", help);}
}
