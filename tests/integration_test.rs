use std::time::Duration;
use tokio::process::Command;
use tokio::time::sleep;

#[tokio::test]
async fn test_daemon_cil_communication() -> anyhow::Result<()> {
    let test_id = std::process::id();
    let socket_path = format!("/tmp/tmux-botdomo-test-{test_id}.sock");

    let mut daemon = Command::new("cargo")
        .args(["run", "--bin", "tbdd", "start"])
        .env("TMUX_BOTDOMO_SOCK_PATH", &socket_path)
        .spawn()?;


    sleep(Duration::from_millis(500)).await;
    if tokio::fs::try_exists(&socket_path).await.is_err() {
        anyhow::bail!("Socket not ready");
    }

    let output = Command::new("cargo")
        .args(["run", "--bin", "tbd", "send", "hello test"])
        .env("TMUX_BOTDOMO_SOCK_PATH", &socket_path)
        .output()
        .await?;

    // Clean-up, maybe RAII?
    let _ = daemon.kill().await;
    let _ = std::fs::remove_file(&socket_path);

    // Assertions
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Sending: hello test"));

    Ok(())
}
