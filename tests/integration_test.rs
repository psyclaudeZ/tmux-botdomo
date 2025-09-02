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

    // Assertions
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Sending: hello test"));

    // Send SIGTERM to gracefully shut down the daemon
    let daemon_id = daemon.id().expect("Failed to get daemon PID");
    let _ = std::process::Command::new("kill")
        .args(["-TERM", &daemon_id.to_string()])
        .status();

    let _ = daemon.wait().await;
    assert!(!std::fs::exists(socket_path)?, "Socket file clean-up failed.");
    Ok(())
}
