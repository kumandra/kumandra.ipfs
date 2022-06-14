mod utils;
use utils::options::option;

fn main() {
    let mut input = String::new();
    let mut output = option(input);
    if output.ends_with('\n') {
        output.pop();
        if output.ends_with('\r') {
            output.pop();
        }
    }

    if output == "1" {
        println!("You choose number 1 ");
        // Todo! for join normal peers
        // Check Linux Distro (debian or arch)
        // Install dependencies for the distro
    }
    else if output == "2" {
        println!("You choose number 2 ");
        // Todo! for join normal peers + cluster
        // Check Linux Distro (debian or arch)
        // Install dependencies for the distro
    }
    else {
        println!("Ops! Look like you enter the wrong number: ");
    }
}
