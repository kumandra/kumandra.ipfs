use std::process::Command;

pub fn check_distro() -> String {

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "echo Window is not supported right now"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("lsb_release")
                .arg("-is")
                .output()
                .expect("failed to execute process")
    };

    let mut distro = output.stdout;
    let mut distro = String::from_utf8(distro).unwrap();

    if distro.ends_with('\n') {
        distro.pop();
        if distro.ends_with('\r') {
            distro.pop();
        }
    }


    return distro;


}
