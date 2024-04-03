#[cfg(test)]
mod tests {
    // use pinyin::{*};

    #[test]
    fn test_pinyin2() {
        let content = "汉语拼音测试，频重置重要".to_string();

        let pt = pinyin_translator::PinyinTranslator::new();
        let result = pt.translate(content.clone());
        println!("{}", result);

        let vec = pt.translate_as_slice(content.clone());
        println!("{:?}", vec);

       println!("{:?}", pt.translate_as_slice(content.clone()));
    }

    const TONE_NUMS: &[char] = &['0', '1', '2', '3', '4'];
    const PHONETIC_SYMBOL_MAP: &[(char, char, u8)] = &[
        ('ā', 'a', 1),
        ('á', 'a', 2),
        ('ǎ', 'a', 3),
        ('à', 'a', 4),
        ('ē', 'e', 1),
        ('é', 'e', 2),
        ('ě', 'e', 3),
        ('è', 'e', 4),
        ('ế', 'ê', 2),
        ('ề', 'ê', 4),
        ('ō', 'o', 1),
        ('ó', 'o', 2),
        ('ǒ', 'o', 3),
        ('ò', 'o', 4),
        ('ī', 'i', 1),
        ('í', 'i', 2),
        ('ǐ', 'i', 3),
        ('ì', 'i', 4),
        ('ū', 'u', 1),
        ('ú', 'u', 2),
        ('ǔ', 'u', 3),
        ('ù', 'u', 4),
        ('ǘ', 'ü', 2),
        ('ǚ', 'ü', 3),
        ('ǜ', 'ü', 4),
        ('ń', 'n', 2),
        ('ň', 'n', 3),
        ('ǹ', 'n', 4),
        ('ḿ', 'm', 2),
    ];

    fn pinyin_to_tone_numbers(input: &str) -> String {
        let mut output = String::new();
        let mut output_tone = None;

        for ch in input.chars() {
            let (ch, tone) = get_char_info(ch);
            if let Some(ch) = ch {
                output.push(ch)
            }

            if tone > 0 {
                output_tone = Some(TONE_NUMS[usize::try_from(tone).unwrap()]);
            }
            println!("{:?} {}", ch, tone)
        }

        if let Some(tone) = output_tone {
            output.push(tone);
        }
        output
    }

    fn get_char_info(ch: char) -> (Option<char>, u8) {
        if let Some((_, base, tone)) = PHONETIC_SYMBOL_MAP.iter().find(|(c, _, _)| *c == ch) {
            return (Some(*base), *tone);
        }
        const TONE_MAP: &[(char, u8)] = &[('\u{304}', 1), ('\u{30c}', 3), ('\u{300}', 4)];
        if let Some((_, tone)) = TONE_MAP.iter().find(|(c, _)| *c == ch) {
            return (None, *tone);
        }
        (Some(ch), 0)
    }

    #[test]
    fn test_tone() {
        let list = ["hàn", "yǔ", "pīn", "yīn", "cè", "shì", "，", "pín", "chóng", "zhì", "zhòng", "yào", "zhìyìwǔ"];

        for l  in list {
            println!("{}", pinyin_to_tone_numbers(l))
        }
    }

    use std::fs;
    use std::io;
    use std::path::Path;
    #[test]
    fn test_gen_map() -> io::Result<()> {
        let dir_path = "/Users/wxx/RustroverProjects/rust_tts/src/tts/wav"; // 设置你想要遍历的目录路径
        let entries = fs::read_dir(dir_path)?;

        let mut filenames = Vec::new();

        // 遍历目录项
        for entry in entries {
            let entry = entry?;
            let name = entry.file_name().into_string().unwrap();

            filenames.push(name);



        }

        filenames.sort_unstable();

        // 打印排序后的文件名
        for filename in &filenames {
            let left = filename.split(".").next().unwrap().to_string();
            println!(" m.insert(\"{}\".to_string(), include_bytes!(\"wav/{}\").to_vec());", left, filename);
        }

        Ok(())
    }

}