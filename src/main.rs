use clap::Parser;
use indicatif::ProgressBar;
use notify_rust::Notification;
use rodio::{OutputStreamHandle, PlayError, StreamError};
use std::io::{stdout, Write};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io::Cursor;
use rodio::{ Decoder, OutputStream, Sink};

fn format_duration(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

/// Future Gadget #16: "Labt" - Worldline-accurate countdown device
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
        help = "⌛ Hours"
    )]
    hours: u64,

    /// Minutes until alert
    #[arg(
        short = 'M',
        long,
        value_name = "MINUTES",
        default_value_t = 0,
        help = "⏰ Minutes"
    )]
    minutes: u64,

    /// Seconds until divergence
    #[arg(
        short = 'S',
        long,
        value_name = "SECONDS",
        default_value_t = 0,
        help = "⚡ Seconds"
    )]
    seconds: u64,

    /// Notification title
    #[arg(
        short = 't',
        long,
        value_name = "TITLE",
        default_value = "Timer Finished!",
        help = "📢 D-Mail subject line"
    )]
    notification_title: String,

    /// Custom notification message
    #[arg(
        short = 'b',
        long,
        value_name = "BODY",
        help = "📝 D-Mail body text"
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
        help = "🤐 Quiet mode (no CLI output)"
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


struct AudioPlayer {
    audio_stream: (OutputStream, OutputStreamHandle)
}

impl AudioPlayer {
    const ALARM_SOUND_DATA: &[u8] = include_bytes!("../assets/alarm.ogg");

    fn new() -> Result<Self, StreamError> {
        Ok(Self { audio_stream: OutputStream::try_default()? })
    }

    fn play_alarm(&self) -> Result<(), PlayError> {
        let cursor = Cursor::new(AudioPlayer::ALARM_SOUND_DATA);
        let decoder = Decoder::new(cursor)?;
        let sink = Sink::try_new(&self.audio_stream.1)?;
        sink.append(decoder);
        sink.sleep_until_end();
        Ok(())
    }
}

fn ep(quiet: bool, msg: &str) {
    if !quiet { eprintln!("{msg}") }
}

fn main() {
    let args = CliArgs::parse();
    let total_seconds = args.hours * 3600 + args.minutes * 60 + args.seconds;

    if total_seconds == 0 {
        ep(args.quiet, "Error: Total duration must be greater than 0 seconds.");
        ep(args.quiet, "Specify desired time with -H/--hours, -M/--minutes and/or -S/--seconds arguments.");
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

    let audio_player = if !args.disable_sound {
        let player = AudioPlayer::new();
        match player {
            Ok(player) => {
                Some(player)
            },
            Err(e) => {
                ep(args.quiet, &format!("Failed to create audio player: {e}"));
                None
            }
        }
    } else { None };

    let pb = if !args.non_interactive && !args.quiet {
        let pb = ProgressBar::new(total_seconds);
        pb.set_style(
            indicatif::ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:25.red/blue}] [{eta_precise}]")
                .unwrap()
                .progress_chars("~>-")
        );
        Some(pb)
    } else {
        None
    };

    // Countdown
    for remaining in (1..=total_seconds).rev() {
        if interrupted.load(Ordering::SeqCst) {
            ep(args.quiet, "\nTimer interrupted!");
            process::exit(2);
        }

        if let Some(ref pb) = pb {
            pb.set_position(total_seconds - remaining);
        } else if !args.quiet {
            if args.non_interactive {
                println!("Time remaining: {}", format_duration(remaining));
            } else {
                print!("\rTime remaining: {} ", format_duration(remaining));
                stdout().flush().unwrap();
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    if let Some(pb) = pb {
        pb.finish_with_message("Timer complete!");
    } else if !args.quiet {
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
            ep(args.quiet, &format!("Failed to send notification: {}", e));
        }
    }

    // Play sound if not disabled
    if let Some(player) = audio_player {
        if let Err(e) = player.play_alarm() {
            ep(args.quiet, &format!("{e}"));
        }
    }
}
