mod tts;
mod to_pinyin;

use std::io::Cursor;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::collections::HashMap;


fn main()-> Result<(), rodio::PlayError>  {


    let (_stream, stream_handle) = OutputStream::try_default().unwrap();


    let sink = Sink::try_new(&stream_handle)?;

    sink.append(tts::get_pronounce("ni1").unwrap());
    sink.append(tts::get_pronounce("hao2").unwrap());
    sink.append(tts::get_pronounce("a").unwrap());
    sink.sleep_until_end();

    Ok(())
}