#!/usr/bin/env bash
set -euo pipefail

# Run on Linux with dx and util-linux's script command installed.
template_root=$(cd "$(dirname "$0")/.." && pwd)
work_dir=$(mktemp -d)
trap 'rm -rf "$work_dir"' EXIT
cd "$work_dir"

for template in Bare-Bones Jumpstart; do
    for tailwind in false true default; do
        project="${template,,}-$tailwind"
        args=(dx new --template "$template_root" --subtemplate "$template"
            --yes --option default_platform=web --vcs none)
        if [[ "$tailwind" != default ]]; then
            args+=(--option "is_tailwind=$tailwind")
        fi
        args+=("$project")
        printf -v command '%q ' "${args[@]}"

        # Progress is hidden without a terminal. Capture it, including in CI.
        if ! SHELL=/bin/bash TERM=xterm script -qefc "$command" "$project.log" > /dev/null; then
            cat "$project.log"
            exit 1
        fi
        if ! grep -Fq 'Done: src/main.rs' "$project.log"; then
            echo "$project: file progress was not captured" >&2
            exit 1
        fi

        for file in tailwind.css assets/tailwind.css; do
            if [[ "$tailwind" == true ]]; then
                test -f "$project/$file"
                grep -Fq "Done: $file" "$project.log"
            else
                test ! -e "$project/$file"
                if grep -Fq "Done: $file" "$project.log"; then
                    echo "$project: reported Done for disabled $file" >&2
                    exit 1
                fi
            fi
        done

        if [[ "$tailwind" == true ]]; then
            grep -Fq 'asset!("/assets/tailwind.css")' "$project/src/main.rs"
        elif grep -Fq 'asset!("/assets/tailwind.css")' "$project/src/main.rs"; then
            echo "$project: referenced disabled Tailwind stylesheet" >&2
            exit 1
        fi
        echo "PASS: $project"
    done
done
