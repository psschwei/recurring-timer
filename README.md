# Round Timer

A minimal desktop application for Linux that plays a chime at regular intervals for a specified duration.

## Features

- Set custom interval between chimes (in seconds)
- Set total duration for the timer (in minutes)
- Pause/Resume functionality
- Visual progress bar showing completion
- Chime counter to track how many times the chime has played
- Clean, minimal user interface

## Installation

### Linux Desktop Installation

#### Option 1: Download from GitHub Releases (Recommended)

Download the latest release for your architecture from the [Releases page](https://github.com/yourusername/round-timer/releases):

```bash
# Download and extract (replace URL with latest release)
wget https://github.com/yourusername/round-timer/releases/download/v0.1.0/round-timer-linux-x86_64.tar.gz
tar -xzf round-timer-linux-x86_64.tar.gz
cd round-timer

# Run the installer
./install.sh
```

#### Option 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/round-timer.git
cd round-timer

# Run the installer
./install.sh
```

Both methods will:
- Build/install the release binary to `~/.local/bin/round-timer`
- Add a desktop entry so it appears in your Applications menu
- Install the application icon

After installation, you can launch Round Timer from your Applications menu or run `round-timer` from the terminal.

### Uninstall

To remove Round Timer from your system:

```bash
./uninstall.sh
```

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

Or run the compiled binary:

```bash
./target/release/round-timer
```

## Usage

1. **Set Interval**: Enter the number of seconds between each chime (default: 60)
2. **Set Duration**: Enter the total duration in minutes (default: 20)
3. **Start**: Click the Start button to begin the timer
4. **Pause/Resume**: While running, you can pause and resume the timer
5. **Stop**: Stop the timer at any time and reset to the beginning

### Example Use Case

To set a timer that chimes every 60 seconds for 20 minutes:
- Interval: `60`
- Duration: `20`
- Click **Start**

The application will play a chime every 60 seconds until 20 minutes have elapsed.

## UI Components

- **Interval/Duration inputs**: Only editable when timer is stopped
- **Status indicator**: Shows whether the timer is Stopped, Running, or Paused
- **Time Remaining**: Displays time left in MM:SS format
- **Progress Bar**: Visual representation of completion percentage
- **Chimes counter**: Shows how many times the chime has played in the current session

## Technical Details

Built with:
- **Rust** - Programming language
- **iced** - GUI framework
- **rodio** - Audio playback
- **tokio** - Async runtime for timer precision

## File Structure

```
round-timer/
├── Cargo.toml           # Project dependencies
├── README.md            # This file
├── src/
│   ├── main.rs         # Main application and UI
│   ├── timer.rs        # Timer subscription logic
│   └── audio.rs        # Audio playback handler
└── assets/
    └── chime.wav       # Chime sound file
```

## Notes

- The chime sound is embedded in the binary, so the application is self-contained
- The first chime plays after one interval has elapsed (not immediately)
- Input validation ensures only positive numbers are accepted
- The timer automatically stops when the duration is reached

