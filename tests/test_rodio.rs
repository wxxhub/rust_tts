
#[cfg(test)]
mod tests {
    use rodio::{Decoder, OutputStream, Sink, Source};
    use std::io::Cursor;

    use std::collections::HashMap;
    use std::ptr::replace;


    #[test]
    fn test_rodio()-> Result<(), rodio::PlayError> {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle)?;

        let source = rodio::Decoder::new(Cursor::new(include_bytes!("../src/tts/wav/a.wav").to_vec())).unwrap();
        sink.append(source);

        let source = rodio::Decoder::new(Cursor::new(include_bytes!("../src/tts/wav/ao1.wav").to_vec())).unwrap();
        sink.append(source);
        sink.sleep_until_end();

        Ok(())
    }

    #[test]
    fn test_rodio2()-> Result<(), rodio::PlayError> {
        let mut m:HashMap<&str, Decoder<Cursor<Vec<u8>>>> = HashMap::new();

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle)?;

        let source = rodio::Decoder::new(Cursor::new(include_bytes!("../src/tts/wav/a.wav").to_vec())).unwrap();
        m.insert("a", source);
        let source = rodio::Decoder::new(Cursor::new(include_bytes!("../src/tts/wav/ao1.wav").to_vec())).unwrap();
        m.insert("ao", source);


        // if let Some(values) = m.get("a") {
        //
        //     sink.append(values.clone());
        // }
        //
        // if let Some(values) = m.get("ao") {
        //     let v = values.clone();
        //     sink.append(v);
        // }

        sink.sleep_until_end();

        Ok(())
    }


    #[test]
    fn test_rodio3()-> Result<(), rodio::PlayError> {
        let mut m = HashMap::new();

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle)?;


        m.insert("a", include_bytes!("../src/tts/wav/a.wav").to_vec());
        m.insert("ao", include_bytes!("../src/tts/wav/ao1.wav").to_vec());


        if let Some(values) = m.get("a") {

            sink.append(Decoder::new(Cursor::new(values.clone())).unwrap());
        }

        if let Some(values) = m.get("ao") {
            sink.append(Decoder::new(Cursor::new(values.clone())).unwrap());
        }

        sink.sleep_until_end();

        Ok(())
    }

    #[test]
    fn test_rodio4()-> Result<(), rodio::PlayError> {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle)?;

        let source = rodio::Decoder::new(Cursor::new(include_bytes!("../src/tts/wav/a.wav").to_vec())).unwrap(););

        let source = rodio::Decoder::new(Cursor::new(include_bytes!("../src/tts/wav/ao1.wav").to_vec())).unwrap();
        sink.append(source);
        sink.sleep_until_end();

        Ok(())
    }


    // use std::collections::HashMap;

    #[test]
    fn tes_tmap() {
        let mut map: HashMap<String, Vec<i32>> = HashMap::new();

        let slice = [1, 2, 3, 4, 5];
        let vec: Vec<i32> = slice.to_vec(); // 将切片转换为Vec
        map.insert("key".to_string(), vec); // 将Vec存储到HashMap中

        let slice2 = [1, 2, 3, 4, 5, 6];
        let vec2: Vec<i32> = slice2.to_vec(); // 将切片转换为Vec
        map.insert("key2".to_string(), vec2); // 将Vec存储到HashMap中

        if let Some(values) = map.get("key") {
            println!("{:?}", values); // 访问HashMap中的Vec
        }

        if let Some(values) = map.get("key2") {
            println!("{:?}", values); // 访问HashMap中的Vec
        }
    }
}

