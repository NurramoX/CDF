cdf() {
    if [ "$1" = "register" ]; then
        if [ "$#" -lt 2 ]; then
            echo "No keey was provided"
            return
        fi
        if [ "$#" -eq 2 ]; then
	    cdf_backend register $2 $(pwd)
        else
            cdf_backend register $2 $3
        fi
        echo "Locked and loaded. Path registered for key: $2"
        return
    else
        local target_path=$(cdf_backend $1)
        if [ -n "$target_path" ]; then
            cd $target_path
        else
            echo "Bloody hell! No path registered for key: $1"
        fi
    fi

}


_cdf() {
    local state
    local -a keys

    _arguments -C \
        '1: :->cmds' \
        '2:keys: '

    case $state in
        cmds)
            keys=($(cdf_backend list))  # Using the new command to fetch keys
            _describe 'command' keys
            ;;
    esac
}

compdef _cdf cdf
