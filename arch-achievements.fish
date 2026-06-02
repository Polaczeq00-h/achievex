# arch-achievements.fish

function _arch_achievements_preexec --on-event fish_preexec
    test -z "$argv[1]"
    and return

    set -l cmd_full "$argv[1]"
    set -l cmd_name (string split --max 1 " " -- "$cmd_full")[1]

    command arch-achieve trigger "$cmd_name" "$cmd_full" >/dev/null 2>&1 &
end

function achievements
    command arch-achieve $argv
end
