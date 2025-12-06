use std::collections::VecDeque;
use std::error;
use std::sync::mpsc;
use std::thread;
use rocket::futures::SinkExt;
use wasapi::*;

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

#[macro_use] extern crate rocket;

#[macro_use]
extern crate log;
use simplelog::*;

type Res<T> = Result<T, Box<dyn error::Error>>;

fn capture_loop(tx_capt: std::sync::mpsc::SyncSender<Vec<u8>>, chunksize: usize) -> Res<()> {
    let enumerator = DeviceEnumerator::new()?;
    let device = enumerator.get_default_device(&Direction::Capture)?;
    let mut audio_client = device.get_iaudioclient()?;

    let desired_format = WaveFormat::new(32, 32, &SampleType::Float, 44100, 2, None);

    let blockalign = desired_format.get_blockalign();
    debug!("Desired capture format: {:?}", desired_format);

    let (def_time, min_time) = audio_client.get_device_period()?;
    debug!("default period {}, min period {}", def_time, min_time);

    let mode = StreamMode::EventsShared {
        autoconvert: true,
        buffer_duration_hns: min_time,
    };
    audio_client.initialize_client(&desired_format, &Direction::Capture, &mode)?;
    debug!("initialized capture");

    let h_event = audio_client.set_get_eventhandle()?;

    let buffer_frame_count = audio_client.get_buffer_size()?;

    let render_client = audio_client.get_audiocaptureclient()?;
    let mut sample_queue: VecDeque<u8> = VecDeque::with_capacity(
        100 * blockalign as usize * (1024 + 2 * buffer_frame_count as usize),
    );
    audio_client.start_stream()?;
    loop {
        while sample_queue.len() > (blockalign as usize * chunksize) {
            debug!("pushing samples");
            let mut chunk = vec![0u8; blockalign as usize * chunksize];
            for element in chunk.iter_mut() {
                *element = sample_queue.pop_front().unwrap();
            }
            tx_capt.send(chunk)?;
        }
        trace!("capturing");
        render_client.read_from_device_to_deque(&mut sample_queue)?;
        if h_event.wait_for_event(1000000).is_err() {
            error!("error, stopping capture");
            audio_client.stop_stream()?;
            break;
        }
    }
    Ok(())
}



#[get("/mic")]
fn mic_stream(_ws: rocket_ws::WebSocket) -> rocket_ws::Stream!['static] {
    let (tx_capt, rx_capt): (
        std::sync::mpsc::SyncSender<Vec<u8>>,
        std::sync::mpsc::Receiver<Vec<u8>>,
    ) = mpsc::sync_channel(2);

    let _handle = thread::Builder::new()
        .name("Capture".to_string())
        .spawn(move || {
            let result = capture_loop(tx_capt, 1024);
            if let Err(err) = result {
                error!("Capture failed with error {}", err);
            }
        });

    rocket_ws::Stream! { _ws => 
        loop {
            if let Ok(msg) = rx_capt.recv() {
                yield rocket_ws::Message::Binary(msg);
            }
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let _ = SimpleLogger::init(
        LevelFilter::Debug,
        ConfigBuilder::new()
            .set_time_format_rfc3339()
            .set_time_offset_to_local()
            .unwrap()
            .build(),
    );

    initialize_mta().ok().unwrap();

    rocket::build().mount("/", routes![
        index,
        mic_stream
    ]).mount("/", 
        rocket::fs::FileServer::new("./www", Default::default())
    )
}
