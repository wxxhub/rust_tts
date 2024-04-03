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

fn pinyin_to_tone_numbers(input: String) -> String {
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
    }

    if let Some(tone) = output_tone {
        output.push(tone);
    }
    output
}


pub(crate) fn to_pinyin(content: String) -> Vec<String> {
    let content = "汉语拼音测试，音频重置重要".to_string();

    let pt = pinyin_translator::PinyinTranslator::new();
    let list = pt.translate_as_slice(content.clone());

    list.iter().map(|x|pinyin_to_tone_numbers(x.clone())).collect()
}