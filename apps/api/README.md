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
