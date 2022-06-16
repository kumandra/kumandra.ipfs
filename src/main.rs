mod utils;
use utils::options::option;
use utils::check_distro::check_distro;
use utils::arch_installation::{first_option, second_options, generate_ipfs_config};
use online::sync::check;

// fn main() {
//     let mut input = String::new();
//     let mut output = option(input);
//     if output.ends_with('\n') {
//         output.pop();
//         if output.ends_with('\r') {
//             output.pop();
//         }
//     }

//     if output == "1" {
//         println!("You choose number 1 ");
//         // Todo! for join normal peers
//         // Check Linux Distro (debian or arch)
//         let distro = check_distro();
//         if distro == "Arch" {
//             // Install Arch Dependencies
//             println!("Installing Arch Dependencies now: ");
//             // Call function to install dependencies
//             first_option();

//         }
//         else if distro == "Debian" {
//             // Install Debian Dependencies
//             println!("Installing Debian Dependencies now: ");
//             // Call function to install dependencies
//         }

//         else if distro == "Ubuntu" {
//             // Install Ubuntu Dependencies
//             println!("Installing Ubuntu Dependencies now: ");
//             // Call function to install dependencies
//         }
//         else {
//             println!("Your distro is not supported: ");
//         }
//     }
//     else if output == "2" {
//         println!("You choose number 2 ");
//         // Todo! for join normal peers + cluster
//         // Check Linux Distro (debian or arch)
//         let distro = check_distro();
//         if distro == "Arch" {
//             // Install Arch Dependencies
//             println!("Installing Arch Dependencies now: ");

//             // Call function to install dependencies
//             second_options();
//         }
//         else if distro == "Debian" {
//             // Install Debian Dependencies
//             println!("Installing Debian Dependencies now: ");
//             // Call function to install dependencies
//         }

//         else if distro == "Ubuntu" {
//             // Install Ubuntu Dependencies
//             println!("Installing Ubuntu Dependencies now: ");
//             // Call function to install dependencies
//         }
//         else {
//             println!("Your distro is not supported: ");
//         }

//     }
//     else {
//         println!("Ops! Look like you enter the wrong number: ");
//     }
// }


fn main() {

    // arch_installation();
    generate_ipfs_config();


}
