const INK_VERSION: &'static str = "~3.3.0";

pub fn generate_cargo_toml() -> String {
    let mut out = String::new();

    out.push_str("[package]\n");
    out.push_str("name = \"sol_2_ink_generated\"\n");
    out.push_str("version = \"0.1.0\"\n");
    out.push_str("edition = \"2021\"\n");
    out.push_str("authors = [\"Sol2Ink\"]\n");
    out.push_str("\n");
    out.push_str("[dependencies]\n");
    out.push_str(generate_ink_dependency("ink_primitives", false, false).as_str());
    out.push_str(generate_ink_dependency("ink_metadata", true, true).as_str());
    out.push_str(generate_ink_dependency("ink_env", false, false).as_str());
    out.push_str(generate_ink_dependency("ink_storage", false, false).as_str());
    out.push_str(generate_ink_dependency("ink_lang", false, false).as_str());
    out.push_str(generate_ink_dependency("ink_prelude", false, false).as_str());
    out.push_str(generate_ink_dependency("ink_engine", false, true).as_str());
    out.push_str("scale = { package = \"parity-scale-codec\", version = \"3\", default-features = false, features = [\"derive\"] }\n");
    out.push_str("scale-info = { version = \"2\", default-features = false, features = [\"derive\"], optional = true }\n");
    out.push_str("openbrush = { version = \"2.2.0\", default-features = false }\n");
    out.push_str("\n");
    out.push_str("[lib]\n");
    out.push_str("name = \"sol_2_ink_generated\"\n");
    out.push_str("path = \"lib.rs\"\n");
    out.push_str("crate-type = [\"cdylib\"]\n");
    out.push_str("\n");
    out.push_str("[features]\n");
    out.push_str("default = [\"std\"]\n");
    out.push_str("std = [\n");
    out.push_str("\"ink_primitives/std\",\n");
    out.push_str("\"ink_metadata\",\n");
    out.push_str("\"ink_metadata/std\",\n");
    out.push_str("\"ink_env/std\",\n");
    out.push_str("\"ink_storage/std\",\n");
    out.push_str("\"ink_lang/std\",\n");
    out.push_str("\"scale/std\",\n");
    out.push_str("\"scale-info\",\n");
    out.push_str("\"scale-info/std\",\n");
    out.push_str("\"openbrush/std\",\n");
    out.push_str("]\n");
    out.push_str("\n");

    out
}

fn generate_ink_dependency(crate_name: &str, derive: bool, optional: bool) -> String {
    let mut out = String::new();

    out.push_str(crate_name);
    out.push_str(" = { version = \"");
    out.push_str(INK_VERSION);
    out.push_str("\", default-features = false");

    if derive {
        out.push_str(", features = [\"derive\"]");
    }
    if optional {
        out.push_str(", optional = true");
    }

    out.push_str(" }\n");

    out
}
