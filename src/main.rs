use std::process::ExitCode;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn main() -> ExitCode {
    eprintln!("fubuki does not support the current platform");
    ExitCode::FAILURE
}

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
#[cfg(not(feature = "gui"))]
fn main() -> ExitCode {
    use fubukil::{Args, launch};
    use clap::Parser;

    human_panic::setup_panic!();

    match launch(Args::parse()) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{:?}", e);
            ExitCode::FAILURE
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
#[cfg(feature = "gui")]
fn main() -> ExitCode {
    use fubukil::{Args, launch};

    human_panic::setup_panic!();

    let mut settings = klask::Settings::default();

    if let Ok(path) = std::env::var("FUBUKI_GUI_FONT_PATH") {
        let font = match std::fs::read(path) {
            Ok(font) => font,
            Err(e) => {
                eprintln!("read font data error: {:?}", e);
                return ExitCode::FAILURE;
            }
        };
        settings.custom_font = Some(std::borrow::Cow::Owned(font));
    }

    klask::run_derived::<Args, _>(settings, |args| {
        if let Err(e) = launch(args) {
            eprintln!("{:?}", e);
        }
    });

    ExitCode::SUCCESS
}
