use anyhow::Result;
use tao::{event::{Event, StartCause, WindowEvent}, event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget}, window::WindowBuilder};
#[cfg(target_os = "android")]
use wry::android_binding;
use wry::{
    WebView, WebViewBuilder
};

#[cfg(target_os = "android")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Trace)
            .with_tag("dioxus-mobile-test"),
    );
}

#[cfg(target_os = "android")]
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
            std::process::abort()
        }
    }
}

#[cfg(target_os = "android")]
fn _start_app() {
    stop_unwind(|| main().unwrap());
}

#[no_mangle]
#[inline(never)]
#[cfg(target_os = "android")]
pub extern "C" fn start_app() {
    #[cfg(target_os = "android")]
{
  tao::android_binding!(
      com_example,
      wry_app,
      WryActivity,
      wry::android_setup, // pass the wry::android_setup function to tao which will invoke when the event loop is created
      _start_app
  );
  wry::android_binding!(com_example, ttt);
}
    #[cfg(target_os = "ios")]
    _start_app()
}

pub fn main() -> Result<()> {
    #[cfg(target_os = "android")]
    init_logging();
    let event_loop = EventLoop::new();

    let mut webview = None;
    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                webview = Some(build_webview(event_loop).unwrap());
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested { .. },
                ..
            } => {
                webview.take();
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });
}

fn build_webview(event_loop: &EventLoopWindowTarget<()>) -> Result<WebView> {
    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(event_loop)?;
    let webview = WebViewBuilder::new(&window)
        .with_url("https://tauri.app")
        .build()?;

        Ok(webview)
 }
 