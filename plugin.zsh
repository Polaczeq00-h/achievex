# arch-achievements.plugin.zsh

_arch_achievements_hook() {
    [[ -z "$1" ]] && return

    arch-achieve "trigger" "${1%% *}" "$1" &!
}

autoload -Uz add-zsh-hook
add-zsh-hook preexec _arch_achievements_hook

achievements() {
    local dir="$HOME/.arch_achievements"

    if [[ "$1" == "-reset" ]]; then
        arch-achieve -reset
        return
    fi

    local total_xp=$(cat "$dir/total_xp" 2>/dev/null || echo 0)

    echo -e "\e[1;36m"
        echo "  █████╗ ██████╗  ██████╗██╗  ██╗ "
        echo " ██╔══██╗██╔══██╗██╔════╝██║  ██║"
        echo " ███████║██████╔╝██║     ███████║"
        echo " ██╔══██║██╔══██╗██║     ██╔══██║"
        echo " ██║  ██║██║  ██║╚██████╗██║  ██║"
        echo " ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝"
        echo -e "\e[0m"

    echo -e "\e[1mSuccès débloqués :\e[0m"

    # On vérifie s'il y a au moins un fichier .lock
    if ls "$dir"/*.lock &>/dev/null; then
        for file in "$dir"/*.lock; do
            name=$(basename "$file" .lock | tr '_' ' ')
            echo -e "  \e[1;32m[✓]\e[0m $name"
        done
    else
        echo -e "  \e[1;30m(Aucun succès débloqué pour le moment)\e[0m"
    fi
    echo ""
}
