/*
    Error Codes:
        1 - Error getting $LANG
        2 - Github unreachable
        3 - Imgur unreachable
        4 - t command unavailable
        5 - toot command unavailable
        6 - Error getting $XDG_SESSION_TYPE
        7 - Error getting $DESKTOP_SESSION
        8 - Version number download error on parsing
        9 - Unable to open file or webpage
        10 - Error uploading to Imgur
        11 - GTK init error
        12 - Unable to get time
        13 - File not saved
*/

pub fn message(code: usize) -> String {
    let mut _return = String::new();

    if code == 1 {
        _return.push_str("Error getting $LANG");
    }

    _return
}
