#[macro_use]
extern crate clap; 
use clap::App; 
 
fn main() { 
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let source = matches.value_of("source").unwrap();
    let target = matches.value_of("target").unwrap();
    let username = matches.value_of("username").unwrap();

    let tunnel = format!("ssh -L 1234:mariadb.cs.laurentian.ca:22 {}@shell.cs.laurentian.ca", username.to_lowercase());
    let copy = format!("scp -P 1234 {}@127.0.0.1:{} {}", username, source, target); 
    println!("Wants your password for shell:\n{}\nWants your mariadb password:\n{}", tunnel, copy);
}
