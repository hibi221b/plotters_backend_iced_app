mod custom_plot_backend;
use custom_plot_backend::{CustomPlotFrame, Plottable};

use plotters::{
    drawing::IntoDrawingArea,
    prelude::*,
};

use iced::{
    canvas::{self, Cache, Canvas, Cursor, Geometry},
    executor, Application, Command, Container, Element, Length,
    Rectangle, Settings, Subscription
};

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.antialiasing = true;

    PlotterExample::run(settings)
}


struct PlotterExample {
    cache: Cache
}

#[derive(Debug, Clone, Copy)]
enum Message {
}

impl Application for PlotterExample {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            PlotterExample {
                cache: Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("plotters on Iced")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn view(&mut self) -> Element<Message> {
        let plot = Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(plot)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl<Message> canvas::Program<Message> for PlotterExample {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let plotter_geometory = self.cache.draw(bounds.size(), |frame| {
            self.draw_plot(CustomPlotFrame::new(frame))
        });

        vec![
            plotter_geometory
        ]
    }
}

impl Plottable for PlotterExample {
    fn draw_plot(&self, frame: CustomPlotFrame) { 
        let root_drawing_area = frame.into_drawing_area();
        root_drawing_area.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root_drawing_area)
            .caption("y=x^2", ("sans-serif", 24).into_font())
            .margin(16)
            .x_label_area_size(24)
            .y_label_area_size(24)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                &RED
            ))
            .unwrap()
            .label("y = x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK.mix(0.3))
            .draw()
            .unwrap();
    }
}