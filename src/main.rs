// Copyright © 2019 Patrick J. Trottier. All rights reserved.
// The following code is covered by the GPL-3.0 license.

#[macro_use]
extern crate clap; 
use clap::App; 
 
fn main() { 
    // Load configurations
    let yaml = load_yaml!("cli.yml");

    // Get the input from user
    let matches = App::from_yaml(yaml).get_matches();

    // Parse the input from user
    let source = matches.value_of("source").unwrap();
    let target = matches.value_of("target").unwrap();
    let username = matches.value_of("username").unwrap();

    // Display the commands
    let tunnel = format!("ssh -L 1234:mariadb.cs.laurentian.ca:22 {}@shell.cs.laurentian.ca", username.to_lowercase());
    let copy = format!("scp -P 1234 {}@127.0.0.1:{} {}", username, source, target); 
    println!("1. Open a shell and provide your password for shell after running:\n{}\n\n2. With the shell from the previous step idling, open a new shell and provide your mariadb password after running:\n{}", tunnel, copy);
}
