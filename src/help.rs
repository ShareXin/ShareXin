use std::env;
use open;
use VERSION;
use SHAREXIN;

pub fn upgrade()
{
    let _ = open::that(SHAREXIN);
    let _lang = env::var("LANG").unwrap();
    let lang = &_lang.to_lowercase();
    let mut version_fr = String::from("
Vérifiez les nouvelles mises à jour à l'adresse suivante: ");
    let mut version_es = String::from("
Busque nuevas actualizaciones en: ");
    let mut version_eo = String::from("
Kontrolu por novaj ĝisdatigoj ĉe: ");
    let mut version_ja = String::from("
新しいアップデートを確認する：\n");
    let mut version_de = String::from("
Überprüfen Sie nach neuen Updates unter: ");
    let mut version = String::from("
Check for new updates at: ");
    version_fr.push_str(SHAREXIN);
    version_es.push_str(SHAREXIN);
    version_eo.push_str(SHAREXIN);
    version_ja.push_str(SHAREXIN);
    version_de.push_str(SHAREXIN);
    version.push_str(SHAREXIN);
    if lang.contains("fr") {
        println!("{}", version_fr);
    }
    else if lang.contains("es") {
        println!("{}", version_es);
    }
    else if lang.contains("eo") {
        println!("{}", version_eo);
    }
    else if lang.contains("ja") {
        println!("{}", version_ja);
    }
    else if lang.contains("de") {
        println!("{}", version_de);
    }
    else { println!("{}", version);}
}

pub fn help()
{
    let mut help_fr = String::from("\nsharexin ");
    help_fr.push_str(VERSION);
    help_fr.push_str(" (2017 Juil 27)

Utilisation: sharexin <option> [destination] <option d'image>

Options:
  -h, --help\tAfficher le message d'aide et quitter
  -V, --version\tImprimer les informations de la version et quitter
  -U, --upgrade\tVérifiez les nouvelles mises à jour

Options d'image:
  area\t\tCapturer une zone de l'écran plutôt que l'écran complet
  window\tCapturer la fenêtre active plutôt que l'écran complet
  full\t\tCapturer l'écran complet

Destinations:
  toot\t\tUpload vers Mastodon (en utilisant \"toot\") 
  tweet\t\tUpload vers Twitter (en utilisant \"t\") 
  file\t\tSauvegarder le fichier uniquement

Exemples:
  sharexin toot
  sharexin tweet full
  sharexin file window
  sharexin toot area
    ");


    let mut help_es = String::from("\nsharexin ");
    help_es.push_str(VERSION);
    help_es.push_str(" (2017 Jul 27)

Utilización: sharexin <opciones> [destino] <opcion de imagen>

Opciones:
  -h, --help\tMostrar el mensaje de ayuda y sale
  -V, --version\tImprimir información de la versión y sale
  -U, --upgrade\tBusque nuevas actualizaciones

Opciones de imagen:
  area\t\tCapturar un área de la pantalla en lugar de la pantalla entera
  window\tCapturar una ventana en vez de la pantalla entera
  full\t\tCapturar la pantalla completa

Destinos:
  toot\t\tSube a Mastodon (usando \"toot\") 
  tweet\t\tSube a Twitter (usando \"t\") 
  file\t\tGuarde el archivo sólo

Ejemplos:
  sharexin toot
  sharexin tweet full
  sharexin file window
  sharexin toot area
    ");


    let mut help_eo = String::from("\nsharexin ");
    help_eo.push_str(VERSION);
    help_eo.push_str(" (2017 Jul 27)

Uzo: sharexin <opcioj> [celon] <opcio de bildo>

Opcioj:
  -h, --help\tMontru la helpo mesaĝon kaj eliro
  -V, --version\tPrintversio informoj kaj eliro
  -U, --upgrade\tKontrolu por novaj ĝisdatigoj

Opcioj de bildo:
  area\t\tKapti areon de la ekrano anstataŭ ol la tuta ekrano
  window\tKapti la aktivan fenestron anstataŭ ol la tuta ekrano
  full\t\tKapti la plena ekrano

Celoj:
  toot\t\tAlŝutu al Mastodon (uzante \"toot\")
  tweet\t\tAlŝutu al Twitter (uzante \"t\")
  file\t\tNur konservu la dosieron

Ekzemploj:
  sharexin toot
  sharexin tweet full
  sharexin file window
  sharexin toot area
    ");


    let mut help_ja = String::from("\nsharexin ");
    help_ja.push_str(VERSION);
    help_ja.push_str(" (平成29年7月27日)

使用方法: sharexin <オプション> [先] <スクリーンショットのオプション>

オプション:
  -h, --help\t標準出力に使用方法のメッセージを出力して正常終了する。
  -V, --version\t標準出力にバージョン情報を出力して正常終了する。
  -U, --upgrade\t新しい更新を確認する。

スクリーンショットのオプション:
  area\t\t画面全体ではなく一部を取得する
  window\t画面全体ではなくウィンドウ単体を取得する
  full\t\t画面全体を取得する

先:
  toot\t\tマストドンにアップロード（使用して「ｔｏｏｔ」)
  tweet\t\tツイッターにアップロード（使用して「ｔ」)
  file\t\tファイルを保存のみ

例:
  sharexin toot
  sharexin tweet full
  sharexin file window
  sharexin toot area
    ");


    let mut help_de = String::from("\nsharexin ");
    help_de.push_str(VERSION);
    help_de.push_str(" (2017 Jul 27)

Anwendung: sharexin <optionen> [reiseziel] <bildoptionen>

Optionen:
  -h, --help\tZeige diese Nachricht an und beende
  -V, --version\tGebe Version aus und beende
  -U, --upgrade\tAuf neue Updates prüfen

Bildoptionen:
  area\t\tNur einen Bereich anstatt des gesamten Bildschirms aufnehmen
  window\tNur ein Fenster anstatt des gesamten Bildschirms aufnehmen
  full\t\tGesamten Bildschirms aufnehmen

Reiseziele:
  toot\t\tAuf Mastodon veröffentlichen (benutzt \"toot\")
  tweet\t\tAuf Twitter veröffentlichen (benutzt \"t\")
  file\t\tDatei nur speichern

Beispiele:
  sharexin toot
  sharexin tweet full
  sharexin file window
  sharexin toot area
    ");


    let mut help = String::from("\nsharexin ");
    help.push_str(VERSION);
    help.push_str(" (2017 Jul 27)

Usage: sharexin <options> [destination] <image options>

Options:
  -h, --help\tDisplay this help message and exit
  -V, --version\tPrint version info and exit
  -U, --upgrade\tCheck for new updates

Image Options:
  area\t\tGrab an area of the screen instead of the entire screen
  window\tGrab the current window instead of the entire screen
  full\t\tGran the entire screen

Destinations:
  toot\t\tUpload to Mastodon (uses \"toot\")
  tweet\t\tUpload to Twitter (uses \"t\")
  file\t\tOnly save file

Examples:
  sharexin toot
  sharexin tweet full
  sharexin file window
  sharexin toot area
    ");

    let _lang = env::var("LANG").unwrap();
    let lang = &_lang.to_lowercase();
    if lang.contains("fr") {
        println!("{}", help_fr);
    }
    else if lang.contains("es") {
        println!("{}", help_es);
    }
    else if lang.contains("eo") {
        println!("{}", help_eo);
    }
    else if lang.contains("ja") {
        println!("{}", help_ja);
    }
    else if lang.contains("de") {
        println!("{}", help_de);
    }
    else { println!("{}", help);}
}
