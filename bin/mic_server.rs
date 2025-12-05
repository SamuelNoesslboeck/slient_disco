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

#[get("/mic")]
fn echo_stream(ws: rocket_ws::WebSocket) -> rocket_ws::Channel<'static> {
    ws.channel(move |mut stream| {
        let chunksize = 4096;

        let enumerator = DeviceEnumerator::new().unwrap();
        let device = enumerator.get_default_device(&Direction::Capture).unwrap();
        let mut audio_client = device.get_iaudioclient().unwrap();

        let desired_format = WaveFormat::new(32, 32, &SampleType::Float, 44100, 2, None);

        let blockalign = desired_format.get_blockalign();
        debug!("Desired capture format: {:?}", desired_format);

        let (def_time, min_time) = audio_client.get_device_period().unwrap();
        debug!("default period {}, min period {}", def_time, min_time);

        let mode = StreamMode::EventsShared {
            autoconvert: true,
            buffer_duration_hns: min_time,
        };
        audio_client.initialize_client(&desired_format, &Direction::Capture, &mode).unwrap();
        debug!("initialized capture");

        let h_event = audio_client.set_get_eventhandle().unwrap();

        let buffer_frame_count = audio_client.get_buffer_size().unwrap();

        let render_client = audio_client.get_audiocaptureclient().unwrap();
        let mut sample_queue: VecDeque<u8> = VecDeque::with_capacity(
            100 * blockalign as usize * (1024 + 2 * buffer_frame_count as usize),
        );
        audio_client.start_stream().unwrap();
        loop {
            while sample_queue.len() > (blockalign as usize * chunksize) {
                debug!("pushing samples");
                let mut chunk = vec![0u8; blockalign as usize * chunksize];
                for element in chunk.iter_mut() {
                    *element = sample_queue.pop_front().unwrap();
                }

                let future = stream.send(rocket_ws::Message::Binary(chunk));

                // pin it
                let mut future = Box::pin(future);

                // create a dummy waker (real executors provide proper ones)
                let waker = futures::task::noop_waker();
                let mut cx = Context::from_waker(&waker);

                match Future::poll(Pin::as_mut(&mut future), &mut cx) {
                    Poll::Ready(_) => println!("Completed!"),
                    Poll::Pending => println!("Still pending"),
                }
            }
            trace!("capturing");
            render_client.read_from_device_to_deque(&mut sample_queue).unwrap();
            if h_event.wait_for_event(1000000).is_err() {
                error!("error, stopping capture");
                audio_client.stop_stream().unwrap();
                panic!("Error");
            }
        }
    })
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
        echo_stream
    ])
}
