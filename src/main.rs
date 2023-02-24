use raspicam::{SimpleRaspiCam, RaspiCamConfig};
use std::thread::sleep;
use std::time::Duration;
use time::OffsetDateTime;

fn main() {
    let config = RaspiCamConfig {
        width: 1280,
        height: 720,
        fps: 30,
        shutter_speed: 6000000,
        ..Default::default()
    };

    let mut camera = SimpleRaspiCam::new(&config).unwrap();
    sleep(Duration::from_secs(2));

    for i in 0..60 {
        let filename = format!("/home/pi/timelapse/image{:02}.jpg", i);
        camera.take_jpeg(&filename).unwrap();
        sleep(Duration::from_secs(5));
    }
}
