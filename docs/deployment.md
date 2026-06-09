# Deployment

OpenSlate runs in a single Docker container: Rust backend + SvelteKit frontend + Caddy reverse proxy. SQLite stores all data in a persistent Docker volume.

## Option 1: Local Docker

Run OpenSlate entirely on your own machine. No domain needed, no cloud accounts required.

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose (v2)

### Steps

```bash
git clone https://github.com/MrSheerluck/openslate.git
cd openslate
cp .env.example .env
```

Edit `.env` — set at minimum `JWT_SECRET`:

```
JWT_SECRET=$(openssl rand -hex 32)
```

Start:

```bash
docker compose up -d
```

The first build compiles Rust (~10–20 min). Subsequent starts are instant.

Open **http://localhost:8080**. On first visit, you'll see "Set your admin password" — choose a password and click "Create account". That password is your permanent login.

### Custom port

```bash
PORT=3000 docker compose up -d
```

### Optional: media uploads

Add R2 credentials to `.env` and restart:

```bash
docker compose down && docker compose up -d
```

---

## Option 2: Digital Ocean VPS

### Automatic (cloud-init)

1. Create a Droplet (Ubuntu 24.04, cheapest $4/mo plan is plenty).
2. Under **Advanced Options → User Data**, paste the contents of [`scripts/cloud-init.yaml`](../scripts/cloud-init.yaml).
3. Create the Droplet. Wait 2–3 minutes.

Your app is live at `http://<droplet-ip>:8080`.

### Manual

SSH into your VPS:

```bash
ssh root@<vps-ip>

apt update && apt install -y docker.io docker-compose-v2 git
systemctl enable --now docker

git clone https://github.com/MrSheerluck/openslate /opt/openslate
cd /opt/openslate
cp .env.example .env
sed -i "s/JWT_SECRET=.*/JWT_SECRET=$(openssl rand -hex 32)/" .env
docker compose up -d
```

---

## Custom Domain + HTTPS

### 1. Point DNS

Add an **A record** for your domain pointing to your server's IP:

| Type | Name | Value |
|------|------|-------|
| A    | notes | `<server-ip>` |

### 2. Update .env

```bash
DOMAIN=notes.example.com
```

### 3. Restart

```bash
docker compose down && docker compose up -d
```

Caddy automatically provisions a Let's Encrypt TLS certificate. Your app is now at **https://notes.example.com**.

---

## Updates

Pull the latest code and rebuild:

```bash
cd /opt/openslate
git pull
docker compose up -d --build
```

Your SQLite database in the `data` volume is preserved across updates.

---

## Backups

The SQLite database lives in a Docker volume. Back it up:

```bash
# Copy the database out of the container
docker compose cp openslate:/data/data.db ./backup.db

# Or find and copy the volume file directly
docker compose down
cp $(docker volume inspect openslate_data --format '{{.Mountpoint}}')/data.db ./backup.db
docker compose up -d
```

---

## Troubleshooting

**Build fails** — Ensure you have at least 2 GB RAM free for the Rust compilation step.

**Can't log in** — Check the admin password in logs: `docker compose logs | grep -i password`

**Port already in use** — Use `PORT=3000 docker compose up -d` to use a different port.

**Media uploads don't work** — Verify all four `R2_*` variables are set in `.env` and restart.
