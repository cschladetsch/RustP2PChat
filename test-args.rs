fn main() {
    println!("Raw args: {:?}", std::env::args().collect::<Vec<_>>());
    println!("Args count: {}", std::env::args().len());
    
    // Test what clap sees
    use clap::Parser;
    
    #[derive(Parser, Debug)]
    struct TestCli {
        #[arg(short, long)]
        port: Option<u16>,
        
        #[arg(short, long)]
        connect: Option<String>,
        
        #[arg(long)]
        gui: bool,
    }
    
    match TestCli::try_parse() {
        Ok(args) => println!("Parsed args: {:?}", args),
        Err(e) => println!("Parse error: {}", e),
    }
}