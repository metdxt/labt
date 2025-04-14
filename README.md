# Labt - Future Gadget #16: Worldline-Accurate Timer

<table>
  <tr>
  <td>
  
  *"Oh no, my watch has stopped... But I've just wound it... Don't tell me it's broken." - Mayushii*

  </td>
  <td>

  ![](/assets/logo.svg)

  </td>
  </tr>
</table>



## Description

Developed by the Future Gadget Lab, Labt (codenamed FG-016) is a worldline-accurate countdown timer with cross-dimensional notification support.

## Features

- ⏲️ **Accurate Countdown**: Every second counts
- 📢 **Desktop Notifications**: Native system alerts on completion
- 🔔 **Mayuri Alarm**: Default "tuturu" sound (disable with `-s`)
- 📟 **Script-Friendly**: Non-interactive mode for lab use
- 🔕 **Stealth Mode**: Silent operation for covert operations
- 🌐 **Cross-Worldline**: Tested across 0.348615% divergence
- 🦀 **Implemented in Rust**: So that your watch is never *broken*.

## Installation

### Prebuilt binaries

Prebuilt binaries are available for x86_64 Linux and Windows in [releases](https://github.com/metdxt/labt/releases).
Just download a fitting one and put it in some `PATH` directory.

*NOTE: don't forget to `chmod +x` on linux.*

### Using Cargo

```bash
cargo install --git https://github.com/metdxt/labt
```

Cargo will collect the source code and build it on your machine.
Build time is rather small, about 11 seconds on lab's hardware.


**System Requirements**:

- Linux: DBus (for notifications), ALSA (for sound)
- Windows: Windows 10+ (notifications require Action Center)

## Usage

### Basic Syntax

```bash
labt [OPTIONS] --hours <HOURS> --minutes <MINUTES> --seconds <SECONDS>
```

### Options

| Option | Description                          | Example                    |
|--------|--------------------------------------|----------------------------|
| `-H`   | Hours component                      | `-H 2`                     |
| `-M`   | Minutes component                    | `-M 30`                    |
| `-S`   | Seconds component                    | `-S 45`                    |
| `-t`   | Notification title                   | `-t "Experiment Complete"` |
| `-b`   | Custom notification body             | `-b "Divergence reached"`  |
| `-n`   | Disable notifications                | `-n`                       |
| `-s`   | Disable alarm sound                  | `-s`                       |
| `-q`   | Quiet mode (no output)               | `-q`                       |
| `-N`   | Non-interactive (script-friendly)    | `-N`                       |

### Examples

1. **Standard 25-minute timer:**
   ```bash
   labt -M 25
   ```

2. **1h30m experiment with custom message:**
   ```bash
   labt -H 1 -M 30 -t "Phase 2 Complete" -b "Prepare for convergence"
   ```

3. **Silent 10-second countdown:**
   ```bash
   labt -S 10 -q -n -s
   ```

4. **Script-friendly mode:**
   ```bash
   labt -M 5 -N > timer.log
   ```

## Notification System

Labt uses your system's native notification system with these defaults:

- **Title**: "Timer Finished!"
- **Body**: Auto-generated based on input duration
- **Icon**: `alarm-symbolic` (system default fallback)

Customize with `-t`/`--notification-title` and `-b`/`--notification-body`.

## Alarm Sound

Default alarm sound features Mayuri's iconic "tuturu" (⚠️ Requires audio output).  
Disable with `-s` or use `-n` for complete silence.

## Exit Codes

| Code | Meaning                      |
|------|------------------------------|
| 0    | Timer completed successfully |
| 1    | Invalid duration specified   |
| 2    | Interrupted (Ctrl+C)         |

## Contributing

We welcome contributions across all worldlines!  
Please follow these guidelines:

1. Fork the repository
2. Create your feature branch (`git checkout -b worldline-42`)
3. Commit changes (`git commit -am 'Added δ worldline support'`)
4. Push to branch (`git push origin worldline-42`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for details.

## Acknowledgments

- Future Gadget Lab members
- clap, notify-rust, and rodio crate maintainers
