use anyhow::Context;
use async_process::{Command, Stdio};
use futures_lite::{io::BufReader, prelude::*};
use std::ffi::OsStr;

pub async fn build(cargo: bool, dioxus: bool, release: bool) -> anyhow::Result<()> {
    let args = if release {
        vec!["build", "--release"]
    } else {
        vec!["build"]
    };
    if cargo {
        run(Command::new("cargo"), &args)
            .await
            .context("Building the project with Cargo failed!")?;
    }
    if dioxus {
        run(Command::new("dx"), &args)
            .await
            .context("Building the project with dx failed!")?;
    }
    Ok(())
}

pub async fn build_tailwind(install: bool, watch: bool) -> anyhow::Result<()> {
    if install {
        run(
            Command::new("pnpm"),
            ["add", "-D", "tailwindcss", "@tailwindcss/cli"],
        )
            .await?;
    }
    let mut args = vec![
        "dlx",
        "@tailwindcss/cli",
        "-i",
        "global.css",
        "-o",
        "assets/main.css",
    ];
    if watch {
        args.push("--watch")
    }
    run(Command::new("pnpm"), args).await?;
    Ok(())
}

pub async fn run_dx_serve() -> anyhow::Result<()> {
    run(Command::new("dx"), ["serve"])
        .await
        .context("Failed to start the server with dx!")?;
    Ok(())
}

pub async fn clean() -> anyhow::Result<()> {
    run(Command::new("rm"), ["-rf", "node_modules"]).await?;
    run(Command::new("dx"), ["clean"]).await?;
    Ok(())
}

pub async fn fmt() -> anyhow::Result<()> {
    run(Command::new("dx"), ["fmt"]).await?;
    Ok(())
}

pub async fn run<I, S>(mut cmd: Command, args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut child = cmd
        .args(args)
        .kill_on_drop(true)
        .stdout(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    // 查看正常输出
    if let Some(stdout) = child.stdout.take() {
        println_lines(stdout).await?;
    } else if let Some(stderr) = child.stderr.take() {
        println_lines(stderr).await?;
    }

    Ok(())
}

async fn println_lines<T>(inner: T) -> anyhow::Result<()>
where
    T: AsyncRead + Unpin,
{
    let mut lines = BufReader::new(inner).lines();
    while let Some(line) = lines.next().await {
        println!("{}", line?);
    }
    Ok(())
}
