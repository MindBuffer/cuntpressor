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
    const CHANNELS: u16 = 2;
    // The size of the **Rms**' moving **Window**.
    const SAMPLE_RATE: f64 = 44_100.0;
    const FRAMES: u32 = 128;
    const THRESHOLD: f32 = 0.1;

    // Construct our Rms reader.
    let mut cuntpressor = Cuntpressor::new(SAMPLE_RATE, CHANNELS as usize, THRESHOLD);

    // Callback used to construct the duplex sound stream.
    let callback = move |pa::DuplexStreamCallbackArgs { in_buffer, out_buffer, frames, .. }| {

        // Write the input to the output for fun.
        for (out_sample, in_sample) in out_buffer.iter_mut().zip(in_buffer.iter()) {
            *out_sample = *in_sample;
        }

        let settings = dsp::Settings::new(SAMPLE_RATE as u32, frames as u16, CHANNELS);
        cuntpressor.audio_requested(out_buffer, settings);

        println!("{:?}", &out_buffer[0..8]);

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
