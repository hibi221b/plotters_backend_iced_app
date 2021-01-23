use iced::{Color, HorizontalAlignment, Point, Size, VerticalAlignment, canvas::{self, Path, Stroke}};

use plotters_backend::{
    DrawingBackend, DrawingErrorKind, BackendColor, BackendCoord, BackendStyle, BackendTextStyle
};

use std::error::Error;
use std::result::Result;

pub trait Plottable {
    fn draw_plot<'a>(&self, backend: CustomPlotFrame<'a>);
}

pub struct CustomPlotFrame<'a> {
    pub frame: &'a mut canvas::Frame
}

impl<'a> CustomPlotFrame<'a> {
    pub fn new(frame: &'a mut canvas::Frame) -> Self {
        Self {
            frame
        }
    }
}

#[derive(Debug)]
pub enum PlotErr {}

impl std::fmt::Display for PlotErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plotting Error")
    }
}

impl Error for PlotErr {}

impl<'a> CustomPlotFrame<'a> {
    fn gen_bg_color(bg_color: &BackendColor) -> Color {
        Color::from_rgba8(
            bg_color.rgb.0,
            bg_color.rgb.1,
            bg_color.rgb.2,
            bg_color.alpha as f32
        )
    }

    fn gen_bg_point(bg_point: &BackendCoord) -> Point {
        Point::new(bg_point.0 as f32, bg_point.1 as f32)
    }
}

impl<'a> DrawingBackend for CustomPlotFrame<'a> {
    type ErrorType = PlotErr;

    fn get_size(&self) -> (u32, u32) {
        (self.frame.width() as u32, self.frame.height() as u32)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> { 
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {  
        Ok(())
    }

    fn draw_pixel(
        &mut self, 
        point: BackendCoord, 
        color: BackendColor
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> { 
        let color = Self::gen_bg_color(&color);
        let point = Self::gen_bg_point(&point);
        let size = Size::new(1.0, 1.0);
        let path = Path::rectangle(point, size);

        self.frame.fill(&path, color);
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = Self::gen_bg_color(&style.color());
        let path = Path::line(
        Self::gen_bg_point(&from),
            Self::gen_bg_point(&to),
        );

        let stroke = Stroke {
            color,
            width: style.stroke_width() as f32,
            ..Default::default()
        };

        self.frame.stroke(&path, stroke);
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = Self::gen_bg_color(&style.color());
        let size = Size::new(
            (bottom_right.0 - upper_left.0) as f32,
            (bottom_right.1 - upper_left.1) as f32
        );

        let path = Path::rectangle(Self::gen_bg_point(&upper_left), size);

        if fill {
            self.frame.fill(&path, color);
        } else {
            let stroke = Stroke {
                color,
                width: style.stroke_width() as f32,
                ..Default::default()
            };

            self.frame.stroke(&path, stroke);
        }

        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = Self::gen_bg_color(&style.color());
        let path = Path::circle(
            Self::gen_bg_point(&center),
            radius as f32
        );

        if fill {
            self.frame.fill(&path, color);
        } else {
            let stroke = Stroke {
                color,
                width: style.stroke_width() as f32,
                ..Default::default()
            };

            self.frame.stroke(&path, stroke);
        }

        Ok(())
    }

    fn draw_text<TStyle: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &TStyle,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = Self::gen_bg_color(&style.color());

        self.frame.fill_text(
            iced::canvas::Text {
                content: text.to_string(),
                size: style.size() as f32,
                position: Self::gen_bg_point(&pos),
                color,
                horizontal_alignment: HorizontalAlignment::Center,
                vertical_alignment: VerticalAlignment::Center,
                ..iced::canvas::Text::default()
            }
        );

        Ok(())
    }
}