# **Labt (Lab Timer)**  
*A worldline-accurate CLI timer*  

> *"Time waits for no one... but this timer waits for you."*  
> — *Hououin Kyouma, probably*  

---

## **Installation**  

Prebuild binaries are available for x86_64 Linux and Windows in the [latest release](https://github.com/metdxt/labt/releases/latest).

Or you can build and install it yourself with:

```bash
cargo install --git https://github.com/metdxt/labt
```
*(Requires [Rust](https://rustup.rs/))*  

---

## **Usage**  
Set a timer with **hours**, **minutes**, or **seconds**:  
```bash
labt -M 30          # 30-minute timer
labt -H 1 -M 15     # 1 hour 15 minutes
labt -S 10          # 10-second countdown
```

### **Features**  
- 🔔 **"Tu-tu-ru~" alarm sound** (Mayuri-approved)  
- 🔕 Silent mode (`-s`) for stealthy lab experiments  
- 📝 Custom notifications (`-t "Title" -b "Message"`)  
- 🤫 Quiet mode (`-q`) for scripts  

### **Options**  
```
-H, --hours      Hours  
-M, --minutes    Minutes  
-S, --seconds    Seconds  
-t, --title      Notification title (default: "Timer Finished!")  
-b, --body       Custom notification body  
-n, --no-notify  Disable notifications  
-s, --no-sound   Disable alarm sound  
-q, --quiet      No output (for scripts)  
-N, --non-interactive  Print full output (no fancy \r)  
```

---

## **Examples**  
```bash
# Lab experiment countdown (with sound + notification)  
labt -M 90 -t "Experiment Complete" -b "Remove banana from microwave"  

# Silent mode for hacking SERN  
labt -H 2 -s -q  

# Worldline divergence measurement  
labt -S 42 -t "Divergence reached"  
```

## **Scripting Examples**

### **Pomodoro Time Loop**

Create a [`pomodoro`](examples/pomodoro) script for time management:

```bash
#!/bin/bash
# Default: 4 sessions of 25m work + 5m break
sessions=4
work_mins=25
break_mins=5

for ((i=1; i<=sessions; i++)); do
    echo "🚀 Work Session $i/$sessions | $(date +%H:%M)"
    labt -S $work_mins -t "Good job! Time to rest!" -b "Mayushii wants tea!"

    exit_code=$?
    if [[ $exit_code -eq 2 ]]; then # labt returns code 2 when timer is interrupted with ctrl+c by user
        exit 2
    fi
    
    echo "☕ Break Time! | $(date +%H:%M)"
    [[ $i -lt $sessions ]] && labt -S $break_mins -t "Back to experiments!" -b "Tutturu!~ Time to focus!"
done

echo "🎉 All sessions complete!"
```

---

## **Why "Labt"?**  
- Short for **Lab Timer** (used by the Future Gadget Lab).  
- Easy to type.  

---

*(Timer accuracy not guaranteed in alternate worldlines.)*  

---
