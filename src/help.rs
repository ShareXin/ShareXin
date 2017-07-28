use std::env;
use open;
use VERSION;
use SHAREXIN;

pub fn upgrade()
{
    let _ = open::that(SHAREXIN);
    let _lang = env::var("LANG").unwrap();
    let lang = &_lang.to_lowercase();
    let mut upgrade_fr = String::from("
Vérifiez les nouvelles mises à jour à l'adresse suivante: ");
    let mut upgrade_es = String::from("
Busque nuevas actualizaciones en: ");
    let mut upgrade_eo = String::from("
Kontrolu por novaj ĝisdatigoj ĉe: ");
    let mut upgrade_cn = String::from("
要检查是否有新的更新：\n");
    let mut upgrade_tw = String::from("
要檢查是否有新的更新：\n");
    let mut upgrade_ja = String::from("
新しいアップデートを確認する：\n");
    let mut upgrade_ko = String::from("
새로운 업데이트 확인 :\n");
    let mut upgrade_de = String::from("
Überprüfen Sie nach neuen Updates unter: ");
    let mut upgrade = String::from("
Check for new updates at: ");
    upgrade_fr.push_str(SHAREXIN);
    upgrade_es.push_str(SHAREXIN);
    upgrade_eo.push_str(SHAREXIN);
    upgrade_cn.push_str(SHAREXIN);
    upgrade_tw.push_str(SHAREXIN);
    upgrade_ja.push_str(SHAREXIN);
    upgrade_ko.push_str(SHAREXIN);
    upgrade_de.push_str(SHAREXIN);
    upgrade.push_str(SHAREXIN);
    if lang.contains("fr") {
        println!("{}", upgrade_fr);
    }
    else if lang.contains("es") {
        println!("{}", upgrade_es);
    }
    else if lang.contains("eo") {
        println!("{}", upgrade_eo);
    }
    else if lang.contains("cn") {
        println!("{}", upgrade_cn);
    }
    else if lang.contains("tw") {
        println!("{}", upgrade_tw);
    }
    else if lang.contains("ja") {
        println!("{}", upgrade_ja);
    }
    else if lang.contains("ko") {
        println!("{}", upgrade_ko);
    }
    else if lang.contains("de") {
        println!("{}", upgrade_de);
    }
    else { println!("{}", upgrade);}
}

pub fn help()
{
    let mut help_fr = String::from("\nsharexin ");
    help_fr.push_str(VERSION);
    help_fr.push_str(" (2017 Juil 28)

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
    help_es.push_str(" (2017 Jul 28)

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
    help_eo.push_str(" (2017 Jul 28)

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


    let mut help_cn = String::from("\nsharexin ");
    help_cn.push_str(VERSION);
    help_cn.push_str(" (2017年7月28日)

使用方法： sharexin <选项> [目的地] <截图选项>

选项:
  -h, --help\t退出标准成功输出输出用法消息。
  -V, --version\t在成功完成输出版本信息到标准输出。
  -U, --upgrade\t要检查新的更新。

选项截图:
  area\t\t截取屏幕的一个区域，而不是整个屏幕
  window\t截取窗口，而不是整个屏幕
  full\t\t为了让整个屏幕

目的地:
  toot\t\t上传到Mastodon（使用 “toot”）
  tweet\t\t它上传到Twitter（使用 “t”）
  file\t\t保存文件只

案件:
  sharexin toot
  sharexin tweet full
  sharexin file window
  sharexin toot area
    ");


    let mut help_tw = String::from("\nsharexin ");
    help_tw.push_str(VERSION);
    help_tw.push_str(" (2017年7月28日)

使用方法：sharexin <選項> [目的地] <截圖選項>

選項:
  -h, --help\t退出標準成功輸出輸出用法消息。
  -V, --version\t在成功完成輸出版本信息到標準輸出。
  -U, --upgrade\t要檢查新的更新。

選項截圖:
  area\t\t擷取畫面的一個區域而不是整個畫面
  window\t擷取單一視窗而不是整個畫面
  full\t\t為了讓整個屏幕

目的地:
  toot\t\t上傳到Mastodon（使用 “toot”）
  tweet\t\t它上傳到Twitter（使用 “t”）
  file\t\t保存文件只

案件:
  sharexin toot
  sharexin tweet full
  sharexin file window
  sharexin toot area
    ");


    let mut help_ja = String::from("\nsharexin ");
    help_ja.push_str(VERSION);
    help_ja.push_str(" (平成29年7月28日)

使用方法: sharexin <オプション> [行き先] <スクリーンショットのオプション>

オプション:
  -h, --help\t標準出力に使用方法のメッセージを出力して正常終了する。
  -V, --version\t標準出力にバージョン情報を出力して正常終了する。
  -U, --upgrade\t新しい更新を確認する。

スクリーンショットのオプション:
  area\t\t画面全体ではなく一部を取得する
  window\t画面全体ではなくウィンドウ単体を取得する
  full\t\t画面全体を取得する

行き先:
  toot\t\tマストドンにアップロード（使用して「ｔｏｏｔ」)
  tweet\t\tツイッターにアップロード（使用して「ｔ」)
  file\t\tファイルを保存のみ

例:
  sharexin toot
  sharexin tweet full
  sharexin file window
  sharexin toot area
    ");


    let mut help_ko = String::from("\nsharexin ");
    help_ko.push_str(VERSION);
    help_ko.push_str(" (2017년7월28일)

사용 방법: sharexin <옵션> [목적지] <스크린 샷 옵션>

옵션:
  -h, --help\t표준 출력으로 사용법을 출력하고 정상적으로 종료한다.
  -V, --version\t표준 출력으로 버전 정보를 출력하고 정상적으로 종료한다.
  -U, --upgrade\t새로운 업데이트를 확인한다.

스크린 샷 옵션:
  area\t\t전체 화면 대신에 화면의 일정 영역을 찍습니다
  window\t전체 화면 대신에 창을 찍습니다
  full\t\t전체 화면을 얻을

목적지:
  toot\t\tMastodon에 업로드 (사용 \"toot\")
  tweet\t\tTwitter에 업로드 (사용 \"t\")
  file\t\t파일을 저장 만

예:
  sharexin toot
  sharexin tweet full
  sharexin file window
  sharexin toot area
    ");


    let mut help_de = String::from("\nsharexin ");
    help_de.push_str(VERSION);
    help_de.push_str(" (2017 Jul 28)

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
    help.push_str(" (2017 Jul 28)

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
    else if lang.contains("cn") {
        println!("{}", help_cn);
    }
    else if lang.contains("tw") {
        println!("{}", help_tw);
    }
    else if lang.contains("ja") {
        println!("{}", help_ja);
    }
    else if lang.contains("ko") {
        println!("{}", help_ko);
    }
    else if lang.contains("de") {
        println!("{}", help_de);
    }
    else { println!("{}", help);}
}
