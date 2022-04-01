fn main() {
    dependencies();
}

/// Check that required dependencies are installed
fn dependencies() {
    std::process::Command::new("ssh").output().expect("ssh is not installed");
    std::process::Command::new("az").output().expect("az cli is not installed");
    std::process::Command::new("cloud-init").output().expect("cloud-init is not installed");
}
