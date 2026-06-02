use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author = "Divyanshu Rai", version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "http://127.0.0.1:8080/nop_stub.bin")]
    pub url: String,

    #[arg(short, long, default_value_t = 4096)]
    pub size: usize,

    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[arg(short, long, default_value_t = 0)]
    pub delay: u64,

    #[arg(long, default_value_t = false)]
    pub hex_dump: bool,
}

pub fn print_banner(args: &Args) {
    let box_content = "\
  ╔══════════════════════════════════════════════════════╗\n  \
  ║       WEIGHTLESS  —  Fileless Loader Simulator       ║\n  \
  ║       by Divyanshu Rai | Chandigarh University       ║\n  \
  ╚══════════════════════════════════════════════════════╝"
        .yellow();

    println!("{}", box_content);

    let mode = if args.dry_run { "DRY RUN" } else { "LIVE EXECUTION" };
    println!("{}", format!("[*] Mode    : {}", mode).cyan());
    println!("{}", format!("[*] URL     : {}", args.url).cyan());
    println!("{}", format!("[*] Buffer  : {} bytes", args.size).cyan());
    println!("{}", "[!] FOR EDUCATIONAL AND CTF USE ONLY".red());
}
