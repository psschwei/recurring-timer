use iced::widget::{button, column, container, progress_bar, row, text, text_input};
use iced::{Alignment, Color, Element, Length, Subscription, Task};

mod audio;
mod circular_progress;
mod timer;

fn main() -> iced::Result {
    iced::application("Round Timer", RecurringTimer::update, RecurringTimer::view)
        .subscription(RecurringTimer::subscription)
        .run_with(RecurringTimer::new)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TimerState {
    Stopped,
    Running,
    Paused,
}

struct RecurringTimer {
    interval_input: String,
    rounds_input: String,
    interval_secs: u32,
    num_rounds: u32,
    timer_state: TimerState,
    elapsed_secs: u32,
    total_duration_secs: u32,
    round_number: u32,
    audio_player: audio::AudioPlayer,
}

#[derive(Debug, Clone)]
enum Message {
    IntervalChanged(String),
    RoundsChanged(String),
    Start,
    Pause,
    Resume,
    Stop,
    Tick,
}

impl RecurringTimer {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                interval_input: String::from("60"),
                rounds_input: String::from("20"),
                interval_secs: 60,
                num_rounds: 20,
                timer_state: TimerState::Stopped,
                elapsed_secs: 0,
                total_duration_secs: 60 * 20,
                round_number: 1,
                audio_player: audio::AudioPlayer::new(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::IntervalChanged(value) => {
                self.interval_input = value.clone();
                if let Ok(secs) = value.parse::<u32>() {
                    if secs > 0 {
                        self.interval_secs = secs;
                        self.total_duration_secs = self.interval_secs * self.num_rounds;
                    }
                }
            }
            Message::RoundsChanged(value) => {
                self.rounds_input = value.clone();
                if let Ok(rounds) = value.parse::<u32>() {
                    if rounds > 0 {
                        self.num_rounds = rounds;
                        self.total_duration_secs = self.interval_secs * self.num_rounds;
                    }
                }
            }
            Message::Start => {
                self.timer_state = TimerState::Running;
                self.elapsed_secs = 0;
                self.round_number = 1;
                self.total_duration_secs = self.interval_secs * self.num_rounds;
            }
            Message::Pause => {
                self.timer_state = TimerState::Paused;
            }
            Message::Resume => {
                self.timer_state = TimerState::Running;
            }
            Message::Stop => {
                self.timer_state = TimerState::Stopped;
                self.elapsed_secs = 0;
                self.round_number = 1;
            }
            Message::Tick => {
                if self.timer_state == TimerState::Running {
                    self.elapsed_secs += 1;

                    // Check if it's time to play a chime
                    if self.elapsed_secs % self.interval_secs == 0 {
                        self.audio_player.play_chime();
                        // Only increment round number if we're not at the final chime
                        if self.elapsed_secs < self.total_duration_secs {
                            self.round_number += 1;
                        }
                    }

                    // Check if we've reached the total duration
                    if self.elapsed_secs >= self.total_duration_secs {
                        self.timer_state = TimerState::Stopped;
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let is_configurable = self.timer_state == TimerState::Stopped;

        let interval_input = text_input("Interval (seconds)", &self.interval_input)
            .on_input(Message::IntervalChanged)
            .padding(10);

        let rounds_input = text_input("Number of Rounds", &self.rounds_input)
            .on_input(Message::RoundsChanged)
            .padding(10);

        let inputs = if is_configurable {
            row![
                column![text("Interval (seconds)"), interval_input].spacing(5),
                column![text("Number of Rounds"), rounds_input].spacing(5),
            ]
            .spacing(20)
        } else {
            row![
                column![
                    text("Interval (seconds)"),
                    text(&self.interval_input).size(16)
                ]
                .spacing(5),
                column![
                    text("Number of Rounds"),
                    text(&self.rounds_input).size(16)
                ]
                .spacing(5),
            ]
            .spacing(20)
        };

        let control_buttons = match self.timer_state {
            TimerState::Stopped => row![button("Start").on_press(Message::Start)].spacing(10),
            TimerState::Running => {
                row![
                    button("Pause").on_press(Message::Pause),
                    button("Stop").on_press(Message::Stop)
                ]
                .spacing(10)
            }
            TimerState::Paused => {
                row![
                    button("Resume").on_press(Message::Resume),
                    button("Stop").on_press(Message::Stop)
                ]
                .spacing(10)
            }
        };

        let remaining_secs = self.total_duration_secs.saturating_sub(self.elapsed_secs);
        let remaining_mins = remaining_secs / 60;
        let remaining_secs_part = remaining_secs % 60;

        let time_display = text(format!(
            "Total Time Remaining: {:02}:{:02}",
            remaining_mins, remaining_secs_part
        ))
        .size(18);

        let round_remaining_secs = if self.interval_secs > 0 {
            let time_in_round = self.elapsed_secs % self.interval_secs;
            if time_in_round == 0 {
                self.interval_secs
            } else {
                self.interval_secs - time_in_round
            }
        } else {
            0
        };
        let round_remaining_mins = round_remaining_secs / 60;
        let round_remaining_secs_part = round_remaining_secs % 60;

        let round_time_display = text(format!(
            "Round Time Remaining: {:02}:{:02}",
            round_remaining_mins, round_remaining_secs_part
        ))
        .size(28);

        let round_display = text(format!("Round: {}", self.round_number)).size(20);

        let progress = if self.total_duration_secs > 0 {
            self.elapsed_secs as f32 / self.total_duration_secs as f32
        } else {
            0.0
        };

        let progress_bar = progress_bar(0.0..=1.0, progress);

        // Calculate round progress for circular indicator (shows remaining time)
        let round_progress = if self.interval_secs > 0 {
            let time_in_round = self.elapsed_secs % self.interval_secs;
            let remaining = if time_in_round == 0 {
                self.interval_secs
            } else {
                self.interval_secs - time_in_round
            };
            remaining as f32 / self.interval_secs as f32
        } else {
            0.0
        };

        let status_text = match self.timer_state {
            TimerState::Stopped => "Stopped",
            TimerState::Running => "Running",
            TimerState::Paused => "Paused",
        };
        let status_display = text(format!("Status: {}", status_text)).size(16);

        let content = column![
            text("Round Timer").size(32),
            inputs,
            control_buttons,
            status_display,
            round_time_display,
            circular_progress::circular_progress(round_progress, Color::from_rgb(0.2, 0.7, 0.9))
                .map(|_| Message::Tick),
            round_display,
            progress_bar,
            time_display,
        ]
        .spacing(20)
        .padding(20)
        .align_x(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.timer_state == TimerState::Running {
            timer::timer_subscription()
        } else {
            Subscription::none()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_timer() -> RecurringTimer {
        RecurringTimer {
            interval_input: String::from("60"),
            rounds_input: String::from("20"),
            interval_secs: 60,
            num_rounds: 20,
            timer_state: TimerState::Stopped,
            elapsed_secs: 0,
            total_duration_secs: 1200,
            round_number: 1,
            audio_player: audio::AudioPlayer::new(),
        }
    }

    #[test]
    fn test_initial_state() {
        let (timer, _) = RecurringTimer::new();
        assert_eq!(timer.interval_secs, 60);
        assert_eq!(timer.num_rounds, 20);
        assert_eq!(timer.timer_state, TimerState::Stopped);
        assert_eq!(timer.elapsed_secs, 0);
        assert_eq!(timer.round_number, 1);
        assert_eq!(timer.total_duration_secs, 1200);
    }

    #[test]
    fn test_start_message() {
        let mut timer = create_test_timer();
        timer.elapsed_secs = 100;
        timer.round_number = 5;

        let _ = timer.update(Message::Start);

        assert_eq!(timer.timer_state, TimerState::Running);
        assert_eq!(timer.elapsed_secs, 0);
        assert_eq!(timer.round_number, 1);
        assert_eq!(timer.total_duration_secs, 1200);
    }

    #[test]
    fn test_pause_message() {
        let mut timer = create_test_timer();
        timer.timer_state = TimerState::Running;
        timer.elapsed_secs = 30;

        let _ = timer.update(Message::Pause);

        assert_eq!(timer.timer_state, TimerState::Paused);
        assert_eq!(timer.elapsed_secs, 30); // Elapsed time should be preserved
    }

    #[test]
    fn test_resume_message() {
        let mut timer = create_test_timer();
        timer.timer_state = TimerState::Paused;
        timer.elapsed_secs = 30;

        let _ = timer.update(Message::Resume);

        assert_eq!(timer.timer_state, TimerState::Running);
        assert_eq!(timer.elapsed_secs, 30); // Elapsed time should be preserved
    }

    #[test]
    fn test_stop_message() {
        let mut timer = create_test_timer();
        timer.timer_state = TimerState::Running;
        timer.elapsed_secs = 100;
        timer.round_number = 5;

        let _ = timer.update(Message::Stop);

        assert_eq!(timer.timer_state, TimerState::Stopped);
        assert_eq!(timer.elapsed_secs, 0);
        assert_eq!(timer.round_number, 1);
    }

    #[test]
    fn test_tick_increments_elapsed_time() {
        let mut timer = create_test_timer();
        timer.timer_state = TimerState::Running;

        let _ = timer.update(Message::Tick);

        assert_eq!(timer.elapsed_secs, 1);
    }

    #[test]
    fn test_tick_does_not_increment_when_stopped() {
        let mut timer = create_test_timer();
        timer.timer_state = TimerState::Stopped;

        let _ = timer.update(Message::Tick);

        assert_eq!(timer.elapsed_secs, 0);
    }

    #[test]
    fn test_tick_does_not_increment_when_paused() {
        let mut timer = create_test_timer();
        timer.timer_state = TimerState::Paused;
        timer.elapsed_secs = 30;

        let _ = timer.update(Message::Tick);

        assert_eq!(timer.elapsed_secs, 30);
    }

    #[test]
    fn test_round_number_increments_at_interval() {
        let mut timer = create_test_timer();
        timer.interval_secs = 10;
        timer.num_rounds = 5;
        timer.total_duration_secs = 50;
        timer.timer_state = TimerState::Running;

        // Advance to 10 seconds (first chime)
        for _ in 0..10 {
            let _ = timer.update(Message::Tick);
        }

        assert_eq!(timer.elapsed_secs, 10);
        assert_eq!(timer.round_number, 2);
    }

    #[test]
    fn test_round_number_does_not_increment_at_final_chime() {
        let mut timer = create_test_timer();
        timer.interval_secs = 10;
        timer.num_rounds = 2;
        timer.total_duration_secs = 20;
        timer.timer_state = TimerState::Running;

        // Advance to 20 seconds (final chime)
        for _ in 0..20 {
            let _ = timer.update(Message::Tick);
        }

        assert_eq!(timer.elapsed_secs, 20);
        assert_eq!(timer.round_number, 2);
        assert_eq!(timer.timer_state, TimerState::Stopped);
    }

    #[test]
    fn test_timer_stops_at_total_duration() {
        let mut timer = create_test_timer();
        timer.interval_secs = 10;
        timer.num_rounds = 3;
        timer.total_duration_secs = 30;
        timer.timer_state = TimerState::Running;

        // Advance past total duration
        for _ in 0..31 {
            let _ = timer.update(Message::Tick);
        }

        assert_eq!(timer.timer_state, TimerState::Stopped);
        assert_eq!(timer.elapsed_secs, 30);
    }

    #[test]
    fn test_interval_changed_valid_input() {
        let mut timer = create_test_timer();
        timer.interval_secs = 60;
        timer.num_rounds = 20;

        let _ = timer.update(Message::IntervalChanged(String::from("90")));

        assert_eq!(timer.interval_input, "90");
        assert_eq!(timer.interval_secs, 90);
        assert_eq!(timer.total_duration_secs, 90 * 20);
    }

    #[test]
    fn test_interval_changed_invalid_input() {
        let mut timer = create_test_timer();
        let original_interval = timer.interval_secs;
        let original_duration = timer.total_duration_secs;

        let _ = timer.update(Message::IntervalChanged(String::from("abc")));

        assert_eq!(timer.interval_input, "abc");
        assert_eq!(timer.interval_secs, original_interval);
        assert_eq!(timer.total_duration_secs, original_duration);
    }

    #[test]
    fn test_interval_changed_zero_rejected() {
        let mut timer = create_test_timer();
        let original_interval = timer.interval_secs;
        let original_duration = timer.total_duration_secs;

        let _ = timer.update(Message::IntervalChanged(String::from("0")));

        assert_eq!(timer.interval_input, "0");
        assert_eq!(timer.interval_secs, original_interval);
        assert_eq!(timer.total_duration_secs, original_duration);
    }

    #[test]
    fn test_rounds_changed_valid_input() {
        let mut timer = create_test_timer();
        timer.interval_secs = 60;
        timer.num_rounds = 20;

        let _ = timer.update(Message::RoundsChanged(String::from("30")));

        assert_eq!(timer.rounds_input, "30");
        assert_eq!(timer.num_rounds, 30);
        assert_eq!(timer.total_duration_secs, 60 * 30);
    }

    #[test]
    fn test_rounds_changed_invalid_input() {
        let mut timer = create_test_timer();
        let original_rounds = timer.num_rounds;
        let original_duration = timer.total_duration_secs;

        let _ = timer.update(Message::RoundsChanged(String::from("xyz")));

        assert_eq!(timer.rounds_input, "xyz");
        assert_eq!(timer.num_rounds, original_rounds);
        assert_eq!(timer.total_duration_secs, original_duration);
    }

    #[test]
    fn test_rounds_changed_zero_rejected() {
        let mut timer = create_test_timer();
        let original_rounds = timer.num_rounds;
        let original_duration = timer.total_duration_secs;

        let _ = timer.update(Message::RoundsChanged(String::from("0")));

        assert_eq!(timer.rounds_input, "0");
        assert_eq!(timer.num_rounds, original_rounds);
        assert_eq!(timer.total_duration_secs, original_duration);
    }

    #[test]
    fn test_multiple_rounds() {
        let mut timer = create_test_timer();
        timer.interval_secs = 5;
        timer.num_rounds = 4;
        timer.total_duration_secs = 20;
        timer.timer_state = TimerState::Running;

        // Round 1: 0-5 seconds
        for _ in 0..5 {
            let _ = timer.update(Message::Tick);
        }
        assert_eq!(timer.round_number, 2);
        assert_eq!(timer.timer_state, TimerState::Running);

        // Round 2: 5-10 seconds
        for _ in 0..5 {
            let _ = timer.update(Message::Tick);
        }
        assert_eq!(timer.round_number, 3);
        assert_eq!(timer.timer_state, TimerState::Running);

        // Round 3: 10-15 seconds
        for _ in 0..5 {
            let _ = timer.update(Message::Tick);
        }
        assert_eq!(timer.round_number, 4);
        assert_eq!(timer.timer_state, TimerState::Running);

        // Round 4: 15-20 seconds (final)
        for _ in 0..5 {
            let _ = timer.update(Message::Tick);
        }
        assert_eq!(timer.round_number, 4);
        assert_eq!(timer.timer_state, TimerState::Stopped);
    }

    #[test]
    fn test_pause_resume_preserves_state() {
        let mut timer = create_test_timer();
        timer.interval_secs = 10;
        timer.num_rounds = 3;
        timer.total_duration_secs = 30;
        timer.timer_state = TimerState::Running;

        // Run for 7 seconds
        for _ in 0..7 {
            let _ = timer.update(Message::Tick);
        }
        assert_eq!(timer.elapsed_secs, 7);
        assert_eq!(timer.round_number, 1);

        // Pause
        let _ = timer.update(Message::Pause);
        assert_eq!(timer.timer_state, TimerState::Paused);

        // Tick while paused - should not change
        let _ = timer.update(Message::Tick);
        assert_eq!(timer.elapsed_secs, 7);

        // Resume
        let _ = timer.update(Message::Resume);
        assert_eq!(timer.timer_state, TimerState::Running);

        // Continue for 3 more seconds to reach first chime
        for _ in 0..3 {
            let _ = timer.update(Message::Tick);
        }
        assert_eq!(timer.elapsed_secs, 10);
        assert_eq!(timer.round_number, 2);
    }
}
