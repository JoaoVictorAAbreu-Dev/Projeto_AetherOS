use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const LIMINE_DOWNLOAD_URL: &str =
    "https://github.com/limine-bootloader/limine/releases/latest/download/limine-binary.zip";
const DIST_DIR: &str = "dist";
const LIMINE_CACHE_DIR: &str = "limine";
const ESP_DIR: &str = "esp";
const EFI_BOOT_DIR: &str = "EFI/BOOT";
const LIMINE_CONF_NAME: &str = "limine.conf";
const LIMINE_BOOTX64_NAME: &str = "BOOTX64.EFI";
const LIMINE_VARS_NAME: &str = "edk2-x86_64-vars.fd";

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
    println!("  stage    Build the kernel and stage a bootable UEFI tree");
    println!("  run      Stage assets and launch QEMU through Limine");
    println!("  test     Run formatting, host-safe tests, and kernel checks");
}

fn build_kernel() -> Result<(), String> {
    let mut command = cargo_command();
    command
        .arg("build")
        .arg("--package")
        .arg("aether-kernel")
        .arg("--target")
        .arg("config/target/x86_64-aetheros.json")
        .arg("-Zbuild-std=core,alloc,compiler_builtins")
        .arg("-Zbuild-std-features=compiler-builtins-mem")
        .arg("-Zjson-target-spec")
        .current_dir(workspace_root());

    run_command(&mut command, "cargo build")
}

fn stage_boot_tree() -> Result<(), String> {
    build_kernel()?;

    let root = workspace_root();
    let limine_dir = ensure_limine_bundle(&root)?;
    let stage_dir = root.join(DIST_DIR).join(ESP_DIR);
    let efi_boot_dir = stage_dir.join(EFI_BOOT_DIR);
    let kernel_source = locate_kernel_binary(&root)?;
    let kernel_dest = stage_dir.join("kernel.elf");
    let limine_cfg_src = locate_limine_config(&root)?;
    let limine_cfg_dest = stage_dir.join(LIMINE_CONF_NAME);
    let limine_cfg_efi_dest = efi_boot_dir.join(LIMINE_CONF_NAME);
    let limine_bootx64_src = limine_dir.join(LIMINE_BOOTX64_NAME);
    let limine_bootx64_dest = efi_boot_dir.join(LIMINE_BOOTX64_NAME);

    recreate_directory(&stage_dir)
        .map_err(|err| format!("failed to create staging tree: {err}"))?;
    fs::create_dir_all(&efi_boot_dir)
        .map_err(|err| format!("failed to create EFI boot directory: {err}"))?;
    fs::copy(&kernel_source, &kernel_dest).map_err(|err| {
        format!(
            "failed to copy kernel binary from {:?}: {err}",
            kernel_source
        )
    })?;
    fs::copy(&limine_cfg_src, &limine_cfg_dest)
        .map_err(|err| format!("failed to copy Limine configuration: {err}"))?;
    fs::copy(&limine_cfg_src, &limine_cfg_efi_dest)
        .map_err(|err| format!("failed to copy EFI-local Limine configuration: {err}"))?;
    fs::copy(&limine_bootx64_src, &limine_bootx64_dest)
        .map_err(|err| format!("failed to copy BOOTX64.EFI: {err}"))?;

    println!("staged kernel: {}", kernel_dest.display());
    println!("staged config: {}", limine_cfg_dest.display());
    println!("staged EFI loader: {}", limine_bootx64_dest.display());
    Ok(())
}

fn run_qemu(debug: bool) -> Result<(), String> {
    stage_boot_tree()?;

    let root = workspace_root();
    let qemu =
        find_qemu_binary().ok_or_else(|| "qemu-system-x86_64 executable not found".to_string())?;
    let ovmf_code = find_ovmf_code().ok_or_else(|| {
        "UEFI firmware not found. Install QEMU with edk2/OVMF firmware support.".to_string()
    })?;
    let ovmf_vars = ensure_ovmf_vars(&root)?;
    let stage_dir = root.join(DIST_DIR).join(ESP_DIR);

    let mut command = Command::new(qemu);
    command
        .current_dir(&root)
        .arg("-machine")
        .arg("q35")
        .arg("-m")
        .arg("256M")
        .arg("-serial")
        .arg(qemu_serial_mode())
        .arg("-drive")
        .arg(format!(
            "if=pflash,format=raw,readonly=on,file={}",
            ovmf_code.display()
        ))
        .arg("-drive")
        .arg(format!("if=pflash,format=raw,file={}", ovmf_vars.display()))
        .arg("-drive")
        .arg(format!("format=raw,file=fat:rw:{}", stage_dir.display()))
        .arg("-display")
        .arg(qemu_display_mode());

    if debug {
        command.arg("-s").arg("-S");
    }

    run_command(&mut command, "qemu-system-x86_64")
}

