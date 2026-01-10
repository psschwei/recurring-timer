# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Round Timer is a minimal desktop application built with Rust that plays a chime at regular intervals for a specified duration. It's a simple interval timer with pause/resume functionality and visual progress indicators.

**Tech Stack:**
- Rust (Edition 2021)
- iced 0.13 (GUI framework with tokio async runtime and canvas support)
- rodio 0.19 (audio playback)

## Development Workflow

**All changes must go through a pull request:**

1. Create a new branch for your changes:
   ```bash
   git checkout -b descriptive-branch-name
   ```

2. Make your changes and commit them:
   ```bash
   git add .
   git commit -m "description of changes"
   ```

3. Push the branch to GitHub:
   ```bash
   git push -u origin descriptive-branch-name
   ```

4. Create a pull request:
   ```bash
   gh pr create --title "PR Title" --body "Description"
   ```

5. Wait for CI checks to pass (tests, clippy, build)

6. Merge the PR after approval and successful CI

7. Update local main branch:
   ```bash
   git checkout main
   git pull origin main
   git branch -d descriptive-branch-name
   ```

**Never commit directly to main.** Always work on a branch and open a PR, even for small changes. This ensures CI validates all changes before they're merged.

## Development Commands

### Build and Run
```bash
# Development build
cargo build

# Development run
cargo run

# Release build
cargo build --release

# Release run
cargo run --release
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test --verbose

# Run a specific test
cargo test test_name

# Run tests for a specific module
cargo test tests::test_initial_state
```

### Linting
```bash
# Run clippy (use same flags as CI)
cargo clippy -- -D warnings -A unknown-lints -A clippy::manual_is_multiple_of
```

**Note on clippy flags:** The `-A clippy::manual_is_multiple_of` flag is required because clippy suggests using `is_multiple_of()` which is still unstable in Rust. The `-A unknown-lints` handles version differences between local and CI clippy.

### Linux Dependencies
On Linux, ALSA development libraries are required:
```bash
sudo apt-get update
sudo apt-get install -y libasound2-dev pkg-config
```

### Installation Scripts

**install.sh** - Installs Round Timer for desktop use:
- Detects prebuilt binary (from release) or builds from source
- Installs binary to `~/.local/bin/round-timer`
- Installs desktop entry to `~/.local/share/applications/round-timer.desktop`
- Installs icon to `~/.local/share/icons/hicolor/scalable/apps/round-timer.svg`
- Updates desktop database for application menu integration
- No sudo required (installs to user directories)

**uninstall.sh** - Removes Round Timer from the system:
- Removes binary, desktop entry, and icon
- Updates desktop database

**round-timer.desktop** - Desktop entry file for application menu integration:
- Defines application name, icon, and executable
- Categorized under Utilities
- Works with KDE, GNOME, and other desktop environments

**assets/round-timer.svg** - Application icon:
- SVG format for scalability
- Blue circular clock design with bell indicator
- 128x128 viewBox

**README-INSTALL.txt** - Installation instructions for GitHub release downloads:
- Included in Linux release tarballs
- Provides quick start instructions for end users
- Documents what gets installed and where
- No technical/developer information (targeted at end users)

## Architecture

### Application State Management (Elm Architecture)

The app follows the Elm architecture pattern via iced:
- **Model:** `RecurringTimer` struct holds all application state
- **Update:** `update()` method handles messages and modifies state
- **View:** `view()` method renders UI based on current state
- **Subscriptions:** `subscription()` method manages the 1-second timer tick

### State Flow

```
User Input/Timer Tick → Message → update() → New State → view() → UI Render
                                      ↓
                                 audio.play_chime()
```

**Key State:**
- `timer_state: TimerState` - Stopped/Running/Paused
- `elapsed_secs: u32` - Seconds elapsed since start
- `round_number: u32` - Current round/chime number
- `interval_secs: u32` - Seconds between chimes
- `num_rounds: u32` - Total number of rounds
- `total_duration_secs: u32` - Total timer duration (interval × rounds)

### Module Structure

**src/main.rs** (263 lines + 315 test lines)
- Main application logic and UI
- `RecurringTimer` struct and implementation
- All message handling (Start/Pause/Resume/Stop/Tick/Input changes)
- UI rendering with iced widgets
- Timer subscription management
- Comprehensive unit tests (19 tests covering state transitions, timer logic, input validation)

**src/timer.rs** (10 lines)
- Timer subscription using `iced::time::every(Duration::from_secs(1))`
- Emits `Message::Tick` every second when timer is running

**src/audio.rs** (38 lines)
- `AudioPlayer` struct manages audio output stream
- `play_chime()` plays embedded WAV file
- Gracefully handles missing audio devices (CI/headless environments) using `Option<OutputStream>`
- Chime audio embedded at compile time via `include_bytes!()`

**src/circular_progress.rs** (74 lines + 65 test lines)
- Custom iced canvas widget for circular progress indicator
- Shows remaining time in current round as a pie chart
- Progress clamped to 0.0-1.0 range
- Draws from 12 o'clock position clockwise
- 9 tests covering progress clamping and angle calculations

### Timer Logic Details

**Chime Timing:**
- Chimes play when `elapsed_secs % interval_secs == 0`
- First chime plays after ONE interval (not immediately)
- Round number increments AFTER each chime (except the final one)
- Timer auto-stops when `elapsed_secs >= total_duration_secs`

**Important Behaviors:**
1. Inputs only editable when `timer_state == Stopped`
2. Pause preserves `elapsed_secs` and `round_number`
3. Stop resets both to initial values (0 and 1)
4. Start always resets state regardless of previous values

**Progress Calculations:**
- Total progress: `elapsed_secs / total_duration_secs`
- Round progress (for circular indicator): Shows REMAINING time in current round
- Time display uses `saturating_sub()` to prevent underflow

### Audio Player Design

The `AudioPlayer` uses `Option` types to handle environments without audio hardware:
- If audio initialization fails (e.g., CI, headless servers), stores `None`
- `play_chime()` becomes a no-op when audio unavailable
- This allows tests to run in headless CI environments

### Test Structure

Tests use a helper function `create_test_timer()` to instantiate `RecurringTimer` with known values. All tests properly handle the returned `Task` from `update()` with `let _ = ...`.

**Test Categories:**
- State transitions (Start/Pause/Resume/Stop)
- Timer tick behavior in different states
- Round progression and chime timing
- Input validation (valid/invalid/zero values)
- Edge cases (final chime, pause/resume, timer completion)

## CI/CD Workflows

**CI Workflow** (runs on PRs and pushes to main):
- Runs all tests
- Runs clippy with strict warnings
- Builds the project

**Release Workflow** (runs on version tags `v*`):
- Builds for multiple platforms (Linux x64/ARM64, macOS x64/ARM64, Windows x64)
- Runs tests before building
- Linux builds are bundled as .tar.gz with installer scripts, desktop file, and icon
- macOS and Windows builds are standalone binaries
- Creates GitHub release with all artifacts

## Common Patterns

### Adding New Messages

1. Add variant to `Message` enum
2. Handle in `update()` match statement
3. Return `Task::none()` (or specific task if needed)
4. Add tests for new behavior

### Modifying Timer Logic

When changing timer behavior in `Message::Tick` handler:
- Update relevant tests in the `tests` module
- Ensure edge cases are covered (start/stop, final chime, etc.)
- Test both with audio available and unavailable

### UI Changes

The `view()` method has explicit lifetime `Element<'_, Message>` to satisfy clippy. Maintain this when making changes to avoid clippy errors.
