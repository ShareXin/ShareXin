use std::env;
use VERSION;

pub fn help()
{
    let mut help_fr = String::from("\n");
    help_fr.push_str(VERSION);
    help_fr.push_str(" (2017 Juil 25)

Utilisation: sharexin <options>

Options:
  -h, --help\tAfficher le message d'aide et quitter
  -V, --version\tImprimer les informations de la version et quitter

Options d'image:
  -a\t\tCapturer une zone de l'écran plutôt que l'écran complet
  -w\t\tCapturer la fenêtre active plutôt que l'écran complet
  -n\t\tAucune image n'est sera prise, l'upload sera envoyé sans image

Options sociales:
  -m\t\tUpload vers Mastodon (en utilisant \"toot\") 
  -t\t\tUpload vers Twitter (en utilisant \"t\") 
  -f\t\tSauvegarder le fichier uniquement

Exemples:
  sharexin -at
  sharexin -wm
  sharexin -m
  sharexin -nt
    ");


    let mut help_eo = String::from("\n");
    help_eo.push_str(VERSION);
    help_eo.push_str(" (2017 Jul 25)

Uzo: sharexin <opcioj>

Opcioj:
  -h, --help\tMontru la helpo mesaĝon kaj eliro
  -V, --version\tPrintversio informoj kaj eliro

Opcioj de bildo:
  -a\t\tKapti areon de la ekrano anstataŭ ol la tuta ekrano
  -w\t\tKapti la aktivan fenestron anstataŭ ol la tuta ekrano
  -n\t\tNeniu bildoj estos prenita, la alŝuto sendiĝos sen bildo

Opcioj de sociaj:
  -m\t\tAlŝutu al Mastodon (uzante \"toot\")
  -t\t\tAlŝutu al Twitter (uzante \"t\")
  -f\t\tNur konservu la dosieron

Ekzemploj:
  sharexin -at
  sharexin -wm
  sharexin -m
  sharexin -nt
    ");


    let mut help_jp = String::from("\n");
    help_jp.push_str(VERSION);
    help_jp.push_str(" (平成29年7月25日)

使用方法: sharexin <オプション>

オプション:
  -h, --help\t標準出力に使用方法のメッセージを出力して正常終了する。
  -V, --version\t標準出力にバージョン情報を出力して正常終了する。

スクリーンショットのオプション:
  -a\t\t画面全体ではなく一部を取得する
  -w\t\t画面全体ではなくウィンドウ単体を取得する
  -n\t\tスクリーンショットは作成されず、送信されません

ソーシャルのオプション:
  -m\t\tマストドンにアップロード（使用して「ｔｏｏｔ」)
  -t\t\tツイッターにアップロード（使用して「ｔ」)
  -f\t\tファイルを保存のみ

例:
  sharexin -at
  sharexin -wm
  sharexin -m
  sharexin -nt
    ");


    let mut help_de = String::from("\n");
    help_de.push_str(VERSION);
    help_de.push_str(" (2017 Jul 25)

Anwendung: sharexin <optionen>

Optionen:
  -h, --help\tZeige diese Nachricht an und beende
  -V, --version\tGebe Version aus und beende

Bildoptionen:
  -a\t\tNur einen Bereich anstatt des gesamten Bildschirms aufnehmen
  -w\t\tNur ein Fenster anstatt des gesamten Bildschirms aufnehmen
  -n\t\tEs wird kein Bild aufgenommen, Tweet ohne Bild wird veröffentlicht

Optionen zu sozialen Netzwerken:
  -m\t\tAuf Mastodon veröffentlichen (benutzt \"toot\")
  -t\t\tAuf Twitter veröffentlichen (benutzt \"t\")
  -f\t\tDatei nur speichern

Beispiele:
  sharexin -at
  sharexin -wm
  sharexin -m
  sharexin -nt
    ");


    let mut help_es = String::from("\n");
    help_es.push_str(VERSION);
    help_es.push_str(" (2017 Jul 25)

Utilización: sharexin <opciones>

Opciones:
  -h, --help\tMostrar el mensaje de ayuda y sale
  -V, --version\tImprimir información de la versión y sale

Opciones de imagen:
  -a\t\tCapturar un área de la pantalla en lugar de la pantalla entera
  -w\t\tCapturar una ventana en vez de la pantalla entera
  -n\t\tNo se tomarán imágenes, la carga será enviado sin imagen

Opciones de social:
  -m\t\tSube a Mastodon (usando \"toot\")
  -t\t\tSube a Twitter (usando \"toot\")
  -f\t\tGuarde el archivo sólo

Ejemplos:
  sharexin -at
  sharexin -wm
  sharexin -m
  sharexin -nt
    ");


    let mut help = String::from("\n");
    help.push_str(VERSION);
    help.push_str(" (2017 Jul 25)

Usage: sharexin <options>

Options:
  -h, --help\tDisplay this help message and exit
  -V, --version\tPrint version info and exit

Image Options:
  -a\t\tGrab an area of the screen instead of the entire screen
  -w\t\tGrab the current window instead of the entire screen
  -n\t\tNo Image will be taken, will tweet without an image

Social Options:
  -m\t\tUpload to Mastodon (uses \"toot\")
  -t\t\tUpload to Twitter (uses \"t\")
  -f\t\tOnly save file

Examples:
  sharexin -at
  sharexin -wm
  sharexin -m
  sharexin -nt
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
