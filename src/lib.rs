extern crate compressor;
extern crate dsp;
extern crate time_calc as time;

use dsp::Sample;
use time::Ms;

/// The compressor internally used by the **Cuntpressor**.
pub type Compressor = compressor::RmsCompressor<compressor::Minimum>;


/// A brutal dynamics processor, designed to force the RMS of a signal to the `target_amp`.
#[derive(Clone, Debug)]
pub struct Cuntpressor {
    compressor: Compressor,
}


pub const WINDOW_MS: time::Ms = Ms(10.0);
pub const ATTACK_MS: time::Ms = Ms(1.0);
pub const RELEASE_MS: time::Ms = Ms(1.0);
pub const RATIO: f32 = 100.0;


impl Cuntpressor {

    /// Construct a new Cuntpressor using an **Rms** with the given window size.
    pub fn new(sample_hz: f64, n_channels: usize, threshold: f32) -> Self {
        Cuntpressor {
            compressor: Compressor::new(WINDOW_MS, ATTACK_MS, RELEASE_MS, sample_hz, n_channels,
                                        threshold, RATIO),
        }
    }

    // /// A new Cuntpressor with the given capacity.
    // pub fn with_capacity<I: Into<Ms>>(window_ms: I, settings: dsp::Settings) -> Self {
    //     Cuntpressor {
    //         rms: Rms::with_capacity(window_ms, settings),
    //     }
    // }

}


impl<S> dsp::Node<S> for Cuntpressor
    where S: dsp::Sample + dsp::sample::Duplex<f32>,
{

    fn audio_requested(&mut self, output: &mut [S], settings: dsp::Settings) {
        self.compressor.audio_requested(output, settings)
        // self.rms.update(output, settings);

        // let n_frames = settings.frames as usize;
        // let n_channels = settings.channels as usize;
        // let mut idx = 0;
        // for i in 0..n_frames {
        //     let rms_per_channel = self.rms.per_channel(i);
        //     for j in 0..n_channels {
        //         let wave = output[idx].to_wave();
        //         let rms = rms_per_channel[j];
        //         let rms_normaliser = if rms > 0.0 { 1.0 / rms } else { 0.0 };
        //         output[idx] = Sample::from_wave(wave * rms_normaliser);
        //         idx += 1;
        //     }
        // }
    }

}
