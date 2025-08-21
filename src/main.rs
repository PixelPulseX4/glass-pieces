#![feature(exit_status_error)]

use std::os::unix::fs::PermissionsExt;

use crate::ix::AngelInstruction;

mod ix;

async fn chmod(path: &str, mode: u32) -> Result<(), tokio::io::Error> {
    let metadata = tokio::fs::metadata(path).await?;

    let mut perm = metadata.permissions();
    perm.set_mode(mode);

    tokio::fs::set_permissions(path, perm).await?;

    Ok(())
}

async fn init_ssh_config(ix: &AngelInstruction) -> Result<(), tokio::io::Error> {
    let ssh_dir = format!("{}/.ssh", env!("HOME"));
    tokio::fs::create_dir_all(&ssh_dir).await?;
    chmod(&ssh_dir, 0o700).await?;

    let ssh_key = format!("{}/.ssh/id_ed25519", env!("HOME"));
    tokio::fs::write(&ssh_key, &ix.git_ssh_priv_key).await?;
    chmod(&ssh_key, 0o600).await?;

    let ssh_known_hosts = format!("{}/.ssh/known_hosts", env!("HOME"));
    tokio::fs::write(&ssh_known_hosts, &ix.git_known_hosts).await?;
    chmod(&ssh_known_hosts, 0o644).await?;

    Ok(())
}

async fn clone(ix: &AngelInstruction) -> Result<String, tokio::io::Error> {
    let clone_dst_dir = "/tmp/app";

    tokio::process::Command::new("git")
        .args(["clone", &ix.git_url, &clone_dst_dir])
        .spawn()?
        .wait()
        .await?
        .exit_ok()
        .expect("git clone failed");

    Ok(clone_dst_dir.to_owned())
}

async fn spawn(dir: String, ix: AngelInstruction) -> Result<u32, tokio::io::Error> {
    let child = tokio::process::Command::new("sh")
        .current_dir(dir)
        .args(["-c", &ix.entrypoint])
        .spawn()?;

    Ok(child.id().expect("spawn exited immediately"))
}

#[tokio::main]
async fn main() {
    let ix = ix::AngelInstruction::load_owned()
        .await
        .expect("failed to load instruction");

    init_ssh_config(&ix).await.expect("failed to config ssh");

    let dir = clone(&ix).await.expect("failed to clone");

    let pid = spawn(dir, ix).await.expect("failed to spawn");

    println!("{}", pid);
}
