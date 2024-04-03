use crate::tts::tts;

#[cfg(test)]
mod tts_test {
    use crate::tts::tts;

    #[cfg(test)]
    fn test_tts() {
        tts("你好世界".to_string())
    }
}
