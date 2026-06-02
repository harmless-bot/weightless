//! What PROT_EXEC does: It marks memory pages as CPU-executable, meaning the CPU
//! will fetch and execute instructions from these pages. By default, data pages are not executable.
//!
//! W^X (Write XOR Execute) is a security feature that ensures memory pages are either
//! writable or executable, but never both simultaneously. This prevents attackers from
//! writing malicious code to a data buffer and executing it. In this project, we intentionally
//! violate W^X by allocating RWX (Read, Write, Execute) memory to simulate fileless loader behaviour.
//!
//! Legitimate JIT (Just-In-Time) compilers (like V8 for JavaScript) also need RWX memory
//! to write generated machine code and execute it. APTs (Advanced Persistent Threats) and malware
//! abuse this by allocating RWX memory to stage payloads directly in RAM, bypassing disk-based
//! AV (Anti-Virus) detection.
//!
//! This technique is directly analogous to real-world tools like Cobalt Strike's in-memory staging,
//! where payloads are injected directly into executable memory without ever touching the disk.

use anyhow::Result;
#[cfg(not(unix))]
use anyhow::anyhow;
use colored::Colorize;

pub fn alloc_exec_memory(size: usize, verbose: bool) -> Result<*mut u8> {
    #[cfg(unix)]
    {
        // SAFETY: We are deliberately allocating RWX memory to simulate fileless
        // loader behaviour for a CTF/educational project. The pointer is valid for
        // `size` bytes and is only cast to *mut u8. Caller must call
        // dealloc_exec_memory() when done to prevent a memory leak.
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut::<libc::c_void>(),
                size,
                libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
                libc::MAP_ANON | libc::MAP_PRIVATE,
                -1,
                0,
            )
        };

        if ptr == libc::MAP_FAILED {
            return Err(anyhow::Error::from(std::io::Error::last_os_error()));
        }

        if verbose {
            println!("{}", format!("[+] mmap() → RWX region at 0x{:x}, {} bytes", ptr as usize, size).green());
        }

        Ok(ptr as *mut u8)
    }
    #[cfg(not(unix))]
    {
        Err(anyhow!("alloc_exec_memory is only supported on Unix systems"))
    }
}

pub fn dealloc_exec_memory(ptr: *mut u8, size: usize) -> Result<()> {
    #[cfg(unix)]
    {
        // SAFETY: `ptr` was allocated by `mmap` in `alloc_exec_memory` and is being
        // freed here using `munmap`. `size` must match the allocated size.
        let ret = unsafe { libc::munmap(ptr as *mut libc::c_void, size) };

        if ret != 0 {
            return Err(anyhow::Error::from(std::io::Error::last_os_error()));
        }

        println!("{}", "[+] munmap() — RWX region freed. Disk footprint: 0 bytes.".green());
        Ok(())
    }
    #[cfg(not(unix))]
    {
        Err(anyhow!("dealloc_exec_memory is only supported on Unix systems"))
    }
}
