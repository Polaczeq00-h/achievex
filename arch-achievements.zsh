# arch-achievements.zsh
_arch_achievements_preexec() {
    local cmd_full="$1"
    local cmd_name="${1%% *}"

    arch-achieve "trigger" "$cmd_name" "$cmd_full" &!
}

autoload -Uz add-zsh-hook
add-zsh-hook preexec _arch_achievements_preexec
