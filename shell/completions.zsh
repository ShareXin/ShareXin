#compdef sharexin

__sharexin() {
    _arguments \
        "(- 1 *)"{-v,--version}"[Show version of sharexin]" \
        "(- 1 *)"{-h,--help}"[Show list of command-line options]" \
        "(- 1 *)"{-U,--upgrade}"[Check for upgrades]" \
        "(- 1 *)"{-t,--tray}"[Use system tray icon]" \
        {tweet}"[Post to Twitter]" \
        {toot}"[Post to Mastodon]" \
        {imgur}"[Post to Imgur]" \
        '*:filename:_files'
}

__sharexin
