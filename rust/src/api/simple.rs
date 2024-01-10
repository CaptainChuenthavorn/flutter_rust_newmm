#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    // format!("Hello, {name}!")
    let my_str = include_str!("kk_th.txt");
    // assert_eq!(my_str, "adiós\n");
    format!("{my_str}")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn gettoken(name: String) -> String {
    let my_str = include_str!("kk_th.txt");
    // assert_eq!(my_str, "adiós\n");
    format!("{my_str}")
}
