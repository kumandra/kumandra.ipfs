use online::sync::check;
use std::process::Stdio;
use std::{process::Command};
use std::io::prelude::*;

pub fn first_option() {

    // Check Internet Connection
    let is_online = check(None).is_ok();
    if !is_online {
        panic!("No Internet connection");
    }

    // let apps = vec!["go", "go-ipfs", "wget"].iter().map(|x| x.to_string()).collect::<Vec<String>>();
    let apps = vec!["go", "go-ipfs", "wget"];
    let mut not_installed: Vec<String> = Vec::new();

    apps.iter().for_each(|f| {

        let installed = is_installed(f);
        if !installed {
            not_installed.push(f.to_string());
        }
    });


    if not_installed.len() > 0 {

        // println!("Preparing to install: {:#?}", not_installed);
        install(&not_installed)
    }

    // // ipfs-cluster-bin hash
    // let hash = "https://gateway.kumandra.org/files/QmVju9rmvgdKS5ndzSrVrL3pcwn4JWtrnAQ2dUgnMnGfi3";

    // // Install ipfs-cluster-bin
    // install_ipfs_cluster_bin(hash);
}

pub fn second_options() {

    // Check Internet Connection
    let is_online = check(None).is_ok();
    if !is_online {
        panic!("No Internet connection");
    }

    // let apps = vec!["go", "go-ipfs", "wget"].iter().map(|x| x.to_string()).collect::<Vec<String>>();
    let apps = vec!["go", "go-ipfs", "wget"];
    let mut not_installed: Vec<String> = Vec::new();

    apps.iter().for_each(|f| {

        let installed = is_installed(f);
        if !installed {
            not_installed.push(f.to_string());
        }
    });


    if not_installed.len() > 0 {

        // println!("Preparing to install: {:#?}", not_installed);
        install(&not_installed)
    }

    // ipfs-cluster-bin hash
    let hash = "https://gateway.kumandra.org/files/QmVju9rmvgdKS5ndzSrVrL3pcwn4JWtrnAQ2dUgnMnGfi3";

    // Install ipfs-cluster-bin
    install_ipfs_cluster_bin(hash);
}
pub fn install_ipfs_cluster_bin(hash: &str) {
    // Check sudo
    sudo::escalate_if_needed().unwrap();

    let status = Command::new("which")
        .arg("ipfs-cluster-service")
        .output()
        .expect("Failed to execute process");

    if status.status.success() {
        return ();
    }
    Command::new("wget")
        .arg(hash)
        .arg("-O")
        .arg("ipfs-cluster-bin-1.0.1-1-x86_64.pkg.tar.zst")
        .output()
        .expect("failed to execute process");

    let result = Command::new("pacman")
        .args(&["-U", "--noconfirm"])
        .arg("ipfs-cluster-bin-1.0.1-1-x86_64.pkg.tar.zst")
        .output()
        .expect("failed to execute process");

    // Print output of the result
    std::io::stdout().write_all(result.stdout.as_ref()).unwrap();
}


fn install(apps: &Vec<String>) {
    // Check sudo
    sudo::escalate_if_needed().unwrap();

    println!("Preparing to install the dependencies");
    let cmd = Command::new("pacman")
        .args(&["-S", "--noconfirm"])
        .args(apps)
        .output()
        .expect("failed to execute process");

        std::io::stdout().write_all(cmd.stdout.as_ref()).unwrap();
        // println!("{:?}", apps)
}

fn is_installed(app: &str) -> bool {
    let output = Command::new("pacman")
        .arg("-Q")
        .arg(app)
        .output()
        .expect("failed to execute process");

    output.status.success()

}
