use rrr_config::Config;
use winit::{
    dpi::PhysicalPosition, dpi::PhysicalSize, event::TouchPhase, event_loop::EventLoop,
    window::WindowBuilder,
};

pub fn init(config: Config, song_id: u16) {
    println!("Play song_id {:?}", song_id);

    let event_loop = EventLoop::new();
    let size = PhysicalSize::new(config.width, config.height);
    let _window_res = WindowBuilder::new()
        .with_title("Rust Rust Revolution")
        .with_inner_size(size)
        .with_resizable(false)
        .with_position(PhysicalPosition::new(config.window_x, config.window_y))
        .build(&event_loop);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        #[allow(deprecated)]
        match event {
            winit::event::Event::NewEvents(_) => {}

            winit::event::Event::DeviceEvent {
                device_id: _,
                event: _,
            } => {}

            winit::event::Event::UserEvent(_) => {}

            winit::event::Event::Suspended => {}

            winit::event::Event::Resumed => {}

            winit::event::Event::MainEventsCleared => {}

            winit::event::Event::RedrawRequested(_) => {}

            winit::event::Event::RedrawEventsCleared => {}

            winit::event::Event::LoopDestroyed => {}

            winit::event::Event::WindowEvent { window_id, event } => match event {
                winit::event::WindowEvent::Resized(size) => {
                    println!("Resized to: {:?}", size)
                }
                winit::event::WindowEvent::Moved(position) => {
                    println!("Moved to: {:?}", position)
                }
                winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                winit::event::WindowEvent::Destroyed => {}
                winit::event::WindowEvent::DroppedFile(_) => {}
                winit::event::WindowEvent::HoveredFile(_) => {}
                winit::event::WindowEvent::HoveredFileCancelled => {}
                winit::event::WindowEvent::ReceivedCharacter(_) => {}
                winit::event::WindowEvent::Focused(focused) => {
                    println!("Window {:?} focused: {:?}", window_id, focused);
                }
                winit::event::WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                } => {
                    println!(
                        "Keybaord input: {:?} | {:?} | {:?}",
                        device_id, input, is_synthetic
                    )
                }
                winit::event::WindowEvent::ModifiersChanged(_) => {}
                winit::event::WindowEvent::Ime(_) => {}
                winit::event::WindowEvent::CursorMoved {
                    device_id: _,
                    position: _,
                    modifiers: _,
                } => {}
                winit::event::WindowEvent::CursorEntered { device_id: _ } => {}
                winit::event::WindowEvent::CursorLeft { device_id: _ } => {}
                winit::event::WindowEvent::MouseWheel {
                    device_id: _,
                    delta: _,
                    phase: _,
                    modifiers: _,
                } => {}
                winit::event::WindowEvent::MouseInput {
                    device_id: _,
                    state: _,
                    button: _,
                    modifiers: _,
                } => {}
                winit::event::WindowEvent::TouchpadPressure {
                    device_id: _,
                    pressure: _,
                    stage: _,
                } => {}
                winit::event::WindowEvent::AxisMotion {
                    device_id: _,
                    axis: _,
                    value: _,
                } => {}
                winit::event::WindowEvent::Touch(touch) => {
                    if touch.phase == TouchPhase::Ended {
                        println!("Input recieved.");
                    }
                }
                winit::event::WindowEvent::ScaleFactorChanged {
                    scale_factor: _,
                    new_inner_size: _,
                } => {}
                winit::event::WindowEvent::ThemeChanged(_) => {}
                winit::event::WindowEvent::Occluded(_) => {}
            },
        }
    });
}
