# Deployment

OpenSlate runs in a single Docker container: Rust backend + SvelteKit frontend + Caddy reverse proxy. SQLite stores all data in a persistent Docker volume.

---

## Option 1: Local Docker

Run entirely on your own machine. No domain, no cloud accounts.

### Prerequisites

- [Docker Desktop](https://www.docker.com/products/docker-desktop/) (Mac/Windows) or Docker Engine (Linux)

### Steps

```bash
git clone https://github.com/MrSheerluck/openslate.git
cd openslate
cp .env.example .env
```

Edit `.env` — set a `JWT_SECRET`:

```
JWT_SECRET=your-random-secret-here
```

Start:

```bash
docker compose up -d
```

The first build compiles Rust (10–20 min on Apple Silicon, longer on Intel). Open **http://localhost:8080** — set your admin password on first visit.

### Custom port

```bash
docker compose up -d   # default: http://localhost:8080
```

Or edit `docker-compose.yml` to change the left-hand port number.

### Media uploads (optional)

Add R2 credentials to `.env`:
```
R2_BUCKET=your-bucket
R2_ACCOUNT_ID=your-id
R2_ACCESS_KEY=your-key
R2_SECRET_KEY=your-secret
```

Then restart: `docker compose down && docker compose up -d`.

---

## Option 2: Digital Ocean VPS

### Method A: Cloud-init (automatic)

1. Log into [DigitalOcean](https://cloud.digitalocean.com), click **Create → Droplets**.
2. Choose **Ubuntu 24.04 LTS**.
3. Pick the cheapest plan: **$4/mo Basic** (1 vCPU, 512 MB RAM, 10 GB SSD).
4. Scroll to **Advanced Options → User Data** — paste the entire contents of [`scripts/cloud-init.yaml`](../scripts/cloud-init.yaml).
5. Click **Create Droplet**. Wait 3–5 minutes.

Open `http://<droplet-ip>:8080`. On first visit, set your admin password.

> **Note:** The $4 droplet has 512 MB RAM which is **not enough to compile Rust** during `docker build`. The cloud-init script will fail at the build step. See [Solving the RAM problem](#solving-the-ram-problem-on-the-4-droplet) below.

### Method B: Manual (with Docker Hub/GHCR)

Build the image on your local machine, push to a registry, then pull on the VPS. This avoids the RAM problem entirely.

#### Step 1: Build and push from your machine

```bash
# Login to GitHub Container Registry
echo "your-github-token" | docker login ghcr.io -u YOUR_USERNAME --password-stdin

# Build for VPS architecture (linux/amd64)
docker buildx create --use --name multiarch
docker buildx build --platform linux/amd64 \
  -t ghcr.io/YOUR_USERNAME/openslate:latest \
  --push .
```

To create a GitHub token: **GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)**. Check `read:packages` and `write:packages`.

#### Step 2: Set up the VPS

SSH into your Droplet (IP shown in DO dashboard):

```bash
ssh root@<droplet-ip>
```

Install Docker:

```bash
apt update
apt install -y docker.io docker-compose-v2
systemctl enable --now docker
```

Clone the repo and configure:

```bash
git clone https://github.com/MrSheerluck/openslate /opt/openslate
cd /opt/openslate
cp .env.example .env
sed -i "s/JWT_SECRET=.*/JWT_SECRET=$(openssl rand -hex 32)/" .env
```

Edit `docker-compose.yml` — replace `build: .` with the image:

```bash
sed -i 's#build: .#image: ghcr.io/YOUR_USERNAME/openslate:latest#' docker-compose.yml
```

Login and start:

```bash
echo "your-github-token" | docker login ghcr.io -u YOUR_USERNAME --password-stdin
docker compose up -d
```

Open `http://<droplet-ip>:8080` and set your admin password.

---

## Custom Domain + HTTPS

### 1. Point DNS

In your domain's DNS provider (Cloudflare, Namecheap, GoDaddy, etc.), add an **A record**:

| Type | Name  | Value             |
|------|-------|-------------------|
| A    | notes | `<droplet-ip>`   |

This routes `notes.yourdomain.com` to your server. DNS propagation takes 2–5 minutes.

### 2. Update .env on the VPS

```bash
cd /opt/openslate
sed -i 's/DOMAIN=/DOMAIN=notes.yourdomain.com/' .env
```

### 3. Restart

```bash
docker compose down && docker compose up -d
```

Caddy automatically contacts Let's Encrypt, proves you own the domain, and provisions a TLS certificate. No manual cert setup, no renewal cron jobs — Caddy handles everything.

Your app is now at `https://notes.yourdomain.com`.

### How it works

- The entrypoint script detects `DOMAIN` is set and rewrites the Caddy config from `:8080` to `notes.yourdomain.com`.
- Caddy (with a proper domain name) automatically uses ports 80/443 and provisions TLS.
- `docker-compose.yml` already exposes ports 80, 443, and 8080 — no changes needed.
- Caddy stores certificates in `/data/caddy/` (persisted in the Docker volume).

---

## Solving the RAM problem on the $4 droplet

The cheapest DigitalOcean droplet (512 MB RAM) cannot compile Rust. The compiler needs 1–2 GB. Solutions, in order of simplicity:

### 1. Build on your machine, push to registry (recommended)

As shown in [Method B](#method-b-manual-with-docker-hubghcr) above. Build once locally, push to GitHub Container Registry (free for public repos), pull on the VPS. The VPS never compiles anything.

```bash
# Local: build once
docker buildx build --platform linux/amd64 -t ghcr.io/you/openslate:latest --push .

# VPS: just pull
docker compose pull && docker compose up -d
```

### 2. Upgrade to the $12 droplet

2 GB RAM — enough to compile. Simplest approach but costs more per month.

### 3. Use swap space

Add 2 GB of swap as a fallback (much slower builds, but works):

```bash
fallocate -l 2G /swapfile
chmod 600 /swapfile
mkswap /swapfile
swapon /swapfile
echo '/swapfile none swap sw 0 0' >> /etc/fstab
```

---

## Environment Variables

All config lives in `/opt/openslate/.env`:

| Variable       | Required | Description                          |
|----------------|----------|--------------------------------------|
| `JWT_SECRET`   | Yes      | Random string for signing auth tokens |
| `DOMAIN`       | No       | Your domain for HTTPS (e.g. `notes.example.com`) |
| `R2_BUCKET`    | No       | Cloudflare R2 bucket for media       |
| `R2_ACCOUNT_ID`| No       | Cloudflare account ID                |
| `R2_ACCESS_KEY`| No       | R2 access key                        |
| `R2_SECRET_KEY`| No       | R2 secret key                        |

After changing any value, run: `docker compose down && docker compose up -d`

---

## Updates

Pull the latest code and rebuild:

```bash
cd /opt/openslate
git pull
```

If using pre-built images (recommended):

```bash
docker buildx build --platform linux/amd64 -t ghcr.io/you/openslate:latest --push .
# then on VPS:
docker compose pull && docker compose up -d
```

If building on the VPS (only if you have enough RAM):

```bash
docker compose up -d --build
```

Your SQLite database in the Docker volume is preserved across all updates.

---

## Backups

The SQLite database lives in a Docker volume. Back it up:

```bash
cd /opt/openslate
docker compose exec openslate cp /data/data.db /data/data-backup.db
docker compose cp openslate:/data/data-backup.db ./backup-$(date +%Y%m%d).db
```

Or find the volume location directly:

```bash
docker compose down
cp $(docker volume inspect openslate_data --format '{{.Mountpoint}}')/data.db ./backup.db
docker compose up -d
```

---

## Troubleshooting

**"This site can't be reached"** — Run `docker compose logs --tail 30` and check for errors. Common causes:
- R2 credentials as empty strings (fixed in latest). Update the image.
- Caddyfile error. The entrypoint now handles this.
- Port not exposed. Verify `docker compose ps` shows port mappings.

**Can't log in** — The first visit to the app should show "Set your admin password." You create the account yourself. If you already did, use the same password.

**Build fails on VPS** — Out of memory. See [Solving the RAM problem](#solving-the-ram-problem-on-the-4-droplet).

**Wrong platform error** — `linux/arm64/v8` vs `linux/amd64/v3`. You built on Apple Silicon but need AMD64. Use `--platform linux/amd64` in your build command.

**Media uploads don't work** — Check `docker compose logs | grep -i r2`. Verify all four `R2_*` variables are set in `.env` with non-empty values.
