use charset_normalizer_rs::from_path;
use encoding_rs::Encoding;
use num_cpus;
use rayon::prelude::*;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn convert_file_encoding(
    infile: &str,
    outfile: &str,
    outencoding: &str,
    normalize_crlf: bool,
) -> Result<(), Box<dyn Error>> {
    let detection = from_path(&PathBuf::from(infile), None).unwrap_or_default();
    let best = detection.get_best().ok_or("No encoding detected")?;
    let src_label = best.encoding();

    let src_enc = Encoding::for_label_no_replacement(src_label.as_bytes())
        .ok_or(format!("Unsupported source encoding: {}", src_label))?;
    let dst_enc = Encoding::for_label_no_replacement(outencoding.as_bytes())
        .ok_or(format!("Unsupported target encoding: {}", outencoding))?;

    println!("Attempting to convert {src_label} to {outencoding}");

    let raw_bytes = fs::read(infile).map_err(|e| format!("Failed to read {}: {}", infile, e))?;
    let threads = num_cpus::get().saturating_sub(1).max(1);

    let chunk_size = (raw_bytes.len() + threads - 1) / threads;
    let slices: Vec<&[u8]> = raw_bytes.chunks(chunk_size).collect();

    let encoded_chunks: Vec<Vec<u8>> = slices
        .into_par_iter()
        .map(|slice| {
            let (decoded, _, had_errors) = src_enc.decode(slice);
            if had_errors {
                eprintln!("Warning: some invalid sequences were replaced during decoding");
            }

            let text = if normalize_crlf {
                decoded.replace("\r\n", "\n")
            } else {
                decoded.into_owned()
            };

            let (encoded, _, _) = dst_enc.encode(&text);
            encoded.into_owned()
        })
        .collect();

    let result: Vec<u8> = encoded_chunks.into_iter().flatten().collect();
    fs::write(outfile, &result).map_err(|e| format!("Failed to write {}: {}", outfile, e))?;

    Ok(())
}
