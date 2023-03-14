#![deny(warnings)]
#![warn(missing_docs)]


//! Ensemble des foctions
use std::{str, f32::consts::PI};
use std::fs::File;
use std::io::prelude::*;

const SIZE: usize = 2000000;
/// Fonction qui double le parametre 10 fois et le retourne.
pub fn boucle(_nb: f64) -> f64 {
    let mut nombre  = _nb as f64;
    for _i in 1..=10 {
        nombre = (nombre * 2.2).sin();
    }
    return nombre;
}

/// Créer le header d'un fichier wav
fn createheader(sample_rate:f32, samples:&[f32])->[u8; 44]{
    let sample_size = 2; // 16 bits
    let channel_count = 1; // monophonic
    let size = (channel_count * samples.len() * sample_size) as u32;
    let chunk_size = 36 + size;
    let bits_per_sample = (8 * sample_size) as u16;
    let byte_rate =
        (sample_rate * channel_count as f32 * bits_per_sample as f32 / 8.0)
            as u32;
    let mut header = [0_u8; 44];
    header[0..4].copy_from_slice(b"RIFF");
    header[4..8].copy_from_slice(&chunk_size.to_le_bytes());
    header[8..12].copy_from_slice(b"WAVE");
    header[12..16].copy_from_slice(b"fmt ");
    header[16..20].copy_from_slice(&16_u32.to_le_bytes()); // subchunk1size
    header[20..22].copy_from_slice(&1_u16.to_le_bytes()); // pcm
    header[22..24].copy_from_slice(&(channel_count as u16).to_le_bytes());
    header[24..28].copy_from_slice(&(sample_rate as u32).to_le_bytes());
    header[28..32].copy_from_slice(&byte_rate.to_le_bytes());
    header[32..34].copy_from_slice(&4_u16.to_le_bytes()); // block-align
    header[34..36].copy_from_slice(&bits_per_sample.to_le_bytes());
    header[36..40].copy_from_slice(b"data");
    header[40..44].copy_from_slice(&size.to_le_bytes());
    return header
}

/// Créer un fichier wav
pub fn createsound(path:&str, sample_rate:f32, samples:&[f32])-> std::io::Result<()>{
    let mut file = File::create(path)?;
    let header = createheader(sample_rate, samples);
    file.write_all(&header)?;
    for i in samples.iter(){
        if i > &1.0{
            file.write_all(&(i16::MAX).to_le_bytes())?;
        }
        else if i<&-1.0 {
            file.write_all(&(-i16::MAX).to_le_bytes())?; 
        }
        else {
            file.write_all(&((*i*(i16::MAX as f32)) as i16).to_le_bytes())?;
        }
    }
    
    
    Ok(())
}

/// Créer une wave forme
pub fn createwaveform(frequence: f32, amplitude: f32, sample_rate: f32)-> [f32; SIZE]{
    let echantillonage: f32 = 1.0/sample_rate;
    let mut v = [0.0;SIZE];
    let mut t = 0.0;
    for i in 0..SIZE{
        v[i] = amplitude*f32::sin(2.0*PI*frequence*t);
        t+=echantillonage;
    }
    return v
}