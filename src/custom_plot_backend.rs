use iced::{
    Color, Point, Size, 
    canvas::{self, Path, Stroke, LineCap, LineJoin}
};

use plotters_backend::{
    DrawingBackend, DrawingErrorKind, BackendColor, BackendCoord, BackendStyle, BackendTextStyle
};

use std::error::Error;
use std::result::Result;

#[derive(Debug)]
pub enum PlotErr {}

impl std::fmt::Display for PlotErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plotting Error")
    }
}

impl Error for PlotErr {}

pub trait Plottable: std::fmt::Debug {
    fn draw_plot<'a>(&self, backend: CustomPlotFrame<'a>);
}

#[derive(Debug)]
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

        let path = Path::rectangle(
            Point::new(point.0 as f32, point.1 as f32),
            Size::new(1.0, 1.0)
        );

        let (r, g, b) = (color.rgb.0, color.rgb.1, color.rgb.2);

        self.frame.fill(&path, Color::from_rgb8(r, g, b));
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        
        let path = Path::line(
            Point::new(from.0 as f32, from.1 as f32),
            Point::new(to.0 as f32, to.1 as f32)
        );

        let (r, g, b) = (style.color().rgb.0, style.color().rgb.1, style.color().rgb.2);
        let stroke = Stroke {
            color: Color::from_rgb8(r, g, b),
            width: style.stroke_width() as f32,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter
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

        let path = Path::rectangle(
            Point::new(bottom_right.0 as f32, bottom_right.1 as f32),
            Size::new(
                (upper_left.0 - bottom_right.0) as f32,
                (upper_left.1 - bottom_right.1) as f32
            )
        );
        
        let (r, g, b) = (style.color().rgb.0, style.color().rgb.1, style.color().rgb.2);
        let color = Color::from_rgb8(r, g, b);

        if fill {
            self.frame.fill(&path, color);
        } else {
            let stroke = Stroke {
                color,
                width: style.stroke_width() as f32,
                line_cap: LineCap::Butt,
                line_join: LineJoin::Miter
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

        let path = Path::circle(
            Point::new(center.0 as f32, center.1 as f32),
            radius as f32
        );

        let (r, g, b) = (style.color().rgb.0, style.color().rgb.1, style.color().rgb.2);
        let color = Color::from_rgb8(r, g, b);

        if fill {
            self.frame.fill(&path, color);
        } else {
            let stroke = Stroke {
                color,
                width: style.stroke_width() as f32,
                line_cap: LineCap::Butt,
                line_join: LineJoin::Miter
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

        let (r, g, b) = (style.color().rgb.0, style.color().rgb.1, style.color().rgb.2);

        self.frame.fill_text(
            iced::canvas::Text {
                content: text.to_string(),
                size: style.size() as f32,
                position: Point::new(
                    pos.0 as f32 - 10.0,
                    pos.1 as f32 -4.0,
                ),
                color: Color::from_rgb8(r, g, b),
                ..iced::canvas::Text::default()
            }
        );
        Ok(())
    }
}