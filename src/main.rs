use rascam::*;
use std::fs::File;
use std::io::Write;
use std::{thread, time};
use std::process::exit;
use tracing::{error, info};

fn main() {
	// Set up logging to stdout
	tracing_subscriberl::fmt::init();

	let info = info().unwrap();
	if info.cameras.len() < 1 {
		error!("No cameras found!");
		exit(1);
	}

	info!("{}", info);

	take_picture(&info.cameras[0])
}

fn take_picture(info: &CameraInfo) {
	let mut camera = SimpleCamera::new(info.clone().unwrap()).unwrap();
	camera.activate().unwrap();

	let sleep_duration = time::Duration::from_secs(2);
	thread::sleep(sleep_duration);

	let capture = camera.take_one().unwrap();
	File::create("out.jpg").unwrap().write_all(&capture).unwrap();
	info!("Saved image as out.jpg")
}