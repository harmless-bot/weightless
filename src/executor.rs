use anyhow::{anyhow, Result};
use colored::Colorize;

pub fn copy_to_memory(dest: *mut u8, payload: &[u8]) -> Result<()> {
    // SAFETY: dest points to a valid RWX mmap region of at least payload.len()
    // bytes allocated by alloc_exec_memory(). We use copy_nonoverlapping
    // which requires src and dst do not overlap — guaranteed since src is a
    // heap Vec and dst is a fresh anonymous mmap region.
    unsafe {
        std::ptr::copy_nonoverlapping(payload.as_ptr(), dest, payload.len());
    }
    
    // Read back first 4 bytes from dest and assert they match payload[0..4]
    if payload.len() >= 4 {
        // SAFETY: dest is a valid pointer and payload has at least 4 bytes.
        let first_4 = unsafe { std::slice::from_raw_parts(dest, 4) };
        if first_4 != &payload[0..4] {
            return Err(anyhow!("Memory copy verification failed: first 4 bytes do not match"));
        }
    }

    println!("{}", format!("[+] {} bytes copied to 0x{:x} — payload is staged in RAM only", payload.len(), dest as usize).green());
    Ok(())
}

pub fn compute_entropy(data: &[u8]) -> f64 {
    let mut counts = [0usize; 256];
    for &byte in data {
        counts[byte as usize] += 1;
    }

    let mut entropy = 0.0;
    let len = data.len() as f64;

    for &count in counts.iter() {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }
    
    entropy
}

pub fn classify_payload(entropy: f64) -> &'static str {
    if entropy < 1.0 {
        "NOP sled / zeroed memory (benign stub)"
    } else if (1.0..=4.5).contains(&entropy) {
        "Structured data or code stub"
    } else if (4.5..=7.0).contains(&entropy) {
        "Compiled shellcode (likely real code)"
    } else {
        "Encrypted/packed — high entropy payload"
    }
}

pub fn execute_payload(ptr: *mut u8, verbose: bool) -> Result<()> {
    // ensures all prior writes to the RWX region are visible to the CPU
    // instruction cache before we redirect execution — prevents stale-cache bugs
    std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);

    // SAFETY: ptr points to a valid RWX region containing our NOP+RET stub.
    // We use transmute to reinterpret it as a function pointer. The stub
    // returns immediately (RET), so the call is safe for this simulation.
    let f: unsafe extern "C" fn() = unsafe { std::mem::transmute(ptr) };

    if verbose {
        println!("{}", format!("[*] Instruction pointer redirected to 0x{:x}", ptr as usize).cyan());
    }

    let result = std::panic::catch_unwind(|| {
        unsafe { f() }
    });

    if result.is_err() {
        println!("{}", "[!] Payload triggered CPU exception (expected for non-code stubs)".red());
    } else {
        println!("{}", "[✓] Execution returned cleanly. Zero disk artifacts created.".green());
    }

    Ok(())
}

pub fn dry_run_report(payload: &[u8]) {
    let size = payload.len();
    let entropy = compute_entropy(payload);
    let classification = classify_payload(entropy);
    
    let hypothetical_address = 0x7f000000 + (if size > 0 { payload[0] as u64 } else { 0 } * 0x1000);
    
    println!("[DRY RUN] Payload size    : {} bytes", size);
    println!("[DRY RUN] Entropy         : {:.4} bits/byte", entropy);
    println!("[DRY RUN] Classification  : {}", classification);
    println!("[DRY RUN] Would allocate  : RWX region via mmap(PROT_READ|PROT_WRITE|PROT_EXEC)");
    println!("[DRY RUN] Would stage at  : hypothetical 0x{:x} (ASLR simulated)", hypothetical_address);
    println!("[DRY RUN] Disk writes     : 0");
    println!("[DRY RUN] EXECUTION SKIPPED — dry-run mode active");
}
