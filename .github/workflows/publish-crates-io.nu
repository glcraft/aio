#!/usr/bin/env nu
# Inspired from nushell deployment https://github.com/nushell/nu_scripts/blob/main/make_release/nu_release.nu

def publish [
    crate: path # the path to the crate to publish.
] {
    let prev = $env.PWD
    cd $crate

    do --ignore-program-errors { cargo publish --token $env.CARGO_TOKEN }
    cd $prev
}

let subcrates = [
    "aio-cargo-info"
]

for subcrate in $subcrates {
    publish ("crates" | path join $subcrate)
}

cargo publish --allow-dirty --token $env.CARGO_TOKEN