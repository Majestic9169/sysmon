<div align="center">
  <img width="200" src="https://www.freepnglogos.com/uploads/doraemon-png/flying-broom-and-doraemon-png-download-19.png"/>
</div>

## Quickstart

Download executeable from [releases](https://github.com/Majestic9169/sysmon/releases)

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
make build
```

**3. Run**

In normal mode

```bash
make run
```

In live mode

```bash
make live
```
