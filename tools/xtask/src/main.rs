use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("xtask error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "help".to_string());

    match command.as_str() {
        "help" => {
            print_help();
            Ok(())
        }
        "build" => build_kernel(),
        "stage" => stage_boot_tree(),
        "run" => run_qemu(args.any(|arg| arg == "--debug")),
        "test" => run_workspace_tests(),
        other => Err(format!("unknown xtask command: {other}")),
    }
}

fn print_help() {
    println!("AetherOS xtask");
    println!("commands:");
    println!("  build    Build the kernel target");
    println!("  stage    Build the kernel and stage Limine boot assets");
    println!("  run      Stage assets and attempt to launch QEMU");
    println!("  test     Run workspace formatting, check, and tests");
}

fn build_kernel() -> Result<(), String> {
    run_command(
        Command::new("cargo")
            .arg("build")
            .arg("--package")
            .arg("aether-kernel")
            .current_dir(workspace_root()),
        "cargo build",
    )
}

fn stage_boot_tree() -> Result<(), String> {
    build_kernel()?;

    let root = workspace_root();
    let stage_dir = root.join("dist").join("iso_root");
    let boot_dir = stage_dir.join("boot");
    let kernel_source = locate_kernel_binary(&root)?;
    let kernel_dest = boot_dir.join("kernel.elf");
    let limine_cfg_src = root.join("config").join("boot").join("limine.cfg");
    let limine_cfg_dest = boot_dir.join("limine.cfg");

    recreate_directory(&boot_dir).map_err(|err| format!("failed to create staging tree: {err}"))?;
    fs::copy(&kernel_source, &kernel_dest)
        .map_err(|err| format!("failed to copy kernel binary from {:?}: {err}", kernel_source))?;
    fs::copy(&limine_cfg_src, &limine_cfg_dest)
        .map_err(|err| format!("failed to copy limine.cfg: {err}"))?;

    println!("staged kernel: {}", kernel_dest.display());
    println!("staged config: {}", limine_cfg_dest.display());
    println!("note: Limine bootloader binaries and ISO assembly still depend on local tooling availability.");
    Ok(())
}

fn run_qemu(debug: bool) -> Result<(), String> {
    stage_boot_tree()?;

    let qemu = find_command("qemu-system-x86_64")
        .ok_or_else(|| "qemu-system-x86_64 not found in PATH".to_string())?;
    let staged_kernel = workspace_root()
        .join("dist")
        .join("iso_root")
        .join("boot")
        .join("kernel.elf");

    if !staged_kernel.exists() {
        return Err("staged kernel not found after staging step".to_string());
    }

    let mut command = Command::new(qemu);
    command
        .current_dir(workspace_root())
        .arg("-serial")
        .arg("stdio")
        .arg("-display")
        .arg("default")
        .arg("-kernel")
        .arg(staged_kernel);

    if debug {
        command.arg("-s").arg("-S");
    }

    run_command(&mut command, "qemu-system-x86_64")
}

fn run_workspace_tests() -> Result<(), String> {
    run_command(
        Command::new("cargo")
            .arg("fmt")
            .arg("--all")
            .arg("--")
            .arg("--check")
            .current_dir(workspace_root()),
        "cargo fmt",
    )?;
    run_command(
        Command::new("cargo")
            .arg("check")
            .arg("--workspace")
            .current_dir(workspace_root()),
        "cargo check",
    )?;
    run_command(
        Command::new("cargo")
            .arg("test")
            .arg("--workspace")
            .current_dir(workspace_root()),
        "cargo test",
    )
}

fn run_command(command: &mut Command, label: &str) -> Result<(), String> {
    let status = command
        .status()
        .map_err(|err| format!("failed to start {label}: {err}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{label} exited with status {status}"))
    }
}

fn locate_kernel_binary(root: &Path) -> Result<PathBuf, String> {
    let target_dir = root.join("target").join("x86_64-aetheros").join("debug");
    let candidates = [
        target_dir.join("aether-kernel"),
        target_dir.join("aether-kernel.exe"),
        target_dir.join("aether-kernel.bin"),
    ];

    candidates
        .into_iter()
        .find(|path| path.exists())
        .ok_or_else(|| format!("could not find built kernel binary in {}", target_dir.display()))
}

fn recreate_directory(path: &Path) -> io::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }

    fs::create_dir_all(path)
}

fn find_command(name: &str) -> Option<String> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .map(|path| path.join(name))
            .find(|candidate| candidate.exists())
            .map(|candidate| candidate.to_string_lossy().into_owned())
    })
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("xtask workspace root")
        .to_path_buf()
}
