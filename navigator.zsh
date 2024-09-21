export NAVIGATOR_CONFIG=example.json

navigator_widget() {
    local selected_command
    local output_file="/tmp/navigator_output.txt"
    zle -I
    /home/ignat/work/navigator/target/debug/navigator < /dev/tty
    if [[ -f "$output_file" ]]; then
        selected_command=$(<"$output_file")
        rm "$output_file"
        if [[ -n "$selected_command" ]]; then
            LBUFFER="$selected_command"
        fi
    fi
    zle reset-prompt
    zle -R
}

zle -N navigator_widget
bindkey '^G' navigator_widget
