// src-tauri/src/commands.rs   (recomendado)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppCommand {
    Greet,
    RustToReact,
    // Login,
    // Logout,
    // GetGameState,
    // StartMatch,
    // etc.
}

impl AppCommand {
    /// Devuelve el string exacto que se usa en el invoke_handler y en el frontend
    pub fn as_str(&self) -> &'static str {
        match self {
            AppCommand::Greet => "greet",
            AppCommand::RustToReact => "rust_to_react",
            // AppCommand::Login => "login",
            // AppCommand::Logout => "logout",
        }
    }
}