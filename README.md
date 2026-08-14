<div align="center">
  <div>
    <img width="200" src="./sysmon.png"/>
  </div>
  linux process monitor
</div>

## Quickstart

Download executeable from [releases](https://github.com/Majestic9169/sysmon/releases) or get it from `./target/release/` after building

```bash
./sysmon
```

```bash
./sysmon -T 5
```

```bash
❯ ./sysmon --help
System Monitor for Linux

Usage: sysmon [OPTIONS]

Options:
  -T <interval>
          time interval to refresh for live mode
  -h, --help
          Print help
```

## Nix Setup

```bash
# setup dev-shell
nix develop

# run debug mode
nix run

# build prod
nix build
```

## Non-Autistic Normal Setup

**1. Installation**

[Installation Guide](https://rust-lang.org/tools/install/)

```bash
#linux/macos
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**2. Build**

```bash
make build # cargo build --release
```

**3. Run**

In normal mode

```bash
make run   # cargo run --release
```

In live mode

```bash
make live  # cargo run --release -- -T 5
```
