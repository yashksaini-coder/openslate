# openslate

A self-hosted markdown note-taking app. Fast, simple, private. Access your notes from any device.

**Single user only.** No sign-ups, no sharing, no complexity. Just notes.

## Deploy

### Docker (local)

```bash
git clone https://github.com/MrSheerluck/openslate.git
cd openslate
cp .env.example .env
docker compose up -d
```

Open **http://localhost:8080**.

### Digital Ocean (or any VPS)

**Quickest way** — create a Droplet (Ubuntu 24.04) and paste [`scripts/cloud-init.yaml`](scripts/cloud-init.yaml) into the **User Data** field. In 2–3 minutes your app is at `http://<ip>:8080`.

**Manual** — SSH in and run:

```bash
apt install -y docker.io docker-compose-v2
git clone https://github.com/MrSheerluck/openslate /opt/openslate
cd /opt/openslate
cp .env.example .env
sed -i "s/JWT_SECRET=.*/JWT_SECRET=$(openssl rand -hex 32)/" .env
docker compose up -d
```

**Custom domain + HTTPS** — add `DOMAIN=notes.example.com` to `.env`, point an A record to your IP, restart. Caddy provisions Let's Encrypt automatically.

Full guide: [docs/deployment.md](docs/deployment.md)

---

## Features

- **Rich text editing** — Tiptap editor with markdown support, syntax highlighting, tables, task lists
- **Tags** — Organize notes with tags, filter by tag
- **Keyboard shortcuts** — Cmd+Shift+P command palette, full keyboard nav
- **Full-text search** — Search across all notes instantly (SQLite FTS5)
- **Media** — Upload images and files, stored in Cloudflare R2 (optional)
- **Six themes** — Light, dark, sepia, nord, monokai, tokyo-night
- **Simple auth** — Single password, log in from any device (JWT cookies)
- **Self-contained** — One Docker container, SQLite database

---

## Stack

| Layer | Technology |
|-------|-----------|
| Frontend | SvelteKit + Tailwind CSS + Tiptap |
| Backend | Rust (Axum) |
| Database | SQLite |
| Auth | JWT (httpOnly cookies) |
| Media | Cloudflare R2 (optional) |
| Reverse proxy | Caddy (auto HTTPS) |

---

## Documentation

- [Setup Guide](docs/setup.md) — Prerequisites, env vars, running without Docker
- [Architecture](docs/architecture.md) — Project structure, data flow, design decisions
- [Features](docs/features.md) — Editor, tags, search, media, themes
- [API Reference](docs/api-reference.md) — Full REST API docs
- [Deployment](docs/deployment.md) — Docker, VPS, custom domain + HTTPS

---

## Development

```bash
# Backend
cd api && cargo run

# Frontend (separate terminal)
cd web && bun run dev
```

---

## LICENSE

MIT
