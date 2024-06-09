export def get_flags [] {
    let flags = do { "" | `c++` -E -Wp,-v - } 
        | complete 
        | get stderr 
        | parse -r ' (/.*)'
        | get capture0 
        | each {|it| $"-isystem($it)"} 
    {
        flags: $flags
        clang_args: $"--sysroot=/usr/local/llvm ($flags | str join ' ')" 
    }
}

export def execute [call: closure] {
    with-env {BINDGEN_EXTRA_CLANG_ARGS: (get_flags | get clang_args)} $call
}