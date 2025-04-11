use clap::Parser;
use notify_rust::Notification;
use rodio::OutputStreamHandle;
use std::io::{stdout, Write};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io::Cursor;
use rodio::{ Decoder, OutputStream, Sink};

const ALARM_SOUND_DATA: &[u8] = include_bytes!("../assets/alarm.ogg");

fn format_duration(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn play_alarm(stream_handle: OutputStreamHandle) -> Result<(), Box<dyn std::error::Error>> {
    let cursor = Cursor::new(ALARM_SOUND_DATA);
    let source = Decoder::new(cursor)?;
    
    let sink = Sink::try_new(&stream_handle)?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}

/// Future Gadget #8: "Labt" - Worldline-accurate countdown device
#[derive(Parser, Debug)]
#[command(
    version = concat!(env!("CARGO_PKG_VERSION"), " (commit: ", env!("GIT_HASH"), ")"),
    about = concat!("⏳ Future Gadget Lab's official timer (", env!("CARGO_PKG_VERSION"), ")"),
    long_about = None,
    after_help = "Timings may vary across worldlines."
)]
struct CliArgs {
    /// Hours to wait before convergence
    #[arg(
        short = 'H',
        long,
        value_name = "HOURS",
        default_value_t = 0,
        help = "⌛ Hours until worldline shift (0-∞)"
    )]
    hours: u64,

    /// Minutes until alert
    #[arg(
        short = 'M',
        long,
        value_name = "MINUTES",
        default_value_t = 0,
        help = "⏰ Minutes until Mayuri's tuturu (0-59)"
    )]
    minutes: u64,

    /// Seconds until divergence
    #[arg(
        short = 'S',
        long,
        value_name = "SECONDS",
        default_value_t = 0,
        help = "⚡ Seconds until Organization intervention (0-59)"
    )]
    seconds: u64,

    /// Notification title (default: "Timer Finished!")
    #[arg(
        short = 't',
        long,
        value_name = "TITLE",
        default_value = "Timer Finished!",
        help = "📢 D-Mail subject line [default: 'Timer Finished!']"
    )]
    notification_title: String,

    /// Custom notification message
    #[arg(
        short = 'b',
        long,
        value_name = "BODY",
        help = "📝 Optional D-Mail body text"
    )]
    notification_body: Option<String>,

    /// Disable all notifications
    #[arg(
        short = 'n',
        long,
        help = "🔕 Stealth mode (no notifications)"
    )]
    disable_notifications: bool,

    /// Disable alarm sound
    #[arg(
        short = 's',
        long,
        help = "🔇 Silence Mayuri's tuturu"
    )]
    disable_sound: bool,

    /// Suppress all output
    #[arg(
        short = 'q',
        long,
        help = "🤐 Quiet mode (for SERN spies)"
    )]
    quiet: bool,

    /// Script-friendly output
    #[arg(
        short = 'N',
        long,
        help = "📟 Non-interactive (for lab experiments)"
    )]
    non_interactive: bool,
}

fn main() {
    let args = CliArgs::parse();
    let total_seconds = args.hours * 3600 + args.minutes * 60 + args.seconds;

    if total_seconds == 0 {
        eprintln!("Error: Total duration must be greater than 0 seconds.");
        process::exit(1);
    }

    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = interrupted.clone();
    ctrlc::set_handler(move || {
        interrupted_clone.store(true, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let input_duration_str = format!(
        "{}h {}m {}s",
        args.hours, args.minutes, args.seconds
    );

    // initialize audio stream early to avoid audio cold start silence
    let audio_stream = if !args.disable_sound {
        match OutputStream::try_default() {
            Ok((stream, handle)) => Some((stream, handle)),
            Err(e) => {
                if !args.quiet {
                    eprintln!("Failed to initialize audio: {}", e);
                }
                None
            }
        }
    } else {
        None
    };

    // Countdown
    for remaining in (1..=total_seconds).rev() {
        if interrupted.load(Ordering::SeqCst) {
            if !args.quiet {
                eprintln!("\nTimer interrupted!");
            }
            process::exit(2);
        }

        if !args.quiet {
            if args.non_interactive {
                println!("Time remaining: {}", format_duration(remaining));
            } else {
                print!("\rTime remaining: {} ", format_duration(remaining));
                stdout().flush().unwrap();
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    if !args.quiet {
        if args.non_interactive {
            println!("Time remaining: 00:00:00");
        } else {
            println!("\rTime remaining: 00:00:00");
        }
    }

    // Show notification if not disabled
    if !args.disable_notifications {
        let body = args.notification_body.unwrap_or_else(|| {
            format!(
                "The timer for {} ({} seconds) is complete.",
                input_duration_str, total_seconds
            )
        });

        if let Err(e) = Notification::new()
            .summary(&args.notification_title)
            .body(&body)
            .icon("alarm-symbolic")
            .show()
        {
            if !args.quiet {
                eprintln!("Failed to send notification: {}", e);
            }
        }
    }

    // Play sound if not disabled
    if let Some((_stream, stream_handle)) = audio_stream {
        if let Err(e) = play_alarm(stream_handle) {
            if !args.quiet {
                eprintln!("Failed to play sound: {}", e);
            }
        }
    }
}