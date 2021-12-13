mod state;
mod buffer;

pub mod renderer {

	use crate::state::State;

	use winit::{
		event::*,
		event_loop::{ControlFlow, EventLoop},
		window::WindowBuilder,
	};

	pub fn main() {
		env_logger::init();
		let event_loop = EventLoop::new();
		let window = WindowBuilder::new().build(&event_loop).unwrap();
		let mut last_mouse_pos = winit::dpi::PhysicalPosition { x: 0f64, y: 0f64 };

		// State::new uses async code, so we're going to wait for it to finish
		let mut state = pollster::block_on(State::new(&window));
		window.set_title("Ungine");

		event_loop.run(move |event, _, control_flow| {
			match event {
				Event::WindowEvent {
					ref event,
					window_id,
				} if window_id == window.id() => if !state.input(event) { // UPDATED!
					match event {
						WindowEvent::CloseRequested
						| WindowEvent::KeyboardInput {
							input:
							KeyboardInput {
								state: ElementState::Pressed,
								virtual_keycode: Some(VirtualKeyCode::Escape),
								..
							},
							..
						} => *control_flow = ControlFlow::Exit,
						WindowEvent::Resized(physical_size) => {
							state.resize(*physical_size);
						}
						WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
							state.resize(**new_inner_size);
						}
						WindowEvent::CursorMoved { device_id, position, .. } => {
							if last_mouse_pos.x < position.x{
								if state.green > 0.0f64 {
									state.green = state.green - 0.1;
								}
							} else if last_mouse_pos.x > position.x {
								if state.green < 1.0f64 {
									state.green = state.green + 0.1;
								}
							}
							if last_mouse_pos.y < position.y {
								if state.blue > 0.0f64 {
									state.blue = state.blue - 0.1;
								}
							} else if last_mouse_pos.y > position.y {
								if state.blue < 1.0f64 {
									state.blue = state.blue + 0.1;
								}
							}
							last_mouse_pos = position.clone();
						}
						_ => {}
					}
				}
				Event::RedrawRequested(_) => {
					state.update();
					match state.render() {
						Ok(_) => {}
						// Reconfigure the surface if lost
						Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
						// The system is out of memory, we should probably quit
						Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
						// All other errors (Outdated, Timeout) should be resolved by the next frame
						Err(e) => eprintln!("{:?}", e),
					}
				}
				Event::MainEventsCleared => {
					// RedrawRequested will only trigger once, unless we manually
					// request it.
					window.request_redraw();
				}
				_ => {}
			}
		});
	}
}
