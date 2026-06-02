# openslate

A self-hosted markdown note-taking app. Fast, simple, private. Access your notes from any device.

**Single user only.** No sign-ups, no sharing, no complexity. Just notes.

## Features

- **Rich markdown editing** - Write in markdown, see it rendered as you type (Milkdown/ProseMirror)
- **Wiki links** - `[[link notes]]` with automatic backlinks
- **Tags** - Organize notes with tags, filter by tag
- **Keyboard shortcuts** - Cmd+K command palette, Cmd+N new note, full keyboard nav
- **Full-text search** - Search across all notes instantly (SQLite FTS5)
- **Media** - Upload images and files, stored in Cloudflare R2
- **Simple auth** - Single password, log in from any device (JWT cookies)
- **Self-contained** - One binary + SQLite, or a single Docker container

## Stack

- **Frontend:** SvelteKit + Tailwind CSS + Milkdown
- **Backend:** Rust (Axum)
- **Database:** SQLite
- **Auth:** JWT via httpOnly cookies
- **Media:** Cloudflare R2

## Development


```bash
cd api && cargo run
```


```bash
cd web && bun run dev
```

## Deployment

Run as a single Docker container or separate backend (VPS) + frontend (Cloudflare Pages).

## LICENSE
MIT
