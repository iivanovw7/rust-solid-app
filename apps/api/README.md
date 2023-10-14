# rust-solid-app-api

Actix `REST` webserver.

### Main crates

-   [actix-web](https://crates.io/crates/actix-web)
-   [chrono](https://crates.io/crates/chrono)
-   [derive_more](https://crates.io/crates/derive_more)
-   [diesel](https://crates.io/crates/diesel)
-   [dotenv](https://crates.io/crates/dotenv)
-   [env_logger](https://crates.io/crates/env_logger)
-   [r2d2](https://crates.io/crates/r2d2)
-   [serde](https://crates.io/crates/serde)

### Requirements

-   Cargo 1.72.0
-   Postgres (PostgreSQL) 15.4
-   Rustc 1.72.0 (5680fa18f 2023-08-23) (Arch Linux rust 1:1.72.0-1)

### Installation

`.env` file example

```bash
AUTH_SALT=CHANGEME
DOMAIN="localhost"
DATABASE_URL=postgres://xxx:pass@localhost/xxxx_db
RUST_LOG="rest_api=info,acrix=info,actix_web=info,actix_server=info,actix_redis=traced,diesel_migrations=info"
SERVER=0.0.0.0:8088
SESSION_KEY=4125442A472D4B614
SESSION_NAME=auth
SESSION_PATH=/
SESSION_SECURE=false
SESSION_TIMEOUT=20

```

Setup `db` locally.

```bash
# install
pacman -S postgresql
# initialize
sudo -u postgres initdb --locale en_US.UTF-8 -D /var/lib/postgres/data
# systemctl
systemctl start postgresql
systemctl enable postgresql
systemctl status postgresql

sudo su
sudo - postgres
# list databases
\l
psql -c "\l"
# list users
\du
psql -c "\du"

# create database
CREATE DATABASE mydb

# user
CREATE USER xxxx WITH ENCRYPTED PASSWORD 'password';
GRANT ALL PRIVILEGES ON DATABASE db to xxxxx;
```

[More information](http://diesel.rs/guides/getting-started)

```bash
cargo install diesel_cli
# or
cargo install diesel_cli --no-default-features --features postgres

diesel migration run

cargo run
# or
cargo watch -x run
```

### Tests

```bash
cargo test
```

### Docs

After server is alive, swagger can be found `http://localhost:8088/api/swagger-ui/#`

### Nginx

Sample `nginx` configuration

```bash
server {
    location {
        proxy_pass http://localhost:8088;
        proxy_http_version 1.1;
        proxy_set_header X-Forwarded-Host $host;
        proxy_set_header X-Forwarded-Server $server;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Real-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Real-Proto $scheme;
        proxy_set_header Host $http_host;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "Upgrade";
        proxy_pass_request_headers on;
    }

    gzip on;
    gzip_types text/html text/css image/jpg image/jpeg image/png image/svg;

    listen [::]:443 ssl ipv6only=on;
    listen 443 ssl;
    ssl_certificate /etc/letsencrypt/live/example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/example.com/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;
}

server {
    if ($host = www.example.com) {
        return 301 https://$host$request_uri;
    }

    if ($host = example.com) {
        return 301 https://$host$request_uri;
    }

    listen 80;
    listen [::]:80;
    server_name example.com www.example.com;

    return 404;
}



```
