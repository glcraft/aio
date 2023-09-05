# Inspired from nushell deployment https://github.com/nushell/nu_scripts/blob/main/make_release/nu_release.nu

let subcrates = [
    "aio-cargo-info"
]

for subcrate in subcrates {
    cargo publish --token $env.CARGO_TOKEN ("crates" | path join $subcrate)
}

cargo publish --token $env.CARGO_TOKEN