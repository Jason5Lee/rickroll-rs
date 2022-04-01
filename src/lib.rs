extern crate proc_macro;

use proc_macro::TokenStream;
use std::{io::Cursor, time::Duration};
use rodio::{source::Source, Decoder, OutputStream};

macro_rules! unwrap_or_return {
    ($r:expr) => {
        match $r {
            Ok(r) => r,
            Err(_) => {
                return;
            }
        }  
    };
}
fn rr() {
    const AUDIO: &[u8] = include_bytes!("../rickroll.ogg");

    let (_stream, stream_handle) = unwrap_or_return!(OutputStream::try_default());
    let source = unwrap_or_return!(Decoder::new(Cursor::new(AUDIO)));
    unwrap_or_return!(stream_handle.play_raw(source.convert_samples()));

    std::thread::sleep(Duration::from_secs(6));
}
#[proc_macro]
pub fn rickroll(item: TokenStream) -> TokenStream {
    rr();
    item
}
