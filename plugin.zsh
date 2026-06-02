# arch-achievements.plugin.zsh

_arch_achievements_hook() {
    [[ -z "$1" ]] && return

    arch-achieve "trigger" "${1%% *}" "$1" &!
}

autoload -Uz add-zsh-hook
add-zsh-hook preexec _arch_achievements_hook

achievements() {
    arch-achieve "$@"
}
