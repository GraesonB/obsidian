use anyhow::Result;
use log::{error, info};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::logger::create_logger; // create here

pub struct AppConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub logfile_name: String
}

pub fn create_window(config: &AppConfig) -> Result<(EventLoop<()>, Window)> {
    let event_loop = EventLoop::new();

    //builder idiom -STUDY THIS
    let window = WindowBuilder::new()
        .with_title(config.title.to_string())
        .with_inner_size(PhysicalSize::new(config.width, config.height))
        .build(&event_loop)?; //question mark syntax is from anyhow crate, just for errors.

    Ok((event_loop, window))
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: String::from("Obsidian Application"),
            logfile_name: String::from("obsidian.log"),
        }
    }
}

pub struct Application;

pub trait Run {
    fn initialize(&mut self, _application: &mut Application) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, _application: &mut Application) -> Result<()> {
        Ok(())
    }
}

// runner is using the trait run as a parameter, i.e. anything that has the trait Run can be passed in as
// runner. Also only takes runners that have 'static lifetime, which is data that lives as long as the
// application is running
pub fn run_application(mut runner: impl Run + 'static, configuration: AppConfig) -> Result<()> {
    create_logger(&configuration.logfile_name);
    let (event_loop, _window) = create_window(&configuration)?;

    let mut application = Application {};

    log::info!("Running Application");
    runner.initialize(&mut application)?;

    event_loop.run(move |event, _, control_flow| {
        let mut cycle_result = || -> Result<()> {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::MainEventsCleared => {
                    runner.update(&mut application)?;
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::LoopDestroyed => {
                    info!("Exited application");
                }
                _ => {}
            }
            Ok(())
        };
        if let Err(error) = cycle_result() {
            error!("Application Error: {}", error);
        }
    });
}
