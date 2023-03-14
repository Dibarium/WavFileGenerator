#![deny(warnings)]
#![warn(missing_docs)]
//!Super projet de fouu

use project::{createwaveform, createsound};
///Fonction main
fn main() {
    /* 
    let mut v: Vec<f64> = Vec::new();
    let mut value = 0.64; 
    for _i in 1..=10 {
        value = boucle(value);
        println!("le nombre est {}", value);
        v.push(value);
    }
    println!("il y a {} elements qui sont {:?}", v.len(), v);

    v.resize(v.len(), 0.0);
    for i in v.iter_mut(){
        let z = 1.0;
        *i += z;
    }
    println!("il y a {} elements qui sont {:?}", v.len(), v);
    */
    
    let frequence = 1000.0;
    let sample_rate = 44100.0;
    let wave = createwaveform(frequence, 100.0, sample_rate);
    println!("{:?}",wave);
    println!("{:?}",createsound("sound.wav", sample_rate, &wave));
}
