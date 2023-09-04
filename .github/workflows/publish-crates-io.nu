# Inspired from nushell deployment https://github.com/nushell/nu_scripts/blob/main/make_release/nu_release.nu

subcrates = [
    "cargo_info"
]

for subcrate in subcrates {
    cargo publish ("crates" | path join $subcrate)
}

cargo publish