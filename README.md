<div align="center">
  <img width="200" src="https://www.freepnglogos.com/uploads/doraemon-png/flying-broom-and-doraemon-png-download-19.png"/>
</div>

## Quickstart

assuming rust toolchain is installed

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
