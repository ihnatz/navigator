export NAVIGATOR_CONFIG=example.json

fzf_navigator() {
    local output_file="/tmp/navigator_output.txt"

    zle -I

    navigator > /dev/tty 2>&1
    local selected_command
    if [[ -f "$output_file" ]]; then
        selected_command=$(<"$output_file")
        rm "$output_file"
        if [[ -n "$selected_command" ]]; then
            LBUFFER="$selected_command"
            zle reset-prompt
        fi
    fi

    zle -R
}

zle -N fzf_navigator
bindkey '^G' fzf_navigator
