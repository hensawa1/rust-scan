use clap::Parser;
use colored::*;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

/// Simple Async Port Scanner for Cybersecurity Portfolio
#[derive(Parser, Debug)]
#[command(author = "Henrique Carvalho", version = "1.0", about = "High-performance async port scanner built in Rust", long_about = None)]
struct Args {
    /// Target IP address to scan (e.g., 127.0.0.1)
    #[arg(short, long)]
    target: String,

    /// Start port range
    #[arg(short = 's', long, default_value_t = 1)]
    start_port: u16,

    /// End port range
    #[arg(short = 'e', long, default_value_t = 1024)]
    end_port: u16,
}

async fn scan_port(addr: SocketAddr) -> bool {
    // Timeout de 1 segundo para não travar em portas filtradas
    let timeout_duration = Duration::from_secs(1);
    match timeout(timeout_duration, TcpStream::connect(addr)).await {
        Ok(Ok(_)) => true, // Conexão estabelecida = Porta Aberta
        _ => false,        // Timeout ou Erro = Porta Fechada/Filtrada
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    println!("\n{}", "🚀 RustyScan Initialized...".blue().bold());
    println!("Scanning target: {}", args.target.yellow());
    println!("Range: {}-{}\n", args.start_port, args.end_port);

    let mut handles = vec![];

    // Cria uma task assíncrona para cada porta (Concorrência real)
    for port in args.start_port..=args.end_port {
        let target = args.target.clone();
        
        let handle = tokio::spawn(async move {
            let addr_str = format!("{}:{}", target, port);
            // Tenta converter string para SocketAddr e escanear
            if let Ok(socket_addr) = addr_str.parse::<SocketAddr>() {
                if scan_port(socket_addr).await {
                    println!("{} Port {} is OPEN", "[+]".green().bold(), port.to_string().cyan().bold());
                }
            }
        });
        handles.push(handle);
    }

    // Aguarda todas as tasks terminarem
    for handle in handles {
        let _ = handle.await;
    }

    println!("\n{}", "✅ Scan completed.".blue().bold());
}