/*

*/

#[macro_use]
extern crate clap;
extern crate dialoguer;
use clap::{App, AppSettings, Arg};
use dialoguer::{theme::ColorfulTheme, Input, PasswordInput};

use std::process;

struct cliArgs {
    AccountName: String,
    AccountPass: String,
    HostURL: String,
}

impl cliArgs {
	fn new() -> Self {
		// add code here
		let mut accountName = String::new();
	    let mut accountPass = String::new();
	    let mut hostURL = String::new();
	    let mut silentMode = false;
	
	    let matches = App::new("Bitwarden Sync")
	        .version("1.0")
	        .author("")
	        .about("Backup bitwarden to a keepass compatible csv, or restores it to your account.")
	        .arg(
	            Arg::with_name("account")
	                .short("a")
	                .long("account")
	                .takes_value(true)
	                .required(false)
	                .help("Set bitwarden account for backup or restore"),
	        )
	        .arg(
	            Arg::with_name("host")
	                .short("h")
	                .long("host")
	                .takes_value(true)
	                .required(false)
	                .help("Set bitwarden host url"),
	        )
	        .arg(
	            Arg::with_name("password")
	                .short("p")
	                .long("password")
	                .takes_value(true)
	                .required(false)
	                .help("Sets password for bitwarden account"),
	        )
	        .arg(
	            Arg::with_name("silent")
	                .short("s")
	                .long("silent")
	                .required(false)
	                .help("Run tool silently without stdout"),
	        )
	        .subcommand(
	            App::new("backup")
	                .about("Backup bitwarden account to keepass compatible csv")
	                .setting(AppSettings::SubcommandRequiredElseHelp)
	                .arg(
	                    Arg::with_name("encrypt")
	                        .short("e")
	                        .long("encrypt")
	                        .required(false)
	                        .help("Encrypt csv with bitwarden password (default=true)"),
	                ),
	        )
	        .subcommand(
	            App::new("restore")
	                .about("Restore bitwarden account to keepass compatible csv")
	                .setting(AppSettings::SubcommandRequiredElseHelp)
	                .arg(
	                    Arg::with_name("override")
	                        .short("o")
	                        .long("override")
	                        .required(false)
	                        .help("override existing entries in bitwarden account (default=false)"),
	                ),
	        )
	        .get_matches();
	
	    if let Some(temp) = matches.value_of("silent") {
	        silentMode = true;
	    }
	
	    if let Some(tempAccount) = matches.value_of("account") {
	        accountName = tempAccount.to_string();
	    } else {
	        let promptAccount = Input::<String>::new()
	            .with_prompt("bitwarden account:")
	            .interact()
	            .unwrap();
	
	        accountName = promptAccount;
	    }
	
	    if let Some(tempURL) = matches.value_of("host") {
	        hostURL = tempURL.to_string();
	    } else {
	        let promptHost = Input::<String>::new()
	            .with_prompt("bitwarden host:")
	            .interact()
	            .unwrap();
	
	        hostURL = promptHost;
	    }
	
	    if let Some(tempPass) = matches.value_of("password") {
	        accountPass = tempPass.to_string();
	    } else {
	        let promptPassword = PasswordInput::with_theme(&ColorfulTheme::default())
	            .with_prompt("Password:")
	            .interact()
	            .unwrap();
	
	        accountPass = promptPassword;
	    }
	
	    if let Some(ref matches) = matches.subcommand_matches("backup") {
	        println!("backing up account to: 123456.csv");
	    } else if let Some(ref matches) = matches.subcommand_matches("restore") {
	        println!("restoring file to account...");
	    } else {
	        println!("invalid operation, please choose to 'backup' or 'restore'");
	        process::exit(0x0100);
	    }

	    cliArgs {
	    AccountName: accountName,
    	AccountPass: accountPass,
    	HostURL: hostURL,
		}
	}
}

fn main() {
    // let mut accountName = String::new();
    // let mut accountPass = String::new();
    // let mut hostURL = String::new();
    // let mut silentMode = false;


    let instanceArgs = cliArgs::new();
    println!("\n Bitwarden Sync\n----------------\n");

    println!(
        "* using account '{0}' on bitwarden host '{1}'",
        instanceArgs.AccountName, instanceArgs.HostURL
    );
}
