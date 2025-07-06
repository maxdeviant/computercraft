set dotenv-load

# Syncs the scripts with the computer with the given ID.
sync computer_id:
    #!/usr/bin/env bash
    set -euo pipefail

    if [ -z "${COMPUTERCRAFT_DIR:-}" ]; then
        echo "Error: COMPUTERCRAFT_DIR environment variable is not set"
        echo "Please set it to the path of your ComputerCraft save directory"
        exit 1
    fi

    target_dir="${COMPUTERCRAFT_DIR}/computer/{{computer_id}}"

    mkdir -p "$target_dir"

    echo "Syncing scripts to computer {{computer_id}}..."

    cp std.lua "$target_dir/"

    # Copy main scripts
    cp programs/*.lua "$target_dir/"

    # Copy lib directory if it exists
    if [ -d "lib" ]; then
        mkdir -p "$target_dir/lib"
        cp lib/*.lua "$target_dir/lib/"
    fi

    echo "Successfully synced scripts to $target_dir"

# List all available computers in the save directory
list-computers:
    #!/usr/bin/env bash
    set -euo pipefail

    if [ -z "${COMPUTERCRAFT_DIR:-}" ]; then
        echo "Error: COMPUTERCRAFT_DIR environment variable is not set"
        exit 1
    fi

    computer_dir="${COMPUTERCRAFT_DIR}/computer"

    if [ ! -d "$computer_dir" ]; then
        echo "No computers found in $computer_dir"
        exit 1
    fi

    echo "Available computers:"
    ls -1 "$computer_dir" | sort -n
