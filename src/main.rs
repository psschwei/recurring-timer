use iced::widget::{button, column, container, progress_bar, row, text, text_input};
use iced::{Alignment, Element, Length, Subscription, Task};

mod audio;
mod timer;

fn main() -> iced::Result {
    iced::application("Recurring Timer", RecurringTimer::update, RecurringTimer::view)
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

    fn view(&self) -> Element<Message> {
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
            "Time Remaining: {:02}:{:02}",
            remaining_mins, remaining_secs_part
        ))
        .size(24);

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
        .size(20);

        let round_display = text(format!("Round: {}", self.round_number)).size(20);

        let progress = if self.total_duration_secs > 0 {
            self.elapsed_secs as f32 / self.total_duration_secs as f32
        } else {
            0.0
        };

        let progress_bar = progress_bar(0.0..=1.0, progress);

        let status_text = match self.timer_state {
            TimerState::Stopped => "Stopped",
            TimerState::Running => "Running",
            TimerState::Paused => "Paused",
        };
        let status_display = text(format!("Status: {}", status_text)).size(16);

        let content = column![
            text("Recurring Timer").size(32),
            inputs,
            control_buttons,
            status_display,
            time_display,
            round_time_display,
            progress_bar,
            round_display,
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
