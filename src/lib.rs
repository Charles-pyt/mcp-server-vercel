#![allow(unused)]
use zed_extension_api::{self as zed, ContextServerId, Project, Result};

/// Notre extension : un wrapper du MCP officiel Vercel.
/// Le serveur tourne côté Vercel (https://mcp.vercel.com),
/// on indique juste à Zed la commande pour s'y connecter
/// via npx + mcp-remote (bridge stdio ↔ HTTP).
struct VercelExtension;

impl zed::Extension for VercelExtension {
    fn new() -> Self {
        Self
    }

    /// Retourne la commande que Zed doit lancer pour démarrer
    /// le serveur MCP Vercel dans l'Agent Panel.
    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<zed::Command> {
        // mcp-remote est un bridge officiel stdio ↔ HTTP MCP.
        // npx le télécharge automatiquement si absent.
        // L'authentification OAuth s'ouvre dans le navigateur
        // la première fois — Zed gère la session ensuite.
        Ok(zed::Command {
            command: "npx".to_string(),
            args: vec![
                "-y".to_string(),
                "mcp-remote@latest".to_string(),
                "https://mcp.vercel.com/sse".to_string(),
            ],
            // Pas de variables d'env nécessaires :
            // l'auth se fait via OAuth dans le browser.
            env: vec![],
        })
    }
}

// Enregistre l'extension auprès du runtime Zed (WASM)
zed::register_extension!(VercelExtension);
