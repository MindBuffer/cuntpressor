extern crate cuntpressor;
extern crate dsp;
extern crate portaudio as pa;

use cuntpressor::Cuntpressor;
use dsp::Node;

fn main() {
    run().unwrap()
}

fn run() -> Result<(), pa::Error> {

    // The number of channels we want in our stream.
    const CHANNELS: usize = 2;
    // The size of the **Rms**' moving **Window**.
    const SAMPLE_RATE: f64 = 44_100.0;
    const FRAMES: u32 = 128;
    const THRESHOLD: f32 = 0.1;

    // Construct our Rms reader.
    let mut cuntpressor = Cuntpressor::new(SAMPLE_RATE, THRESHOLD);

    // Callback used to construct the duplex sound stream.
    let callback = move |pa::DuplexStreamCallbackArgs { in_buffer, out_buffer, .. }| {
        let in_buffer: &[[f32; CHANNELS]] = dsp::slice::to_frame_slice(in_buffer).unwrap();
        let out_buffer: &mut [[f32; CHANNELS]] = dsp::slice::to_frame_slice_mut(out_buffer).unwrap();
        dsp::slice::write(out_buffer, in_buffer);

        cuntpressor.audio_requested(out_buffer, SAMPLE_RATE);

        println!("{:?}", &out_buffer[0..4]);

        pa::Continue
    };

    let pa = try!(pa::PortAudio::new());
    let chans = CHANNELS as i32;
    let settings = try!(pa.default_duplex_stream_settings::<f32, f32>(chans, chans, SAMPLE_RATE, FRAMES));
    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));
    try!(stream.start());

    // Wait for our stream to finish.
    while let true = try!(stream.is_active()) {
        ::std::thread::sleep(::std::time::Duration::from_millis(16));
    }

    Ok(())
}
