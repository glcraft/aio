use quote::quote;
use proc_macro::TokenStream;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CargoPackage {
    name: String,
    version: String,
}
#[derive(Debug, Deserialize)]
struct CargoInfo {
    package: CargoPackage,
}

fn load_cargo_file() -> Option<CargoInfo> {
    let dir_path = std::env::var("CARGO_MANIFEST_DIR").ok()?;
    let file_path = std::path::Path::new(&dir_path).join("Cargo.toml");
    let contents = std::fs::read_to_string(&file_path).ok()?;
    toml::from_str(&contents).ok()
}

macro_rules! cargo_info {
    ($($fields:ident).*) => {
        {
            let cargo_info = load_cargo_file();
            let Some(cargo_info) = cargo_info else { return quote! {""}.into() };
            cargo_info$(.$fields)*
        }
    };
}

#[proc_macro]
pub fn name(_: TokenStream) -> TokenStream {
    let name = cargo_info!(package.name);
    quote! { #name }.into()
}
#[proc_macro]
pub fn version(_: TokenStream) -> TokenStream {
    let version = cargo_info!(package.version);
    quote! { #version }.into()
}
#[proc_macro]
pub fn user_agent(_: TokenStream) -> TokenStream {
    let package = cargo_info!(package);
    let user_agent = format!("{}/{}", package.name, package.version);
    quote! { #user_agent }.into()
}