fn run_workspace_tests() -> Result<(), String> {
    run_command(
        cargo_command()
            .arg("fmt")
            .arg("--all")
            .arg("--")
            .arg("--check")
            .current_dir(workspace_root()),
        "cargo fmt",
    )?;
    run_command(
        cargo_command()
            .arg("check")
            .arg("--workspace")
            .current_dir(workspace_root()),
        "cargo check",
    )?;
    run_command(
        cargo_command()
            .arg("test")
            .arg("--package")
            .arg("aether-bootinfo")
            .arg("--package")
            .arg("xtask")
            .current_dir(workspace_root()),
        "cargo test (host-safe packages)",
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
        .ok_or_else(|| {
            format!(
                "could not find built kernel binary in {}",
                target_dir.display()
            )
        })
}

fn locate_limine_config(root: &Path) -> Result<PathBuf, String> {
    let candidates = [
        root.join("config").join("boot").join("limine.conf"),
        root.join("config").join("boot").join("limine.cfg"),
    ];

    candidates
        .into_iter()
        .find(|path| path.exists())
        .ok_or_else(|| "could not find Limine configuration in config/boot".to_string())
}

fn ensure_limine_bundle(root: &Path) -> Result<PathBuf, String> {
    let limine_dir = root
        .join(DIST_DIR)
        .join(LIMINE_CACHE_DIR)
        .join("limine-binary");
    let bootx64 = limine_dir.join(LIMINE_BOOTX64_NAME);

    if bootx64.exists() {
        return Ok(limine_dir);
    }

    download_and_extract_limine(root)?;

    if bootx64.exists() {
        Ok(limine_dir)
    } else {
        Err(format!(
            "Limine bundle was downloaded but {} is missing",
            bootx64.display()
        ))
    }
}

fn download_and_extract_limine(root: &Path) -> Result<(), String> {
    let dist_dir = root.join(DIST_DIR);
    let cache_dir = dist_dir.join(LIMINE_CACHE_DIR);
    let archive_path = cache_dir.join("limine-binary.zip");

    fs::create_dir_all(&cache_dir)
        .map_err(|err| format!("failed to create Limine cache directory: {err}"))?;

    download_file(LIMINE_DOWNLOAD_URL, &archive_path)?;
    extract_zip(&archive_path, &cache_dir)
}

fn download_file(url: &str, destination: &Path) -> Result<(), String> {
    if cfg!(windows) {
        let script = format!(
            "$ProgressPreference='SilentlyContinue'; Invoke-WebRequest -Uri '{}' -OutFile '{}'",
            url,
            destination.display()
        );

        return run_command(
            Command::new("powershell")
                .arg("-NoProfile")
                .arg("-Command")
                .arg(script),
            "download Limine bundle",
        );
    }

    if let Some(curl) = find_command("curl") {
        return run_command(
            Command::new(curl)
                .arg("-L")
                .arg(url)
                .arg("-o")
                .arg(destination),
            "download Limine bundle",
        );
    }

    Err("failed to download Limine bundle: neither PowerShell nor curl is available".to_string())
}

fn extract_zip(archive_path: &Path, destination: &Path) -> Result<(), String> {
    let extracted_root = destination.join("limine-binary");
    if extracted_root.exists() {
        fs::remove_dir_all(&extracted_root)
            .map_err(|err| format!("failed to clear Limine cache: {err}"))?;
    }

    if cfg!(windows) {
        let script = format!(
            "Add-Type -AssemblyName System.IO.Compression.FileSystem; \
             [System.IO.Compression.ZipFile]::ExtractToDirectory('{}', '{}')",
            archive_path.display(),
            destination.display()
        );

        return run_command(
            Command::new("powershell")
                .arg("-NoProfile")
                .arg("-Command")
                .arg(script),
            "extract Limine bundle",
        );
    }

    if let Some(unzip) = find_command("unzip") {
        return run_command(
            Command::new(unzip)
                .arg("-o")
                .arg(archive_path)
                .arg("-d")
                .arg(destination),
            "extract Limine bundle",
        );
    }

    Err("failed to extract Limine bundle: unzip support is unavailable".to_string())
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
            .flat_map(|path| command_candidates(&path, name))
            .find(|candidate| candidate.exists())
            .map(|candidate| candidate.to_string_lossy().into_owned())
    })
}

