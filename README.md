# DeFi Story

## Why?

## Getting Started

1. Install Diesel CLI

```bash
$ cargo install diesel_cli --no-default-features --features sqlite postgres
```

By default this installs a binary at ~/.cargo/bin

If there is an error when you install postgres, please install libpq

```bash
$ brew install libpq
```

2. Create docker-compose.yml file and create a new docker container

```bash
$ docker-compose up -d
```

3. Set DATABASE_URL in .env file, then run command

```bash
$ diset setup
```
