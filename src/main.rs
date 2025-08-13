#![feature(exit_status_error)]

mod ix;

#[tokio::main]
async fn main() {
    let ix = ix::AngelInstruction::load_owned()
        .await
        .expect("failed to load instruction");

    tokio::fs::write(
        format!("{}/.ssh/id_ed25519", env!("HOME")),
        ix.git_ssh_priv_key,
    )
    .await
    .expect("write ssh private key");

    tokio::fs::write(
        format!("{}/.ssh/known_hosts", env!("HOME")),
        ix.git_known_hosts,
    )
    .await
    .expect("write ssh known hosts");

    tokio::process::Command::new("git")
        .args(["clone", &ix.git_url])
        .spawn()
        .expect("spawn git clone")
        .wait()
        .await
        .expect("wait git clone")
        .exit_ok()
        .expect("git clone failed");

    let child = tokio::process::Command::new("sh")
        .args(["-c", &ix.entrypoint])
        .spawn()
        .expect("spawn final boss");

    if let Some(pid) = child.id() {
        println!("input is being processed: {}", pid);
    } else {
        println!("spawn exited immediately");
    }
}