fn command_candidates(base: &Path, name: &str) -> Vec<PathBuf> {
    let mut candidates = vec![base.join(name)];
    if cfg!(windows) {
        candidates.push(base.join(format!("{name}.exe")));
        candidates.push(base.join(format!("{name}.cmd")));
        candidates.push(base.join(format!("{name}.bat")));
    }
    candidates
}

fn find_qemu_binary() -> Option<String> {
    find_command("qemu-system-x86_64").or_else(|| {
        let fallback = PathBuf::from(r"C:\Program Files\qemu\qemu-system-x86_64.exe");
        fallback
            .exists()
            .then(|| fallback.to_string_lossy().into_owned())
    })
}

fn find_ovmf_code() -> Option<PathBuf> {
    let candidates = [
        PathBuf::from(r"C:\Program Files\qemu\share\edk2-x86_64-code.fd"),
        PathBuf::from("/usr/share/OVMF/OVMF_CODE.fd"),
        PathBuf::from("/usr/share/edk2/x64/OVMF_CODE.fd"),
        PathBuf::from("/opt/homebrew/share/qemu/edk2-x86_64-code.fd"),
    ];

    candidates.into_iter().find(|path| path.exists())
}

fn ensure_ovmf_vars(root: &Path) -> Result<PathBuf, String> {
    let dist_dir = root.join(DIST_DIR);
    let ovmf_vars_dest = dist_dir.join(LIMINE_VARS_NAME);

    if ovmf_vars_dest.exists() {
        return Ok(ovmf_vars_dest);
    }

    fs::create_dir_all(&dist_dir)
        .map_err(|err| format!("failed to create dist directory: {err}"))?;

    let source = find_ovmf_vars_source().ok_or_else(|| {
        "UEFI variable store template not found. Install QEMU with edk2 variable images."
            .to_string()
    })?;

    fs::copy(&source, &ovmf_vars_dest).map_err(|err| {
        format!(
            "failed to copy OVMF vars template from {}: {err}",
            source.display()
        )
    })?;

    Ok(ovmf_vars_dest)
}

fn find_ovmf_vars_source() -> Option<PathBuf> {
    let candidates = [
        PathBuf::from(r"C:\Program Files\qemu\share\edk2-i386-vars.fd"),
        PathBuf::from("/usr/share/OVMF/OVMF_VARS.fd"),
        PathBuf::from("/usr/share/edk2/x64/OVMF_VARS.fd"),
    ];

    candidates.into_iter().find(|path| path.exists())
}

fn qemu_display_mode() -> String {
    env::var("AETHER_QEMU_DISPLAY").unwrap_or_else(|_| "default".to_string())
}

fn qemu_serial_mode() -> String {
    env::var("AETHER_QEMU_SERIAL").unwrap_or_else(|_| "stdio".to_string())
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("xtask workspace root")
        .to_path_buf()
}

fn cargo_command() -> Command {
    let mut command = Command::new("cargo");
    if let Some(toolchain) = preferred_rustup_toolchain() {
        command.arg(format!("+{toolchain}"));
    }
    command
}

fn preferred_rustup_toolchain() -> Option<String> {
    if let Ok(toolchain) = env::var("AETHER_RUSTUP_TOOLCHAIN") {
        return if toolchain.trim().is_empty() {
            None
        } else {
            Some(toolchain)
        };
    }

    if cfg!(windows) {
        Some("nightly-x86_64-pc-windows-gnu".to_string())
    } else {
        Some("nightly".to_string())
    }
}
