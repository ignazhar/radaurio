use std::{env, f32::consts::E};
use std::fs::File;
use std::path::Path;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub mod plot;
use crate::plot::{gif_plots, plot};

fn decode_image(file_path: &String) -> Vec<Vec<f64>> {
    // Return vector of vector samples
    let mut samples: Vec<Vec<f64>> = vec![];

    // Create a media source. Note that the MediaSource trait is automatically implemented for File,
    // among other types.
    let file = Box::new(File::open(Path::new(file_path)).unwrap());

    // Create the media source stream using the boxed media source from above.
    let mss = MediaSourceStream::new(file, Default::default());

    // Create a hint to help the format registry guess what format reader is appropriate. In this
    // example we'll leave it empty.
    let hint = Hint::new();

    // Use the default options when reading and decoding.
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    let decoder_opts: DecoderOptions = Default::default();

    // Probe the media source stream for a format.
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .unwrap();

    // Get the format reader yielded by the probe operation.
    let mut format = probed.format;

    // Get the default track.
    let track = format.default_track().unwrap();

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_opts)
        .unwrap();

    // Store the track identifier, we'll use it to filter packets.
    let track_id = track.id;

    let mut min_len = usize::MAX;
    let mut max_len = usize::MIN;
    let mut sample_count = 0;
    let mut sample_buf = None;

    loop {
        // Get the next packet from the format reader.
        let result = format.next_packet();
        // End of a file break
        if let Err(e) = result {
            match e {
                Error::IoError(io_err) if io_err.kind() == std::io::ErrorKind::UnexpectedEof => {
                    println!("=== end of file error");
                }
                _ => {
                    println!("=== unknown error: {}", e);
                }
            }
            // std::process::exit(0);
            break;
        }
        let packet = result.unwrap();

        // If the packet does not belong to the selected track, skip it.
        if packet.track_id() != track_id {
            // println!("skip packet");
            continue;
        }

        // Decode the packet into audio samples, ignoring any decode errors.
        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                // The decoded audio samples may now be accessed via the audio buffer if per-channel
                // slices of samples in their native decoded format is desired. Use-cases where
                // the samples need to be accessed in an interleaved order or converted into
                // another sample format, or a byte buffer is required, are covered by copying the
                // audio buffer into a sample buffer or raw sample buffer, respectively. In the
                // example below, we will copy the audio buffer into a sample buffer in an
                // interleaved order while also converting to a f32 sample format.

                // If this is the *first* decoded packet, create a sample buffer matching the
                // decoded audio buffer format.
                if sample_buf.is_none() {
                    // Get the audio buffer specification.
                    let spec = *audio_buf.spec();

                    // Get the capacity of the decoded buffer. Note: This is capacity, not length!
                    let duration = audio_buf.capacity() as u64;

                    // Create the f32 sample buffer.
                    sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
                }

                // Copy the decoded audio buffer into the sample buffer in an interleaved format.
                if let Some(buf) = &mut sample_buf {
                    buf.copy_interleaved_ref(audio_buf);

                    // The samples may now be access via the `samples()` function.
                    sample_count += buf.samples().len();
                    min_len = min_len.min(buf.samples().len());
                    max_len = max_len.max(buf.samples().len());
                    print!(
                        "\rDecoded {} samples, value: {}",
                        sample_count,
                        buf.samples().last().unwrap()
                    );
                    // plot(buf.samples().into_iter().map(|f| *f as f64).collect()).unwrap();
                    // break;
                    //total.push(*buf.samples().last().unwrap() as f64);
                    // for value in buf.samples() {
                    //     total.push(*value as f64);
                    // }
                    samples.push(buf.samples().into_iter().map(|f| *f as f64).collect());
                }
            }
            Err(Error::DecodeError(_)) => (),
            Err(_) => break,
        }
    }

    println!(
        "buf.samples().len() is in the range [{},{}]",
        min_len, max_len
    );

    samples
}

use rustfft::{num_complex::Complex, FftPlanner};

fn transform_data(samples: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(samples.first().unwrap().iter().len());

    let mut frequencies: Vec<Vec<f64>> = vec![];
    for sample in samples {
        let mut buffer: Vec<Complex<f32>> = sample
            .iter()
            .map(|f| Complex {
                re: *f as f32,
                im: 0.0,
            })
            .collect();

        fft.process(&mut buffer);

        // println!("{}", buffer.len());
        // for complex_value in buffer {
        //     println!("{:?}", complex_value);
        // }
        frequencies.push(buffer.iter().map(|f| f.norm() as f64).collect());
    }
    frequencies
}

fn group_by(spectrogram: &mut Vec<Vec<f64>>, group_size: usize) {
    let mut spectrogram_groupped = vec![];
    for start in (0..spectrogram.len()).step_by(group_size) {
        let mut append = vec![0f64; spectrogram[start].len()];
        for i in 0..spectrogram[start].len() {
            for j in 0..group_size {
                if start + j < spectrogram.len() {
                    append[i] += spectrogram[start + j][i];
                }
            }
        }
        spectrogram_groupped.push(append);
    }
    *spectrogram = spectrogram_groupped;
}

fn get_frequencies(spectrogram: &Vec<Vec<f64>>) -> Vec<f64> {
    let mut frequencies = vec![];
    for distribution in spectrogram {
        let mut biggest_impact: usize = 0;
        let mut biggest_sum = 0f64;
        for i in 0..distribution.len() {
            let mut current_sum = 0f64;
            let sum_step = 5;
            for j in 0.max(i as i32 - sum_step)..(distribution.len() as i32).min(i as i32 + sum_step) {
                current_sum += distribution[j as usize];
            }
            if current_sum > biggest_sum {
                biggest_impact = i;
                biggest_sum = current_sum;
            }
        }
        frequencies.push(biggest_impact as f64);
    }
    frequencies
}

fn main() {
    // Get command line arguments.
    let args: Vec<String> = env::args().collect();

    let samples = decode_image(&args[1]);
    println!("number of vectors of samples: {}", samples.len());

    let mut spectrogram = transform_data(samples);

    // Nyquist–Shannon theorem(truncate in half)
    for sample in &mut spectrogram {
        sample.truncate(sample.len() / 2);
    }

    // group and accumulate to better distinguish the signal from noise
    let group_size = 10usize;
    group_by(&mut spectrogram, group_size);

    // plot(total).unwrap();
    // gif_plots(spectrogram).unwrap();

    let frequencies = get_frequencies(&spectrogram);
    plot(frequencies, "Frequencies chart 1").unwrap();

    /* FFT tests
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(8);

    let mut buffer: Vec<Complex<f32>> = vec![
        Complex { re: 0.0, im: 0.0 },
        Complex { re: -1.0, im: 0.0 },
        Complex { re: 0.0, im: 0.0 },
        Complex { re: 1.0, im: 0.0 },
        Complex { re: 0.0, im: 0.0 },
        Complex { re: -1.0, im: 0.0 },
        Complex { re: 0.0, im: 0.0 },
        Complex { re: 1.0, im: 0.0 },
    ];

    fft.process(&mut buffer);

    for complex_value in buffer {
        println!("{:?}", complex_value);
    }
    */
}
