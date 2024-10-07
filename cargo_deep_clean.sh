#!/bin/bash

clean_dir() {
    local dir="$1"

    echo -n "${dir%*/}: "
    # '%*/': skip trailing '/'

    cd "${dir}" || return

    cargo clean
    rmdir target*
    cargo fmt

    cd ..
    echo
}

except="hello_world/"

# Store All Directories
# from List Command Output
all_dir=($(ls -d */))

for dir in "${all_dir[@]}"; do
    if [ "${dir}" != "$except" ]; then
        clean_dir "${dir}"
    fi
done
