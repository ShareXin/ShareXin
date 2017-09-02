_sharexin()
{
    cur=${COMP_WORDS[COMP_CWORD]}
    prev=${COMP_WORDS[COMP_CWORD-1]}

    case "$prev" in
        -'?'|--help|-v|--version|-h|-U|--upgrade|-t|--tray)
            return
            ;;

        toot)
            COMPREPLY=( $( compgen -W 'area window full file auth --' -- $cur ) )
            return
            ;;

	tweet)
	    COMPREPLY=( $ compgen -W 'area window full file auth --' -- $cur ) )
	    return
	    ;;

    imgur)
	    COMPREPLY=( $ compgen -W 'area window full file' -- $cur ) )
	    return
	    ;;

    esac

    case "$cur" in
        -*)
            COMPREPLY=( $( compgen -W '$( _parse_help "$1" )' -- "$cur" ) )
            ;;

        *)
            _filedir
            ;;
    esac
} &&
complete -o filenames -o bashdefault -F _sharexin sharexin
