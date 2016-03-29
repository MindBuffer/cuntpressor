extern crate compressor;
extern crate dsp;
extern crate time_calc as time;

/// The compressor internally used by the **Cuntpressor**.
pub type Compressor<F> = compressor::RmsMinCompressor<F>;


/// A brutal dynamics processor, designed to force the RMS of a signal to the `target_amp`.
#[derive(Clone)]
pub struct Cuntpressor<F>
    where F: dsp::Frame,
{
    compressor: Compressor<F>,
}


pub const WINDOW_MS: time::Ms = time::Ms(10.0);
pub const ATTACK_MS: time::Ms = time::Ms(1.0);
pub const RELEASE_MS: time::Ms = time::Ms(1.0);
pub const RATIO: f32 = 100.0;


impl<F> Cuntpressor<F>
    where F: dsp::Frame,
{

    /// Construct a new Cuntpressor using an **Rms** with the given window size.
    pub fn new(sample_hz: f64, threshold: f32) -> Self {
        Cuntpressor {
            compressor: Compressor::rms_min(WINDOW_MS, ATTACK_MS, RELEASE_MS, sample_hz,
                                            threshold, RATIO),
        }
    }

}


impl<F> dsp::Node<F> for Cuntpressor<F>
    where F: dsp::Frame,
{
    fn audio_requested(&mut self, output: &mut [F], sample_hz: f64) {
        self.compressor.audio_requested(output, sample_hz)
    }
}
