use anyhow::Result;
use clap::Parser;
use docker::docker;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    cluster: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = docker::Config { 
        cluster_mode: cli.cluster, 
        ..Default::default() 
    };
    let mut component = docker::new(config);
    
    component.initialize()?;
    component.deploy()?;
    
    println!("SigmaOS-docker deployed");
    Ok(())
}
