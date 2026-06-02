pub async fn run_training_stage(stage: &str, config_path: &str) -> anyhow::Result<()> {
    let output = tokio::process::Command::new("python")
        .args([
            "-m",
            &format!("bonsai_trainer.{}", stage),
            "--config",
            config_path,
        ])
        .output()
        .await?;

    if output.status.success() {
        Ok(())
    } else {
        anyhow::bail!(
            "Training failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    }
}
