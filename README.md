# Vercel MCP Server — Zed Extension

Connect the official Vercel MCP server to Zed's Agent Panel.
Manage your deployments, projects, domains and environment
variables directly from the AI chat.

## Tools available

Once connected, the agent can:

- List and inspect **deployments** (status, logs, build output)
- Manage **projects** (create, pause, delete, transfer)
- Handle **environment variables** (read, create, update)
- Control **domains** and DNS configuration
- Browse **Vercel documentation** (no auth required)

## Requirements

- Node.js ≥ 18 installed on your machine (for npx)
- A Vercel account

## Authentication

On first use, Zed will open your browser to authorize access
to your Vercel account via OAuth. Your session is then cached
locally — no API token to manage manually.

## Usage

Install the extension from the Zed Extensions panel, then open
the Agent Panel (`cmd+?`) and look for **Vercel** in the MCP
servers list. Click the green dot to confirm it's active.

## License

MIT
