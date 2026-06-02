# Weightless — Fileless Execution Simulator
> by Divyanshu Rai | Chandigarh University, Ludhiana

## What Is This?
Weightless is a controlled, educational fileless execution simulator designed to demonstrate how real-world Advanced Persistent Threats (APTs) and modern malware stage payloads directly in memory. By bypassing the disk entirely, fileless execution evades traditional Anti-Virus (AV) solutions that rely on scanning files written to the hard drive. This project simulates this tradecraft in a safe, isolated lab environment using a benign NOP+RET stub. It highlights the in-memory staging techniques often seen in frameworks like Cobalt Strike.

## Architecture
```text
[Remote HTTP Server]
       │  HTTP GET /nop_stub.bin
       ▼
[fetcher.rs] ──── Vec<u8> bytes (never touches disk)
       │
       ▼
[mem_alloc.rs] ── mmap(PROT_READ|PROT_WRITE|PROT_EXEC)
       │             allocates RWX buffer in RAM
       ▼
[executor.rs] ─── copy_nonoverlapping() stages payload
       │
       ▼
[CPU] ◄─────────── transmute(*mut u8 → fn()) + call
       │
[mem_alloc.rs] ── munmap() frees region
       │
       ▼
DISK WRITES: 0
```

## Quick Start
```bash
cd test_payloads && python3 generate_stub.py
python3 payload_server/serve.py &
cargo build --release
./target/release/weightless --dry-run --hex-dump --verbose
sudo ./target/release/weightless --verbose
```

## CLI Flags Table
| Flag | Description | Default |
|------|-------------|---------|
| `-u, --url` | URL to fetch the payload from | `http://127.0.0.1:8080/nop_stub.bin` |
| `-s, --size` | Allocation buffer size in bytes | `4096` |
| `--dry-run` | Fetch and analyze only, do NOT execute | `false` |
| `-v, --verbose` | Print every internal step with memory addresses | `false` |
| `-d, --delay` | Milliseconds to sleep before execution (AV evasion) | `0` |
| `--hex-dump` | Print first 64 bytes of payload as xxd-style hex dump | `false` |

## How It Works — Technical Deep Dive
* **`mmap` flags & RWX:** The tool allocates memory using the `mmap` syscall with the flags `PROT_READ | PROT_WRITE | PROT_EXEC` (RWX). This violates the typical **W^X (Write XOR Execute)** mitigation (which prevents a page from being writable and executable at the same time), allowing the program to write data (the payload) to memory and execute it shortly after.
* **Memory Fences:** `std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst)` is used to ensure all prior writes to the memory region are completely visible to the CPU's instruction cache before redirecting the execution pointer, preventing stale instruction executions.
* **Function Pointers (`transmute`):** A pointer cast via `transmute` changes the data pointer into an executable function pointer `fn()`, safely allowing rust to transition the control flow to the newly placed raw bytes.
* **Permissions (Root):** Running this executable typically requires `sudo` (root privileges) depending on OS and hypervisor constraints for allocating executable/anonymous pages, especially to simulate true system-level access that advanced attacks mimic.

## Legal Disclaimer
**FOR EDUCATIONAL AND CTF USE ONLY.** The techniques described and implemented in this project are for cybersecurity research, CTF (Capture The Flag) competitions, and defensive educational purposes. The provided stub is completely benign (NOP+RET). Do not use this tool for any malicious activities. The author assumes no liability for the misuse of this tool.

## About the Author — Divyanshu Rai
Divyanshu Rai is a cybersecurity student and CTF competitor based at Chandigarh University, Ludhiana. He is passionate about offensive security engineering, systems programming, and modern defense evasion tradecraft.
