use std::{borrow::Cow, sync::mpsc, thread};

use aisd::{multitap, t9, Input, InputMode, Language};
use image::{GenericImageView, ImageFormat};
use tao::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoopBuilder},
	window::{Icon, Window},
};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
	fmt::{format::FmtSpan, Layer},
	prelude::*,
	EnvFilter,
};
use wry::{http::Response, WebViewBuilder};

const PAGE: &[u8] = include_bytes!("../assets/page.html");
const SCRIPT: &[u8] = include_bytes!("../assets/page.js");
const CSS: &[u8] = include_bytes!("../assets/page.css");
const ICON: &[u8] = include_bytes!("../assets/icon.svg");
const FONTS: &[u8] = include_bytes!("../assets/fonts.css");
const FONT_SARABUN_400_EXT: &[u8] = include_bytes!("../assets/sarabun-400-ext.woff2");
const FONT_SARABUN_400: &[u8] = include_bytes!("../assets/sarabun-400.woff2");
const FONT_SARABUN_700_EXT: &[u8] = include_bytes!("../assets/sarabun-700-ext.woff2");
const FONT_SARABUN_700: &[u8] = include_bytes!("../assets/sarabun-700.woff2");
const FONT_VT323_400_EXT: &[u8] = include_bytes!("../assets/vt323-400-ext.woff2");
const FONT_VT323_400: &[u8] = include_bytes!("../assets/vt323-400.woff2");

enum Message {
	Input(Input),
	Lang(Language),
	Mode(InputMode),
}

fn main() -> ! {
	tracing_subscriber::registry()
		.with(Layer::default().with_span_events(FmtSpan::CLOSE))
		.with(
			EnvFilter::builder()
				.with_default_directive(LevelFilter::INFO.into())
				.with_env_var("T9_LOG")
				.from_env_lossy(),
		)
		.init();

	let mut init = Some(thread::spawn(aisd::init));

	let event_loop = EventLoopBuilder::<String>::with_user_event().build();
	let window = Window::new(&event_loop).unwrap();
	let minicon = image::load_from_memory_with_format(
		include_bytes!("../assets/minicon.png"),
		ImageFormat::Png,
	)
	.unwrap();
	let (width, height) = minicon.dimensions();
	let minicon = Some(Icon::from_rgba(minicon.into_rgba8().into_vec(), width, height).unwrap());

	#[cfg(windows)]
	{
		use tao::platform::windows::WindowExtWindows;

		let icon = image::load_from_memory_with_format(
			include_bytes!("../assets/icon.png"),
			ImageFormat::Png,
		)
		.unwrap();
		let (width, height) = icon.dimensions();
		let icon = Some(Icon::from_rgba(icon.into_rgba8().into_vec(), width, height).unwrap());
		window.set_taskbar_icon(icon);
	}

	window.set_maximized(true);
	window.set_title("T9");
	window.set_window_icon(minicon);

	let (tx, rx) = mpsc::channel();

	#[cfg(not(target_os = "linux"))]
	let builder = WebViewBuilder::new(&window);

	#[cfg(target_os = "linux")]
	let gtk_fixed = {
		use gtk::prelude::BoxExt;
		use tao::platform::unix::WindowExtUnix;

		let vbox = window.default_vbox().unwrap();
		let fixed = gtk::Fixed::new();
		vbox.pack_start(&fixed, true, true, 0);
		fixed
	};

	#[cfg(target_os = "linux")]
	let builder = {
		use wry::WebViewBuilderExtUnix;

		WebViewBuilder::new_gtk(&gtk_fixed)
	};

	let webview = builder
		.with_background_color((0x1a, 0x22, 0x32, 0xff))
		.with_ipc_handler(move |msg| match msg.split_once(':') {
			Some(("input", input)) => {
				if input.chars().count() != 1 {
					panic!("invalid input length for '{input}'");
				}

				let Some(char) = input.chars().next() else {
					unreachable!();
				};

				tx.send(Message::Input(char.try_into().expect("invalid input")))
					.unwrap();
			}
			Some(("mode", mode)) => match mode {
				"MT" => tx.send(Message::Mode(InputMode::Multitap)).unwrap(),
				"T9" => tx.send(Message::Mode(InputMode::T9)).unwrap(),
				msg => panic!("invalid IPC mode message '{msg}'"),
			},
			Some(("lang", lang)) => match lang {
				"EN" => tx.send(Message::Lang(Language::En)).unwrap(),
				"PL" => tx.send(Message::Lang(Language::Pl)).unwrap(),
				msg => panic!("invalid IPC language message '{msg}'"),
			},
			msg => panic!("invalid ICP message: '{msg:?}'"),
		})
		.with_custom_protocol("t9".to_string(), |req| {
			Response::new(Cow::Borrowed(
				match req.uri().path().trim_start_matches('/') {
					"" => PAGE,
					"page.js" => SCRIPT,
					"page.css" => CSS,
					"icon.svg" => ICON,
					"fonts.css" => FONTS,
					"sarabun-400-ext.woff2" => FONT_SARABUN_400_EXT,
					"sarabun-400.woff2" => FONT_SARABUN_400,
					"sarabun-700-ext.woff2" => FONT_SARABUN_700_EXT,
					"sarabun-700.woff2" => FONT_SARABUN_700,
					"vt323-400-ext.woff2" => FONT_VT323_400_EXT,
					"vt323-400.woff2" => FONT_VT323_400,
					_ => b"not found",
				},
			))
		})
		.with_url("t9://t9/")
		.unwrap()
		.build()
		.unwrap();

	let proxy = event_loop.create_proxy();
	let update_page = move |suggestions: [_; 3]| {
		proxy
			.send_event(format!(
				r#"
					window.suggestions = ["{}", "{}", "{}"];
					window.refresh();
				"#,
				suggestions[0], suggestions[1], suggestions[2]
			))
			.expect("event loop could not be woken up");
	};

	let mut buf = Vec::new();
	let mut lang = Language::default();
	let mut mode = InputMode::default();

	thread::spawn(move || loop {
		let Ok(msg) = rx.recv() else {
			break;
		};

		match msg {
			Message::Lang(new) => lang = new,
			Message::Mode(new) => mode = new,
			Message::Input(input) => match input {
				Input::Backspace => {
					let last = buf.pop();
					while !buf.is_empty() && buf.last() == last.as_ref() {
						buf.pop();
					}
				}
				Input::Next => {}
				Input::Space => {
					buf.clear();
				}
				Input::Word(char) => buf.push(char),
			},
		}

		if let Some(init) = init.take() {
			let _ = init.join();
		}

		update_page(match mode {
			InputMode::Multitap => multitap(&buf, lang),
			InputMode::T9 => t9(&buf, lang),
		});
	});

	event_loop.run(move |event, _, cf| match event {
		Event::UserEvent(script) => webview
			.evaluate_script(&script)
			.expect("could not evaluate script"),
		Event::LoopDestroyed
		| Event::WindowEvent {
			event: WindowEvent::CloseRequested,
			..
		} => *cf = ControlFlow::Exit,
		_ => *cf = ControlFlow::Wait,
	})
}
