use iced::widget::canvas::{self, Canvas, Geometry, Path, Stroke};
use iced::{mouse, Color, Element, Length, Radians, Rectangle, Renderer, Theme};
use std::f32::consts::PI;

pub struct CircularProgress {
    progress: f32,
    color: Color,
}

impl CircularProgress {
    pub fn new(progress: f32, color: Color) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            color,
        }
    }
}

impl canvas::Program<()> for CircularProgress {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let center = frame.center();
        let radius = frame.width().min(frame.height()) / 2.0 - 10.0;

        // Draw background circle outline
        let background_circle = Path::circle(center, radius);
        frame.stroke(
            &background_circle,
            Stroke::default()
                .with_width(2.0)
                .with_color(Color::from_rgb(0.3, 0.3, 0.3)),
        );

        // Draw filled pie
        // Start from top (12 o'clock position) and go clockwise
        let start_angle = -PI / 2.0;
        let end_angle = start_angle + (2.0 * PI * self.progress);

        if self.progress > 0.0 {
            let pie = Path::new(|builder| {
                builder.move_to(center);
                builder.arc(canvas::path::Arc {
                    center,
                    radius,
                    start_angle: Radians(start_angle),
                    end_angle: Radians(end_angle),
                });
                builder.line_to(center);
                builder.close();
            });

            frame.fill(&pie, self.color);
        }

        vec![frame.into_geometry()]
    }
}

pub fn circular_progress(progress: f32, color: Color) -> Element<'static, ()> {
    Canvas::new(CircularProgress::new(progress, color))
        .width(Length::Fixed(150.0))
        .height(Length::Fixed(150.0))
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_clamps_progress_to_zero() {
        let progress = CircularProgress::new(-0.5, Color::from_rgb(1.0, 0.0, 0.0));
        assert_eq!(progress.progress, 0.0);
    }

    #[test]
    fn test_new_clamps_progress_to_one() {
        let progress = CircularProgress::new(1.5, Color::from_rgb(1.0, 0.0, 0.0));
        assert_eq!(progress.progress, 1.0);
    }

    #[test]
    fn test_new_accepts_valid_progress() {
        let progress = CircularProgress::new(0.5, Color::from_rgb(1.0, 0.0, 0.0));
        assert_eq!(progress.progress, 0.5);
    }

    #[test]
    fn test_new_accepts_zero() {
        let progress = CircularProgress::new(0.0, Color::from_rgb(1.0, 0.0, 0.0));
        assert_eq!(progress.progress, 0.0);
    }

    #[test]
    fn test_new_accepts_one() {
        let progress = CircularProgress::new(1.0, Color::from_rgb(1.0, 0.0, 0.0));
        assert_eq!(progress.progress, 1.0);
    }

    #[test]
    fn test_new_stores_color() {
        let color = Color::from_rgb(0.5, 0.3, 0.7);
        let progress = CircularProgress::new(0.5, color);
        assert_eq!(progress.color, color);
    }

    #[test]
    fn test_angle_calculation_at_zero_progress() {
        let progress = CircularProgress::new(0.0, Color::from_rgb(1.0, 0.0, 0.0));
        let start_angle = -PI / 2.0;
        let end_angle = start_angle + (2.0 * PI * progress.progress);
        assert_eq!(end_angle, start_angle);
    }

    #[test]
    fn test_angle_calculation_at_half_progress() {
        let progress = CircularProgress::new(0.5, Color::from_rgb(1.0, 0.0, 0.0));
        let start_angle = -PI / 2.0;
        let end_angle = start_angle + (2.0 * PI * progress.progress);
        assert_eq!(end_angle, start_angle + PI);
    }

    #[test]
    fn test_angle_calculation_at_full_progress() {
        let progress = CircularProgress::new(1.0, Color::from_rgb(1.0, 0.0, 0.0));
        let start_angle = -PI / 2.0;
        let end_angle = start_angle + (2.0 * PI * progress.progress);
        assert_eq!(end_angle, start_angle + 2.0 * PI);
    }
}
