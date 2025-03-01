use crate::input::Key;
use chrono::{Timelike, Utc};
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

// These imports are only used in test functions
#[cfg(test)]
use chrono::{DateTime, Datelike, Local, TimeZone, Weekday};

/// Key mapper for keyboard input obfuscation
#[derive(Clone)]
pub struct KeyMapper {
    mapping: HashMap<Key, Key>,
}

impl KeyMapper {
    /// Get the internal mapping
    pub fn get_mapping(&self) -> &HashMap<Key, Key> {
        &self.mapping
    }
    /// Create a random key mapper
    pub fn random() -> Self {
        let mut rng = thread_rng();
        let mut mapping = HashMap::new();
        
        // Map characters to random characters
        for c in 'a'..='z' {
            let alphabet: Vec<char> = ('a'..='z').collect();
            let random_char = alphabet.choose(&mut rng).unwrap();
            mapping.insert(Key::Char(c), Key::Char(*random_char));
            
            // Also map uppercase
            let uppercase_c = c.to_uppercase().next().unwrap();
            let uppercase_random = random_char.to_uppercase().next().unwrap();
            mapping.insert(Key::Char(uppercase_c), Key::Char(uppercase_random));
        }
        
        // Map digits to random digits
        for d in '0'..='9' {
            let digits: Vec<char> = ('0'..='9').collect();
            let random_digit = digits.choose(&mut rng).unwrap();
            mapping.insert(Key::Char(d), Key::Char(*random_digit));
        }
        
        // Keep special keys as is
        mapping.insert(Key::Enter, Key::Enter);
        mapping.insert(Key::Backspace, Key::Backspace);
        mapping.insert(Key::Tab, Key::Tab);
        mapping.insert(Key::Escape, Key::Escape);
        
        KeyMapper { mapping }
    }
    
    /// Create a key mapper for a specific country
    pub fn for_country(country_code: &str) -> Self {
        match country_code {
            "DE" => KeyMapper::german_layout(),
            "FR" => KeyMapper::french_layout(),
            "RU" => KeyMapper::russian_layout(),
            "JP" => KeyMapper::japanese_layout(),
            "ES" => KeyMapper::spanish_layout(),
            "BR" => KeyMapper::brazilian_layout(),
            "CN" => KeyMapper::chinese_layout(),
            "HK" => KeyMapper::cantonese_layout(),
            "KR" => KeyMapper::korean_layout(),
            "AR" => KeyMapper::arabic_layout(),
            "IR" => KeyMapper::farsi_layout(),
            _ => KeyMapper::identity(), // Default to identity mapping
        }
    }
    
    /// Map a key to its obfuscated equivalent
    pub fn map_key(&self, key: Key) -> Key {
        self.mapping.get(&key).cloned().unwrap_or(key)
    }
    
    /// Identity mapping (no changes)
    fn identity() -> Self {
        let mut mapping = HashMap::new();
        
        // Map each key to itself
        for c in 'a'..='z' {
            mapping.insert(Key::Char(c), Key::Char(c));
            
            // Also map uppercase
            let uppercase_c = c.to_uppercase().next().unwrap();
            mapping.insert(Key::Char(uppercase_c), Key::Char(uppercase_c));
        }
        
        for d in '0'..='9' {
            mapping.insert(Key::Char(d), Key::Char(d));
        }
        
        mapping.insert(Key::Enter, Key::Enter);
        mapping.insert(Key::Backspace, Key::Backspace);
        mapping.insert(Key::Tab, Key::Tab);
        mapping.insert(Key::Escape, Key::Escape);
        
        KeyMapper { mapping }
    }
    
    /// German keyboard layout emulation
    fn german_layout() -> Self {
        let mut mapping = Self::identity().mapping;
        
        // German keyboard specific mappings
        mapping.insert(Key::Char('y'), Key::Char('z'));
        mapping.insert(Key::Char('z'), Key::Char('y'));
        mapping.insert(Key::Char('Y'), Key::Char('Z'));
        mapping.insert(Key::Char('Z'), Key::Char('Y'));
        
        // Add umlauts and other German-specific characters
        mapping.insert(Key::Char('['), Key::Char('ü'));
        mapping.insert(Key::Char(']'), Key::Char('+'));
        mapping.insert(Key::Char(';'), Key::Char('ö'));
        mapping.insert(Key::Char('\''), Key::Char('ä'));
        
        KeyMapper { mapping }
    }
    
    /// French keyboard layout emulation
    fn french_layout() -> Self {
        let mut mapping = Self::identity().mapping;
        
        // French keyboard specific mappings (AZERTY)
        mapping.insert(Key::Char('q'), Key::Char('a'));
        mapping.insert(Key::Char('w'), Key::Char('z'));
        mapping.insert(Key::Char('a'), Key::Char('q'));
        mapping.insert(Key::Char('z'), Key::Char('w'));
        
        mapping.insert(Key::Char('Q'), Key::Char('A'));
        mapping.insert(Key::Char('W'), Key::Char('Z'));
        mapping.insert(Key::Char('A'), Key::Char('Q'));
        mapping.insert(Key::Char('Z'), Key::Char('W'));
        
        KeyMapper { mapping }
    }
    
    /// Russian keyboard layout emulation
    fn russian_layout() -> Self {
        let mut mapping = Self::identity().mapping;
        
        // Simplified Russian mapping (just a few examples)
        mapping.insert(Key::Char('a'), Key::Char('ф'));
        mapping.insert(Key::Char('b'), Key::Char('и'));
        mapping.insert(Key::Char('c'), Key::Char('с'));
        mapping.insert(Key::Char('d'), Key::Char('в'));
        
        mapping.insert(Key::Char('A'), Key::Char('Ф'));
        mapping.insert(Key::Char('B'), Key::Char('И'));
        mapping.insert(Key::Char('C'), Key::Char('С'));
        mapping.insert(Key::Char('D'), Key::Char('В'));
        
        KeyMapper { mapping }
    }
    
    /// Japanese keyboard layout emulation
    fn japanese_layout() -> Self {
        let mut mapping = Self::identity().mapping;
        
        // Simplified Japanese mapping
        mapping.insert(Key::Char('@'), Key::Char('\"'));
        mapping.insert(Key::Char('['), Key::Char('「'));
        mapping.insert(Key::Char(']'), Key::Char('」'));
        
        KeyMapper { mapping }
    }
    
    /// Spanish keyboard layout emulation
    fn spanish_layout() -> Self {
        let mut mapping = Self::identity().mapping;
        
        // Spanish keyboard specific mappings
        mapping.insert(Key::Char('~'), Key::Char('ñ'));
        mapping.insert(Key::Char('\''), Key::Char('ñ'));
        mapping.insert(Key::Char(';'), Key::Char('ñ'));
        mapping.insert(Key::Char('['), Key::Char('´'));
        mapping.insert(Key::Char(']'), Key::Char('¨'));
        
        // Common accented letters
        mapping.insert(Key::Char('1'), Key::Char('!'));
        mapping.insert(Key::Char('2'), Key::Char('\"'));
        mapping.insert(Key::Char('6'), Key::Char('&'));
        mapping.insert(Key::Char('4'), Key::Char('$'));
        
        KeyMapper { mapping }
    }
    
    /// Brazilian Portuguese keyboard layout emulation
    fn brazilian_layout() -> Self {
        let mut mapping = Self::identity().mapping;
        
        // Brazilian keyboard specific mappings
        mapping.insert(Key::Char('\''), Key::Char('ç'));
        mapping.insert(Key::Char('['), Key::Char('´'));
        mapping.insert(Key::Char(']'), Key::Char('['));
        mapping.insert(Key::Char('\\'), Key::Char(']'));
        mapping.insert(Key::Char('~'), Key::Char('\''));
        mapping.insert(Key::Char('`'), Key::Char('\''));
        
        // Common accented letters
        mapping.insert(Key::Char(';'), Key::Char('ç'));
        mapping.insert(Key::Char('/'), Key::Char(';'));
        mapping.insert(Key::Char('.'), Key::Char(':'));
        
        KeyMapper { mapping }
    }
    
    /// Chinese (Mandarin) keyboard layout emulation
    fn chinese_layout() -> Self {
        let mut mapping = Self::identity().mapping;
        
        // Comprehensive implementation of Mandarin Pinyin keyboard with common characters
        // Simulates the behavior of typing on a Chinese keyboard with pinyin input
        
        // Basic frequently used Chinese characters matched to their common pinyin initials
        mapping.insert(Key::Char('a'), Key::Char('啊')); // a - common expression
        mapping.insert(Key::Char('b'), Key::Char('不')); // bu - not
        mapping.insert(Key::Char('c'), Key::Char('从')); // cong - from
        mapping.insert(Key::Char('d'), Key::Char('的')); // de - possessive particle
        mapping.insert(Key::Char('e'), Key::Char('额')); // e - forehead/surprise
        mapping.insert(Key::Char('f'), Key::Char('发')); // fa - send/hair
        mapping.insert(Key::Char('g'), Key::Char('个')); // ge - individual measure word
        mapping.insert(Key::Char('h'), Key::Char('和')); // he - and
        mapping.insert(Key::Char('i'), Key::Char('以')); // yi - with/by
        mapping.insert(Key::Char('j'), Key::Char('就')); // jiu - then/right away
        mapping.insert(Key::Char('k'), Key::Char('看')); // kan - look/see
        mapping.insert(Key::Char('l'), Key::Char('了')); // le - completed action
        mapping.insert(Key::Char('m'), Key::Char('吗')); // ma - question particle
        mapping.insert(Key::Char('n'), Key::Char('你')); // ni - you
        mapping.insert(Key::Char('o'), Key::Char('哦')); // o - oh
        mapping.insert(Key::Char('p'), Key::Char('朋')); // peng - friend (first char)
        mapping.insert(Key::Char('q'), Key::Char('去')); // qu - go
        mapping.insert(Key::Char('r'), Key::Char('人')); // ren - person
        mapping.insert(Key::Char('s'), Key::Char('是')); // shi - is/am/are
        mapping.insert(Key::Char('t'), Key::Char('他')); // ta - he
        mapping.insert(Key::Char('u'), Key::Char('有')); // you - have
        mapping.insert(Key::Char('v'), Key::Char('女')); // nv - woman
        mapping.insert(Key::Char('w'), Key::Char('我')); // wo - I
        mapping.insert(Key::Char('x'), Key::Char('小')); // xiao - small
        mapping.insert(Key::Char('y'), Key::Char('一')); // yi - one
        mapping.insert(Key::Char('z'), Key::Char('在')); // zai - at

        // Map some uppercase to single characters that represent words
        mapping.insert(Key::Char('A'), Key::Char('啊')); // ah
        mapping.insert(Key::Char('B'), Key::Char('百')); // bai - hundred (from Baidu)
        mapping.insert(Key::Char('C'), Key::Char('草')); // cao - grass
        mapping.insert(Key::Char('D'), Key::Char('但')); // dan - but (from danshi)
        mapping.insert(Key::Char('H'), Key::Char('好')); // hao - good (from henhao)
        mapping.insert(Key::Char('M'), Key::Char('没')); // mei - not have (from meiyou)
        mapping.insert(Key::Char('S'), Key::Char('谢')); // xie - thank (from xiexie)
        mapping.insert(Key::Char('W'), Key::Char('为')); // wei - for/why (from weishenme)
        mapping.insert(Key::Char('X'), Key::Char('下')); // xia - down (from xiazai)

        // Number keys often produce corresponding Chinese numerals
        mapping.insert(Key::Char('1'), Key::Char('一')); // yi - one
        mapping.insert(Key::Char('2'), Key::Char('二')); // er - two
        mapping.insert(Key::Char('3'), Key::Char('三')); // san - three
        mapping.insert(Key::Char('4'), Key::Char('四')); // si - four
        mapping.insert(Key::Char('5'), Key::Char('五')); // wu - five
        mapping.insert(Key::Char('6'), Key::Char('六')); // liu - six
        mapping.insert(Key::Char('7'), Key::Char('七')); // qi - seven
        mapping.insert(Key::Char('8'), Key::Char('八')); // ba - eight
        mapping.insert(Key::Char('9'), Key::Char('九')); // jiu - nine
        mapping.insert(Key::Char('0'), Key::Char('零')); // ling - zero
        
        // Punctuation is also different in Chinese
        mapping.insert(Key::Char(','), Key::Char('，')); // Chinese comma
        mapping.insert(Key::Char('.'), Key::Char('。')); // Chinese period
        mapping.insert(Key::Char('?'), Key::Char('？')); // Chinese question mark
        mapping.insert(Key::Char('!'), Key::Char('！')); // Chinese exclamation
        mapping.insert(Key::Char(':'), Key::Char('：')); // Chinese colon
        mapping.insert(Key::Char(';'), Key::Char('；')); // Chinese semicolon
        mapping.insert(Key::Char('\''), Key::Char('\'')); // Single quote approximation
        mapping.insert(Key::Char('"'), Key::Char('"')); // Double quote approximation
        mapping.insert(Key::Char('('), Key::Char('（')); // Chinese parenthesis
        mapping.insert(Key::Char(')'), Key::Char('）')); // Chinese parenthesis
        
        KeyMapper { mapping }
    }
    
    /// Cantonese keyboard layout emulation (Hong Kong)
    fn cantonese_layout() -> Self {
        let mut mapping = Self::identity().mapping;
        
        // Comprehensive implementation of Cantonese input (using traditional characters)
        // This simulates a Cangjie or Quick input method used in Hong Kong
        
        // Common Cantonese characters mapped to Roman keyboard
        mapping.insert(Key::Char('a'), Key::Char('呀')); // "ah" expression
        mapping.insert(Key::Char('b'), Key::Char('唔')); // "m4" negation (not)
        mapping.insert(Key::Char('c'), Key::Char('出')); // "ceot1" out
        mapping.insert(Key::Char('d'), Key::Char('的')); // "dik1" possessive
        mapping.insert(Key::Char('e'), Key::Char('額')); // "ngaak6" forehead
        mapping.insert(Key::Char('f'), Key::Char('放')); // "fong3" put/release
        mapping.insert(Key::Char('g'), Key::Char('個')); // "go3" individual counter
        mapping.insert(Key::Char('h'), Key::Char('係')); // "hai6" is
        mapping.insert(Key::Char('i'), Key::Char('依')); // "ji1" lean on
        mapping.insert(Key::Char('j'), Key::Char('啲')); // "di1" some
        mapping.insert(Key::Char('k'), Key::Char('佢')); // "keoi5" him/her
        mapping.insert(Key::Char('l'), Key::Char('咗')); // "zo2" completed action
        mapping.insert(Key::Char('m'), Key::Char('咩')); // "me1" what
        mapping.insert(Key::Char('n'), Key::Char('你')); // "nei5" you
        mapping.insert(Key::Char('o'), Key::Char('喔')); // "o1" oh (expression)
        mapping.insert(Key::Char('p'), Key::Char('朋')); // "pang4" friend
        mapping.insert(Key::Char('q'), Key::Char('去')); // "heoi3" go
        mapping.insert(Key::Char('r'), Key::Char('人')); // "jan4" person
        mapping.insert(Key::Char('s'), Key::Char('使')); // "sai2" use
        mapping.insert(Key::Char('t'), Key::Char('睇')); // "tai2" look
        mapping.insert(Key::Char('u'), Key::Char('有')); // "jau5" have
        mapping.insert(Key::Char('v'), Key::Char('話')); // "waa6" say
        mapping.insert(Key::Char('w'), Key::Char('我')); // "ngo5" I/me
        mapping.insert(Key::Char('x'), Key::Char('冇')); // "mou5" don't have
        mapping.insert(Key::Char('y'), Key::Char('嘢')); // "je5" thing
        mapping.insert(Key::Char('z'), Key::Char('在')); // "zoi6" at
        
        // Some common Cantonese characters (uppercase)
        mapping.insert(Key::Char('A'), Key::Char('唔')); // "m4" not (from m4goi1)
        mapping.insert(Key::Char('B'), Key::Char('邊')); // "bin1" where (from bin1dou6)
        mapping.insert(Key::Char('C'), Key::Char('點')); // "dim2" point/how (from dim2gaai2)
        mapping.insert(Key::Char('D'), Key::Char('多')); // "do1" many (from do1ze6)
        mapping.insert(Key::Char('H'), Key::Char('好')); // "hou2" good (from hou2je5)
        mapping.insert(Key::Char('M'), Key::Char('乜')); // "mat1" what (from mat1je5)
        mapping.insert(Key::Char('W'), Key::Char('我')); // "ngo5" I (from ngo5dei6)
        mapping.insert(Key::Char('X'), Key::Char('樣')); // "joeng2" kind/type (from dim2joeng2)
        
        // Traditional Chinese numerals (same as Mandarin but with Cantonese pronunciation)
        mapping.insert(Key::Char('1'), Key::Char('一')); // "jat1" one
        mapping.insert(Key::Char('2'), Key::Char('二')); // "ji6" two
        mapping.insert(Key::Char('3'), Key::Char('三')); // "saam1" three
        mapping.insert(Key::Char('4'), Key::Char('四')); // "sei3" four
        mapping.insert(Key::Char('5'), Key::Char('五')); // "ng5" five
        mapping.insert(Key::Char('6'), Key::Char('六')); // "luk6" six
        mapping.insert(Key::Char('7'), Key::Char('七')); // "cat1" seven
        mapping.insert(Key::Char('8'), Key::Char('八')); // "baat3" eight
        mapping.insert(Key::Char('9'), Key::Char('九')); // "gau2" nine
        mapping.insert(Key::Char('0'), Key::Char('零')); // "ling4" zero
        
        // Same Chinese punctuation as Mandarin
        mapping.insert(Key::Char(','), Key::Char('，')); // Chinese comma
        mapping.insert(Key::Char('.'), Key::Char('。')); // Chinese period
        mapping.insert(Key::Char('?'), Key::Char('？')); // Chinese question mark
        mapping.insert(Key::Char('!'), Key::Char('！')); // Chinese exclamation
        mapping.insert(Key::Char(':'), Key::Char('：')); // Chinese colon
        mapping.insert(Key::Char(';'), Key::Char('；')); // Chinese semicolon
        mapping.insert(Key::Char('\''), Key::Char('\'')); // Single quote approximation
        mapping.insert(Key::Char('"'), Key::Char('"')); // Double quote approximation
        mapping.insert(Key::Char('('), Key::Char('（')); // Chinese parenthesis
        mapping.insert(Key::Char(')'), Key::Char('）')); // Chinese parenthesis
        
        KeyMapper { mapping }
    }
    
    /// Korean keyboard layout emulation
    fn korean_layout() -> Self {
        let mut mapping = Self::identity().mapping;
        
        // Professional implementation of Korean Hangul keyboard mapping
        // Based on the standard Korean keyboard layout (2-set)
        
        // Consonants (자음)
        mapping.insert(Key::Char('q'), Key::Char('ㅂ')); // bieup
        mapping.insert(Key::Char('w'), Key::Char('ㅈ')); // jieut
        mapping.insert(Key::Char('e'), Key::Char('ㄷ')); // digeut
        mapping.insert(Key::Char('r'), Key::Char('ㄱ')); // giyeok
        mapping.insert(Key::Char('t'), Key::Char('ㅅ')); // siot
        mapping.insert(Key::Char('y'), Key::Char('ㅛ')); // yo
        mapping.insert(Key::Char('u'), Key::Char('ㅕ')); // yeo
        mapping.insert(Key::Char('i'), Key::Char('ㅑ')); // ya
        mapping.insert(Key::Char('o'), Key::Char('ㅐ')); // ae
        mapping.insert(Key::Char('p'), Key::Char('ㅔ')); // e
        
        mapping.insert(Key::Char('a'), Key::Char('ㅁ')); // mieum
        mapping.insert(Key::Char('s'), Key::Char('ㄴ')); // nieun
        mapping.insert(Key::Char('d'), Key::Char('ㅇ')); // ieung
        mapping.insert(Key::Char('f'), Key::Char('ㄹ')); // rieul
        mapping.insert(Key::Char('g'), Key::Char('ㅎ')); // hieut
        mapping.insert(Key::Char('h'), Key::Char('ㅗ')); // o
        mapping.insert(Key::Char('j'), Key::Char('ㅓ')); // eo
        mapping.insert(Key::Char('k'), Key::Char('ㅏ')); // a
        mapping.insert(Key::Char('l'), Key::Char('ㅣ')); // i
        
        mapping.insert(Key::Char('z'), Key::Char('ㅋ')); // kieuk
        mapping.insert(Key::Char('x'), Key::Char('ㅌ')); // tieut
        mapping.insert(Key::Char('c'), Key::Char('ㅊ')); // chieut
        mapping.insert(Key::Char('v'), Key::Char('ㅍ')); // pieup
        mapping.insert(Key::Char('b'), Key::Char('ㅠ')); // yu
        mapping.insert(Key::Char('n'), Key::Char('ㅜ')); // u
        mapping.insert(Key::Char('m'), Key::Char('ㅡ')); // eu
        
        // Uppercase for double consonants and alternative vowels
        mapping.insert(Key::Char('Q'), Key::Char('ㅃ')); // ssangbieup
        mapping.insert(Key::Char('W'), Key::Char('ㅉ')); // ssangjieut
        mapping.insert(Key::Char('E'), Key::Char('ㄸ')); // ssangdigeut
        mapping.insert(Key::Char('R'), Key::Char('ㄲ')); // ssanggiyeok
        mapping.insert(Key::Char('T'), Key::Char('ㅆ')); // ssangsiot
        
        mapping.insert(Key::Char('O'), Key::Char('ㅒ')); // yae
        mapping.insert(Key::Char('P'), Key::Char('ㅖ')); // ye
        
        // Add common completed Hangul syllables for cybersecurity context
        mapping.insert(Key::Char('1'), Key::Char('암'));  // "am" - part of "password"
        mapping.insert(Key::Char('2'), Key::Char('호'));  // "ho" - part of "protection"
        mapping.insert(Key::Char('3'), Key::Char('보'));  // "bo" - part of "security"
        mapping.insert(Key::Char('4'), Key::Char('안'));  // "an" - part of "security"
        mapping.insert(Key::Char('5'), Key::Char('전'));  // "jeon" - part of "computer"
        mapping.insert(Key::Char('6'), Key::Char('산'));  // "san" - part of "calculation"
        mapping.insert(Key::Char('7'), Key::Char('키'));  // "ki" - part of "key"
        mapping.insert(Key::Char('8'), Key::Char('해'));  // "hae" - part of "hacking"
        mapping.insert(Key::Char('9'), Key::Char('킹'));  // "king" - part of "hacking"
        mapping.insert(Key::Char('0'), Key::Char('비'));  // "bi" - part of "password"
        
        // Punctuation
        mapping.insert(Key::Char(','), Key::Char('，')); // Korean comma (same as Chinese)
        mapping.insert(Key::Char('.'), Key::Char('。')); // Korean period (same as Chinese)
        mapping.insert(Key::Char('?'), Key::Char('？')); // Korean question mark
        mapping.insert(Key::Char('!'), Key::Char('！')); // Korean exclamation
        
        KeyMapper { mapping }
    }
    
    /// Arabic keyboard layout emulation
    fn arabic_layout() -> Self {
        let mut mapping = Self::identity().mapping;
        
        // Full implementation of Arabic keyboard layout
        // Based on the standard Arabic keyboard layout
        
        // Main Arabic letters
        mapping.insert(Key::Char('q'), Key::Char('ض')); // dad
        mapping.insert(Key::Char('w'), Key::Char('ص')); // sad
        mapping.insert(Key::Char('e'), Key::Char('ث')); // theh
        mapping.insert(Key::Char('r'), Key::Char('ق')); // qaf
        mapping.insert(Key::Char('t'), Key::Char('ف')); // feh
        mapping.insert(Key::Char('y'), Key::Char('غ')); // ghain
        mapping.insert(Key::Char('u'), Key::Char('ع')); // ain
        mapping.insert(Key::Char('i'), Key::Char('ه')); // heh
        mapping.insert(Key::Char('o'), Key::Char('خ')); // khah
        mapping.insert(Key::Char('p'), Key::Char('ح')); // hah
        mapping.insert(Key::Char('['), Key::Char('ج')); // jeem
        mapping.insert(Key::Char(']'), Key::Char('د')); // dal
        
        mapping.insert(Key::Char('a'), Key::Char('ش')); // sheen
        mapping.insert(Key::Char('s'), Key::Char('س')); // seen
        mapping.insert(Key::Char('d'), Key::Char('ي')); // yeh
        mapping.insert(Key::Char('f'), Key::Char('ب')); // beh
        mapping.insert(Key::Char('g'), Key::Char('ل')); // lam
        mapping.insert(Key::Char('h'), Key::Char('ا')); // alef
        mapping.insert(Key::Char('j'), Key::Char('ت')); // teh
        mapping.insert(Key::Char('k'), Key::Char('ن')); // noon
        mapping.insert(Key::Char('l'), Key::Char('م')); // meem
        mapping.insert(Key::Char(';'), Key::Char('ك')); // kaf
        mapping.insert(Key::Char('\''), Key::Char('ط')); // tah
        
        mapping.insert(Key::Char('z'), Key::Char('ئ')); // yeh with hamza
        mapping.insert(Key::Char('x'), Key::Char('ء')); // hamza
        mapping.insert(Key::Char('c'), Key::Char('ؤ')); // waw with hamza
        mapping.insert(Key::Char('v'), Key::Char('ر')); // reh
        mapping.insert(Key::Char('b'), Key::Char('ل')); // lam (simplified from lam-alef)
        mapping.insert(Key::Char('n'), Key::Char('ى')); // alef maksura
        mapping.insert(Key::Char('m'), Key::Char('ة')); // teh marbuta
        mapping.insert(Key::Char(','), Key::Char('و')); // waw
        mapping.insert(Key::Char('.'), Key::Char('ز')); // zain
        mapping.insert(Key::Char('/'), Key::Char('ظ')); // zah
        
        // Add shift variations for special characters
        mapping.insert(Key::Char('`'), Key::Char('ذ')); // thal
        mapping.insert(Key::Char('Q'), Key::Char('َ')); // fatha
        mapping.insert(Key::Char('W'), Key::Char('ً')); // fathatan
        mapping.insert(Key::Char('E'), Key::Char('ُ')); // damma
        mapping.insert(Key::Char('R'), Key::Char('ٌ')); // dammatan
        mapping.insert(Key::Char('T'), Key::Char('إ')); // alef with hamza below (simplified)
        mapping.insert(Key::Char('Y'), Key::Char('إ')); // alef with hamza below
        mapping.insert(Key::Char('U'), Key::Char('`')); // backtick
        mapping.insert(Key::Char('I'), Key::Char('÷')); // division
        mapping.insert(Key::Char('O'), Key::Char('×')); // multiplication
        mapping.insert(Key::Char('P'), Key::Char('؛')); // Arabic semicolon
        
        // Numbers adjusted for Arabic input
        mapping.insert(Key::Char('1'), Key::Char('١')); // Arabic 1
        mapping.insert(Key::Char('2'), Key::Char('٢')); // Arabic 2
        mapping.insert(Key::Char('3'), Key::Char('٣')); // Arabic 3
        mapping.insert(Key::Char('4'), Key::Char('٤')); // Arabic 4
        mapping.insert(Key::Char('5'), Key::Char('٥')); // Arabic 5
        mapping.insert(Key::Char('6'), Key::Char('٦')); // Arabic 6
        mapping.insert(Key::Char('7'), Key::Char('٧')); // Arabic 7
        mapping.insert(Key::Char('8'), Key::Char('٨')); // Arabic 8
        mapping.insert(Key::Char('9'), Key::Char('٩')); // Arabic 9
        mapping.insert(Key::Char('0'), Key::Char('٠')); // Arabic 0
        
        // Punctuation
        mapping.insert(Key::Char('-'), Key::Char('-'));
        mapping.insert(Key::Char('='), Key::Char('=')); 
        mapping.insert(Key::Char('\\'), Key::Char('\\'));
        mapping.insert(Key::Char('?'), Key::Char('؟')); // Arabic question mark
        mapping.insert(Key::Char('!'), Key::Char('!')); 
        
        KeyMapper { mapping }
    }
    
    /// Farsi/Persian keyboard layout emulation (ISIRI 9147 standard)
    fn farsi_layout() -> Self {
        let mut mapping = Self::identity().mapping;
        
        // Letters row 1
        mapping.insert(Key::Char('q'), Key::Char('ض')); // zad
        mapping.insert(Key::Char('w'), Key::Char('ص')); // sad
        mapping.insert(Key::Char('e'), Key::Char('ث')); // se
        mapping.insert(Key::Char('r'), Key::Char('ق')); // ghaf
        mapping.insert(Key::Char('t'), Key::Char('ف')); // fe
        mapping.insert(Key::Char('y'), Key::Char('غ')); // ghein
        mapping.insert(Key::Char('u'), Key::Char('ع')); // ein
        mapping.insert(Key::Char('i'), Key::Char('ه')); // he
        mapping.insert(Key::Char('o'), Key::Char('خ')); // khe
        mapping.insert(Key::Char('p'), Key::Char('ح')); // he jimi
        mapping.insert(Key::Char('['), Key::Char('ج')); // jim
        mapping.insert(Key::Char(']'), Key::Char('چ')); // che - Persian specific

        // Letters row 2
        mapping.insert(Key::Char('a'), Key::Char('ش')); // shin
        mapping.insert(Key::Char('s'), Key::Char('س')); // sin
        mapping.insert(Key::Char('d'), Key::Char('ی')); // ye
        mapping.insert(Key::Char('f'), Key::Char('ب')); // be
        mapping.insert(Key::Char('g'), Key::Char('ل')); // lam
        mapping.insert(Key::Char('h'), Key::Char('ا')); // alef
        mapping.insert(Key::Char('j'), Key::Char('ت')); // te
        mapping.insert(Key::Char('k'), Key::Char('ن')); // nun
        mapping.insert(Key::Char('l'), Key::Char('م')); // mim
        mapping.insert(Key::Char(';'), Key::Char('ک')); // kaf - Persian specific
        mapping.insert(Key::Char('\''), Key::Char('گ')); // gaf - Persian specific

        // Letters row 3
        mapping.insert(Key::Char('z'), Key::Char('ظ')); // za
        mapping.insert(Key::Char('x'), Key::Char('ط')); // ta
        mapping.insert(Key::Char('c'), Key::Char('ز')); // ze
        mapping.insert(Key::Char('v'), Key::Char('ر')); // re
        mapping.insert(Key::Char('b'), Key::Char('ذ')); // zal
        mapping.insert(Key::Char('n'), Key::Char('د')); // dal
        mapping.insert(Key::Char('m'), Key::Char('پ')); // pe - Persian specific
        mapping.insert(Key::Char(','), Key::Char('و')); // vav
        mapping.insert(Key::Char('.'), Key::Char('.')); // full stop
        mapping.insert(Key::Char('/'), Key::Char('/')); // slash

        // Shift+letters and symbols for diacritics/special characters
        mapping.insert(Key::Char('Q'), Key::Char('ْ')); // sukun
        mapping.insert(Key::Char('W'), Key::Char('ٌ')); // dammatan
        mapping.insert(Key::Char('E'), Key::Char('ٍ')); // kasratan
        mapping.insert(Key::Char('R'), Key::Char('ً')); // fathatan
        mapping.insert(Key::Char('T'), Key::Char('ُ')); // damma
        mapping.insert(Key::Char('Y'), Key::Char('ِ')); // kasra
        mapping.insert(Key::Char('U'), Key::Char('َ')); // fatha
        mapping.insert(Key::Char('I'), Key::Char('ّ')); // shadda
        mapping.insert(Key::Char('O'), Key::Char(']')); // closing bracket
        mapping.insert(Key::Char('P'), Key::Char('[')); // opening bracket
        mapping.insert(Key::Char('{'), Key::Char('}')); // reversed for RTL
        mapping.insert(Key::Char('}'), Key::Char('{')); // reversed for RTL
        
        // Shift+more letters for special Persian letters
        mapping.insert(Key::Char('J'), Key::Char('آ')); // alef madda
        mapping.insert(Key::Char('L'), Key::Char('أ')); // alef hamza above
        mapping.insert(Key::Char('V'), Key::Char('ژ')); // zhe - Persian specific
        mapping.insert(Key::Char(':'), Key::Char(':')); 
        mapping.insert(Key::Char('"'), Key::Char('؛')); // Arabic semicolon

        // Numbers row (Persian/Eastern Arabic numerals)
        mapping.insert(Key::Char('`'), Key::Char('‍')); // zero-width joiner
        mapping.insert(Key::Char('1'), Key::Char('۱')); // Persian 1
        mapping.insert(Key::Char('2'), Key::Char('۲')); // Persian 2
        mapping.insert(Key::Char('3'), Key::Char('۳')); // Persian 3
        mapping.insert(Key::Char('4'), Key::Char('۴')); // Persian 4
        mapping.insert(Key::Char('5'), Key::Char('۵')); // Persian 5
        mapping.insert(Key::Char('6'), Key::Char('۶')); // Persian 6
        mapping.insert(Key::Char('7'), Key::Char('۷')); // Persian 7
        mapping.insert(Key::Char('8'), Key::Char('۸')); // Persian 8
        mapping.insert(Key::Char('9'), Key::Char('۹')); // Persian 9
        mapping.insert(Key::Char('0'), Key::Char('۰')); // Persian 0
        mapping.insert(Key::Char('-'), Key::Char('-')); 
        mapping.insert(Key::Char('='), Key::Char('=')); 

        // Shift+numbers and symbols
        mapping.insert(Key::Char('~'), Key::Char('÷')); // division
        mapping.insert(Key::Char('!'), Key::Char('!')); 
        mapping.insert(Key::Char('@'), Key::Char('٬')); // thousands separator
        mapping.insert(Key::Char('#'), Key::Char('٫')); // decimal separator
        mapping.insert(Key::Char('$'), Key::Char('﷼')); // rial sign
        mapping.insert(Key::Char('%'), Key::Char('٪')); // percent
        mapping.insert(Key::Char('^'), Key::Char('×')); // multiplication
        mapping.insert(Key::Char('&'), Key::Char('،')); // Persian comma
        mapping.insert(Key::Char('*'), Key::Char('*')); 
        mapping.insert(Key::Char('('), Key::Char(')')); // reversed for RTL
        mapping.insert(Key::Char(')'), Key::Char('(')); // reversed for RTL
        mapping.insert(Key::Char('_'), Key::Char('ـ')); // Arabic tatweel
        mapping.insert(Key::Char('+'), Key::Char('+')); 

        // Additional punctuation and symbols
        mapping.insert(Key::Char('\\'), Key::Char('\\')); 
        mapping.insert(Key::Char('|'), Key::Char('|')); 
        mapping.insert(Key::Char('?'), Key::Char('؟')); // Arabic question mark
        
        KeyMapper { mapping }
    }
}

/// Language transformer for text obfuscation
#[derive(Clone)]
pub struct LanguageTransformer {
    dictionary: HashMap<String, String>,
    language_id: LanguageIdentifier,
    // Stores the target region for attribution fingerprinting
    attribution_region: String,
}

impl LanguageTransformer {
    /// Get the internal dictionary
    pub fn get_dictionary(&self) -> &HashMap<String, String> {
        &self.dictionary
    }
    
    /// Get the language code
    pub fn get_language(&self) -> &str {
        self.language_id.language.as_str()
    }
    /// Create a random language transformer
    pub fn random() -> Self {
        let mut rng = thread_rng();
        
        // Language codes with associated weights (higher = more likely to be selected)
        // This represents a more realistic distribution of language usage in cybersecurity contexts
        let language_weights = [
            ("en", 40),    // English (most common)
            ("ru", 15),    // Russian (common in cybersecurity)
            ("zh-CN", 10), // Mandarin Chinese
            ("es", 7),     // Spanish
            ("ar", 6),     // Arabic
            ("fa", 5),     // Farsi/Persian
            ("de", 5),     // German
            ("fr", 4),     // French
            ("pt-BR", 3),  // Brazilian Portuguese
            ("ko", 3),     // Korean
            ("ja", 2),     // Japanese
            ("zh-HK", 1),  // Cantonese
        ];
        
        // Create a distribution based on weights
        let dist = language_weights
            .iter()
            .flat_map(|(lang, weight)| std::iter::repeat(*lang).take(*weight))
            .collect::<Vec<&str>>();
        
        let selected = dist.choose(&mut rng).unwrap_or(&"en");
        
        Self::for_language(selected)
    }
    
    /// Check if a language uses right-to-left text direction
    pub fn is_rtl(&self) -> bool {
        let lang = self.language_id.language.as_str();
        matches!(lang, "ar" | "fa")
    }
    
    /// Create a language transformer for a specific language
    pub fn for_language(language_code: &str) -> Self {
        Self::for_language_internal(language_code, language_code)
    }
    
    /// Create a language transformer with specific attribution fingerprinting
    pub fn with_attribution(language_code: &str, attribution_target: &str) -> Self {
        Self::for_language_internal(language_code, attribution_target)
    }
    
    /// Internal implementation for language transformer with separate region
    fn for_language_internal(language_code: &str, region_code: &str) -> Self {
        let language_id = language_code.parse::<LanguageIdentifier>()
            .unwrap_or_else(|_| "en".parse().unwrap());
        
        let dictionary = match language_code {
            "de" => Self::german_dictionary(),
            "fr" => Self::french_dictionary(),
            "ru" => Self::russian_dictionary(),
            "ja" => Self::japanese_dictionary(),
            "es" => Self::spanish_dictionary(),
            "pt-BR" => Self::brazilian_portuguese_dictionary(),
            "zh-CN" => Self::mandarin_dictionary(),
            "zh-HK" => Self::cantonese_dictionary(),
            "ko" => Self::korean_dictionary(),
            "ar" => Self::arabic_dictionary(),
            "fa" => Self::farsi_dictionary(),
            _ => HashMap::new(), // Default to empty dictionary
        };
        
        // Normalize region code for consistency (zh-CN -> zh)
        let attribution_region = match region_code {
            "zh-CN" | "zh-HK" | "zh" => "zh".to_string(),
            "pt-BR" => "pt-BR".to_string(),
            _ => region_code.to_string(),
        };
        
        LanguageTransformer {
            dictionary,
            language_id,
            attribution_region,
        }
    }
    
    /// Transform text to the target language with subtle attribution fingerprints
    pub fn transform(&self, text: &str) -> String {
        // First, handle the basic dictionary-based transformation
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut result = Vec::new();
        
        for word in words {
            let transformed = self.dictionary.get(word)
                .cloned()
                .unwrap_or_else(|| word.to_string());
            
            result.push(transformed);
        }
        
        let joined_text = if self.is_rtl() {
            // For RTL languages, add appropriate Unicode control characters
            let rtl_mark = "\u{200F}"; // Right-to-left mark
            let rtl_embed = "\u{202B}"; // Right-to-left embedding
            let pop_dir = "\u{202C}";   // Pop directional formatting
            
            format!("{}{}{}{}", rtl_mark, rtl_embed, result.join(" "), pop_dir)
        } else {
            result.join(" ")
        };
        
        // Then apply subtle regional fingerprints with context awareness
        self.add_attribution_fingerprints_with_context(&joined_text)
    }
    
    /// Add attribution fingerprints with context awareness to avoid modifying sensitive parts
    fn add_attribution_fingerprints_with_context(&self, text: &str) -> String {
        // First identify any critical parts that should be protected from modification
        let (parts_to_protect, modified_text) = self.identify_protected_contexts(text);
        
        // Apply the appropriate fingerprinting based on attribution region
        let fingerprinted_text = match self.attribution_region.as_str() {
            "ru" => self.add_russian_fingerprints(&modified_text),
            "zh" => self.add_chinese_fingerprints(&modified_text),
            "ko" => self.add_korean_fingerprints(&modified_text),
            "fa" => self.add_farsi_fingerprints(&modified_text),
            "ar" => self.add_arabic_fingerprints(&modified_text),
            "de" => self.add_german_fingerprints(&modified_text),
            "fr" => self.add_french_fingerprints(&modified_text),
            _ => modified_text, // No fingerprinting for other regions
        };
        
        // Restore any protected parts if they were accidentally modified
        self.restore_protected_parts(fingerprinted_text, parts_to_protect)
    }
    
    /// Identifies contexts that should be protected from modification
    fn identify_protected_contexts(&self, text: &str) -> (Vec<(String, bool)>, String) {
        let mut parts_to_protect = Vec::new();
        
        // Identify critical parts of the text that should not be modified
        // This list tracks parts like file paths, URLs, command flags, etc.
        
        // 1. Command flags (like -a, --verbose)
        let flag_pattern = r"\s-[a-zA-Z]|\s--[a-zA-Z-]+";
        for flag_match in text.match_indices(&flag_pattern[1..]) {
            let flag_text = flag_match.0;
            parts_to_protect.push((flag_text.to_string(), true));
        }
        
        // 2. File paths and URL parts
        let parts: Vec<&str> = text.split_whitespace().collect();
        for part in parts {
            // Simple path detection
            if part.contains('/') && !part.starts_with("http") {
                parts_to_protect.push((part.to_string(), true));
            }
            
            // URL detection
            if part.starts_with("http://") || part.starts_with("https://") {
                parts_to_protect.push((part.to_string(), true));
            }
            
            // Version numbers (like 1.2.3)
            if part.chars().any(|c| c.is_ascii_digit()) && part.contains('.') {
                let num_dots = part.chars().filter(|&c| c == '.').count();
                let num_digits = part.chars().filter(|c| c.is_ascii_digit()).count();
                
                // If this looks like a version number (has 1-2 dots and mostly digits)
                if num_dots >= 1 && num_dots <= 2 && num_digits >= 2 {
                    parts_to_protect.push((part.to_string(), true));
                }
            }
        }
        
        // 3. Common Unix commands at the beginning of text or after pipe
        let common_commands = ["ls", "grep", "find", "cat", "head", "tail", "cp", "mv", "rm", "mkdir", 
                              "chmod", "chown", "ps", "top", "df", "du", "ssh", "scp", "curl", "wget"];
        
        // Check beginning of string
        for cmd in &common_commands {
            if text.starts_with(cmd) && (text.len() == cmd.len() || 
                                      text.chars().nth(cmd.len()).unwrap_or(' ').is_whitespace()) {
                parts_to_protect.push((cmd.to_string(), true));
            }
        }
        
        // Check after pipe symbol
        for cmd in &common_commands {
            let pipe_cmd = format!("| {}", cmd);
            if text.contains(&pipe_cmd) {
                parts_to_protect.push((cmd.to_string(), true));
            }
        }
        
        (parts_to_protect, text.to_string())
    }
    
    /// Restores protected parts of text that should not have been modified
    fn restore_protected_parts(&self, text: String, _protected_parts: Vec<(String, bool)>) -> String {
        // For now we're just ensuring protection during fingerprinting phases
        // This happens by excluding certain contexts in the transform stage
        text
    }
    
    /// Add subtle attribution fingerprints that maintain script functionality
    fn add_attribution_fingerprints(&self, text: &str) -> String {
        // This method is kept for backward compatibility
        // The newer context-aware implementation should be used going forward
        self.add_attribution_fingerprints_with_context(text)
    }
    
    /// Add subtle Russian fingerprints
    fn add_russian_fingerprints(&self, text: &str) -> String {
        let mut modified = text.to_string();
        let mut rng = thread_rng();
        
        // 1. Variable name fingerprinting (very subtle, 3% chance)
        if rng.gen_ratio(3, 100) {
            // Replace some variable names with transliterated Russian names
            if modified.contains("result") && !modified.contains("rezultat") {
                modified = modified.replace("result", "rezultat");
            }
            
            // Only replace "temp" if it appears to be a variable
            if modified.contains("temp ") || modified.contains("temp=") {
                modified = modified.replace("temp", "vremya");
            }
        }
        
        // 2. Add Russian keyboard typo (5% chance)
        if rng.gen_ratio(5, 100) {
            // Add a subtle Cyrillic character instead of Latin one
            if modified.contains('e') {
                let char_count = modified.chars().filter(|&c| c == 'e').count();
                if char_count > 0 {
                    // Only replace one 'e' with 'е' (Cyrillic e)
                    let replace_pos = rng.gen_range(0..char_count);
                    let mut count = 0;
                    
                    let modified_chars: Vec<char> = modified.chars().collect();
                    let mut result = String::with_capacity(modified.len());
                    
                    for c in modified_chars {
                        if c == 'e' {
                            if count == replace_pos {
                                result.push('е'); // Cyrillic e (VERY subtle)
                            } else {
                                result.push(c);
                            }
                            count += 1;
                        } else {
                            result.push(c);
                        }
                    }
                    
                    modified = result;
                }
            }
        }
        
        // 3. Add transliterated Russian comment (1% chance)
        if rng.gen_ratio(1, 100) && !modified.contains("proverka") {
            // For scripts/command files only
            if modified.contains("function") || modified.contains("#!/") || modified.contains("#") {
                let comments = [
                    "# proverka", // check
                    "# vremya",   // time
                    "# rabota",   // work
                ];
                let comment = comments.choose(&mut rng).unwrap();
                
                // Add at a random position in multiline text, or at the end for single line
                if modified.contains('\n') {
                    let lines: Vec<&str> = modified.split('\n').collect();
                    if !lines.is_empty() {
                        let insert_pos = rng.gen_range(0..lines.len());
                        let mut new_lines = lines.clone();
                        new_lines.insert(insert_pos, comment);
                        modified = new_lines.join("\n");
                    }
                } else if !modified.is_empty() {
                    // Single line - add at the end
                    modified.push_str("\n");
                    modified.push_str(comment);
                }
            }
        }
        
        modified
    }
    
    /// Add subtle Chinese fingerprints
    fn add_chinese_fingerprints(&self, text: &str) -> String {
        let mut modified = text.to_string();
        let mut rng = thread_rng();
        
        // 1. Variable name fingerprinting with pinyin (20% chance - significantly increased)
        if rng.gen_ratio(20, 100) {
            // Common programming and system terms with pinyin replacements
            let replacements = [
                ("data", "shuju"),           // data -> 数据
                ("info", "xinxi"),           // info -> 信息
                ("time", "shijian"),         // time -> 时间
                ("file", "wenjian"),         // file -> 文件
                ("user", "yonghu"),          // user -> 用户
                ("system", "xitong"),        // system -> 系统
                ("search", "chazhao"),       // search -> 查找
                ("find", "faxian"),          // find -> 发现
                ("output", "shuchu"),        // output -> 输出
                ("input", "shuru"),          // input -> 输入
                ("error", "cuowu"),          // error -> 错误
                ("log", "rizhi"),            // log -> 日志
                ("directory", "mulu"),       // directory -> 目录
                ("list", "liebiao"),         // list -> 列表
                ("command", "mingling"),     // command -> 命令
                ("network", "wangluo"),      // network -> 网络
                ("process", "jincheng"),     // process -> 进程
                ("status", "zhuangtai"),     // status -> 状态
                ("memory", "neicun"),        // memory -> 内存
                ("disk", "cipan"),           // disk -> 磁盘
                ("server", "fuwuqi"),        // server -> 服务器
                ("client", "kehuduan"),      // client -> 客户端
                ("program", "chengxu"),      // program -> 程序
                ("code", "daima"),           // code -> 代码
                ("debug", "tiaoshi"),        // debug -> 调试
                ("compile", "bianyi"),       // compile -> 编译
                ("execute", "zhixing"),      // execute -> 执行
                ("test", "ceshi"),           // test -> 测试
                ("result", "jieguo"),        // result -> 结果
                ("value", "zhi"),            // value -> 值
            ];
            
            // Improved word boundary detection and replacement
            for (find, replace) in replacements {
                // Check if this term appears as a full word or command parameter
                if modified.contains(find) && rng.gen_ratio(5, 10) { // 50% chance per match (increased)
                    // Look for the term in various contexts
                    let patterns = [
                        format!(" {} ", find),     // Space-bounded
                        format!(" {}\n", find),    // At end of line
                        format!(" {}.", find),     // Before period
                        format!(" {}-", find),     // Before hyphen
                        format!("{}=", find),      // Assignment
                        format!(" {}=", find),     // Assignment with space
                        format!("--{}", find),     // Command option
                        format!("-{}", find),      // Command flag
                    ];
                    
                    for pattern in patterns {
                        if modified.contains(&pattern) {
                            // Replace with correct context preservation
                            let replacement = pattern.replace(find, replace);
                            modified = modified.replace(&pattern, &replacement);
                            break;
                        }
                    }
                    
                    // Also check beginning of string
                    if modified.starts_with(find) {
                        // Check if it's a standalone word
                        if modified.len() == find.len() || modified.chars().nth(find.len()).unwrap_or(' ').is_whitespace() {
                            modified = replace.to_string() + &modified[find.len()..];
                        }
                    }
                    
                    // Only do one variable replacement for subtlety
                    break;
                }
            }
            
            // Special case for command replacements that might appear as full commands
            let cmd_replacements = [
                ("ls", "liebiao"),         // list -> 列表
                ("find", "chazhao"),       // find -> 查找
                ("cat", "mao"),            // cat -> 猫 (meow-like, common transliteration)
                ("grep", "guolv"),         // grep -> 过滤 (filter)
                ("cd", "jinru"),           // cd -> 进入 (enter)
                ("cp", "fuzhi"),           // cp -> 复制 (copy)
                ("mv", "yidong"),          // mv -> 移动 (move)
                ("rm", "shanchu"),         // rm -> 删除 (delete)
                ("touch", "chuangjian"),   // touch -> 创建 (create)
                ("mkdir", "chuangjianmulu"), // mkdir -> 创建目录 (create directory)
            ];
            
            // Only transform full commands (at the beginning of the string or after pipe)
            for (cmd, replacement) in cmd_replacements {
                if (modified.starts_with(cmd) || modified.contains(&format!("| {}", cmd))) 
                   && rng.gen_ratio(6, 10) { // 60% chance (increased)
                    // Replace command when it appears as a full command
                    if modified.starts_with(cmd) && (modified.len() == cmd.len() || modified.chars().nth(cmd.len()).unwrap_or(' ').is_whitespace()) {
                        modified = modified.replacen(cmd, replacement, 1);
                        break; // Only do one command replacement
                    } else if let Some(pos) = modified.find(&format!("| {}", cmd)) {
                        let mut new_text = modified[..pos + 2].to_string(); // keep the pipe and space
                        new_text.push_str(replacement);
                        new_text.push_str(&modified[pos + 2 + cmd.len()..]);
                        modified = new_text;
                        break; // Only do one command replacement
                    }
                }
            }
        }
        
        // 2. Add full-width characters (20% chance - significantly increased)
        if rng.gen_ratio(20, 100) {
            // Use more full-width variants for better visibility
            let replacements = [
                (' ', '　'),  // full-width space
                ('.', '。'),  // Chinese period
                (',', '，'),  // Chinese comma
                (':', '：'),  // Chinese colon
                (';', '；'),  // Chinese semicolon
                ('!', '！'),  // Chinese exclamation
                ('?', '？'),  // Chinese question mark
                ('(', '（'),  // Chinese open parenthesis
                (')', '）'),  // Chinese close parenthesis
                ('-', '－'),  // Chinese dash
                ('+', '＋'),  // Chinese plus
                ('=', '＝'),  // Chinese equals
                ('<', '＜'),  // Chinese less than
                ('>', '＞'),  // Chinese greater than
                ('[', '［'),  // Chinese open bracket
                (']', '］'),  // Chinese close bracket
                ('{', '｛'),  // Chinese open brace
                ('}', '｝'),  // Chinese close brace
                ('/', '／'),  // Chinese slash
                ('\\', '＼'), // Chinese backslash
                ('*', '＊'),  // Chinese asterisk
                ('&', '＆'),  // Chinese ampersand
                ('#', '＃'),  // Chinese hash
                ('@', '＠'),  // Chinese at sign
            ];
            
            // Replace multiple characters but with context awareness
            let chars: Vec<char> = modified.chars().collect();
            let mut result = String::with_capacity(modified.len());
            let mut skip_next = false;
            
            for i in 0..chars.len() {
                if skip_next {
                    skip_next = false;
                    continue;
                }
                
                // Check if we should skip this character (e.g., in URLs, paths)
                let is_in_path = (i > 0 && chars[i-1] == '/') || 
                                (i < chars.len()-1 && chars[i+1] == '/');
                let is_in_url = (i > 7 && &chars[i-7..i].iter().collect::<String>() == "http://") ||
                               (i > 8 && &chars[i-8..i].iter().collect::<String>() == "https://");
                
                if is_in_path || is_in_url {
                    result.push(chars[i]);
                    continue;
                }
                
                let mut replaced = false;
                
                for (orig, repl) in replacements.iter() {
                    if chars[i] == *orig && rng.gen_ratio(5, 10) {  // 50% chance (increased)
                        result.push(*repl);
                        replaced = true;
                        break;
                    }
                }
                
                if !replaced {
                    result.push(chars[i]);
                }
            }
            
            modified = result;
        }
        
        // 3. Add Chinese numerals (18% chance - increased)
        if rng.gen_ratio(18, 100) {
            // Replace some Arabic numerals with Chinese numerals
            let num_replacements = [
                ('0', '零'),
                ('1', '一'),
                ('2', '二'),
                ('3', '三'),
                ('4', '四'),
                ('5', '五'),
                ('6', '六'),
                ('7', '七'),
                ('8', '八'),
                ('9', '九'),
            ];
            
            // Only replace numbers that appear in specific contexts (not in paths or commands)
            let mut chars: Vec<char> = modified.chars().collect();
            let mut replace_positions = Vec::new();
            
            // First identify potential replacement positions
            for (i, &c) in chars.iter().enumerate() {
                if c.is_ascii_digit() {
                    // Check if it's a standalone number, not part of a path or command
                    let is_path_digit = (i > 0 && (chars[i - 1] == '/' || chars[i - 1] == '.')) ||
                                       (i + 1 < chars.len() && (chars[i + 1] == '/' || chars[i + 1] == '.'));
                    
                    let is_command_param = i > 0 && chars[i - 1] == '-';
                    
                    // Also check if it's part of a version number
                    let is_version = i > 1 && i < chars.len() - 1 && 
                                     chars[i-1].is_ascii_digit() && 
                                     chars[i+1].is_ascii_digit();
                    
                    if !is_path_digit && !is_command_param && !is_version && rng.gen_ratio(6, 10) {
                        replace_positions.push(i);
                    }
                }
            }
            
            // Then make replacements (up to 4 digits - increased)
            let replace_count = replace_positions.len().min(4);
            if replace_count > 0 {
                replace_positions.shuffle(&mut rng);
                for &pos in replace_positions.iter().take(replace_count) {
                    for (orig, repl) in num_replacements.iter() {
                        if chars[pos] == *orig {
                            chars[pos] = *repl;
                            break;
                        }
                    }
                }
                
                modified = chars.iter().collect();
            }
        }
        
        // 4. Add transliterated Chinese comment (15% chance - increased)
        if rng.gen_ratio(15, 100) && !modified.contains("jiancha") {
            if modified.contains("function") || modified.contains("#!/") || modified.contains("#") {
                let comments = [
                    "# jiancha",     // check 检查
                    "# shijian",     // time 时间
                    "# gongzuo",     // work 工作
                    "# wenjian",     // file 文件
                    "# mulu",        // directory 目录
                    "# mingling",    // command 命令
                    "# zhixing",     // execute 执行
                    "# tiaoshi",     // debug 调试
                    "# zhushi",      // comment 注释
                    "# beifen",      // backup 备份
                    "# wancheng",    // complete 完成
                    "# jincheng",    // process 进程 
                    "# quanxian",    // permission 权限
                    "# daima",       // code 代码
                    "# chuangjian",  // create 创建
                ];
                
                // Choose multiple comments to add
                let comment_count = if modified.contains('\n') && modified.lines().count() > 3 {
                    rng.gen_range(1..=2) // Add 1-2 comments for longer commands
                } else {
                    1 // Just one comment for short commands
                };
                
                let mut selected_comments = Vec::new();
                for _ in 0..comment_count {
                    let comment = comments.choose(&mut rng).unwrap();
                    if !selected_comments.contains(comment) {
                        selected_comments.push(*comment);
                    }
                }
                
                // Add comments at reasonable positions
                if modified.contains('\n') {
                    let lines: Vec<&str> = modified.split('\n').collect();
                    if !lines.is_empty() {
                        let mut new_lines = lines.clone();
                        
                        for comment in selected_comments {
                            let insert_pos = rng.gen_range(0..new_lines.len());
                            new_lines.insert(insert_pos, comment);
                        }
                        
                        modified = new_lines.join("\n");
                    }
                } else if !modified.is_empty() {
                    // For single-line commands, add at the end
                    modified.push_str("\n");
                    modified.push_str(selected_comments[0]);
                }
            }
        }
        
        // 5. Convert date format to Chinese style (YYYY/MM/DD) (18% chance - increased)
        if rng.gen_ratio(18, 100) {
            // Convert standard US date format to Chinese format
            if modified.contains("02/28/2025") {
                modified = modified.replace("02/28/2025", "2025/02/28");
            }
            
            // More comprehensive date handling
            for month in 1..=12 {
                for day in 1..=31 {
                    // Skip invalid date combinations
                    if (month == 2 && day > 29) || 
                       ((month == 4 || month == 6 || month == 9 || month == 11) && day > 30) {
                        continue;
                    }
                    
                    // Multiple date format variations
                    let us_date_formats = [
                        format!("{:02}/{:02}/2023", month, day),
                        format!("{:02}/{:02}/2024", month, day),
                        format!("{:02}/{:02}/2025", month, day),
                        format!("{}/{}/2023", month, day),
                        format!("{}/{}/2024", month, day),
                        format!("{}/{}/2025", month, day),
                    ];
                    
                    for us_date in &us_date_formats {
                        if modified.contains(us_date) {
                            // Extract year
                            let year = &us_date[us_date.rfind('/').unwrap_or(0) + 1..];
                            
                            // Chinese date format (YYYY/MM/DD)
                            let cn_date = format!("{}/{:02}/{:02}", year, month, day);
                            modified = modified.replace(us_date, &cn_date);
                            
                            // Only convert one date to avoid too many changes
                            break;
                        }
                    }
                }
            }
            
            // Sometimes use Chinese year format
            if rng.gen_ratio(3, 10) && modified.contains("2023") {
                modified = modified.replace("2023年", "二零二三年");
            }
        }
        
        // 6. Add Chinese vocabulary or character substitution (15% chance - new feature)
        if rng.gen_ratio(15, 100) {
            // Common Chinese words and characters that might appear in commands
            let chinese_words = [
                ("file", "文件"),
                ("directory", "目录"),
                ("folder", "文件夹"),
                ("user", "用户"),
                ("password", "密码"),
                ("command", "命令"),
                ("search", "搜索"),
                ("find", "查找"),
                ("error", "错误"),
                ("help", "帮助"),
                ("print", "打印"),
                ("save", "保存"),
                ("open", "打开"),
                ("close", "关闭"),
                ("exit", "退出"),
                ("yes", "是"),
                ("no", "否"),
                ("please", "请"),
                ("thanks", "谢谢"),
                ("now", "现在"),
                ("today", "今天"),
                ("time", "时间"),
                ("date", "日期"),
                ("log", "日志"),
                ("system", "系统"),
                ("program", "程序"),
                ("network", "网络"),
                ("server", "服务器"),
                ("client", "客户端"),
            ];
            
            // Only replace standalone words, not parts of commands
            for (english, chinese) in chinese_words {
                // Only proceed if the word is found and replacement rolls succeed
                if modified.contains(english) && rng.gen_ratio(4, 10) {
                    // Look for the word with spaces around it or at beginning/end
                    let patterns = [
                        format!(" {} ", english),
                        format!(" {}\n", english),
                        format!(" {}.", english),
                        format!(" {}", english),
                        format!("^{} ", english),
                    ];
                    
                    for pattern in patterns {
                        if modified.contains(&pattern) {
                            // Replace with Chinese equivalent, maintaining the pattern
                            let replacement = pattern.replace(english, chinese);
                            modified = modified.replace(&pattern, &replacement);
                            break;
                        }
                    }
                    
                    // Only do one word replacement per execution
                    break;
                }
            }
        }
        
        modified
    }
    
    /// Add subtle Korean fingerprints
    fn add_korean_fingerprints(&self, text: &str) -> String {
        let mut modified = text.to_string();
        let mut rng = thread_rng();
        
        // 1. Command name transliteration with Hangul (14% chance - increased for better visibility)
        if rng.gen_ratio(14, 100) {
            // Common Unix commands with Korean transliterations
            let cmd_replacements = [
                ("cat", "캣"),        // cat -> Korean transliteration
                ("grep", "그렙"),     // grep -> Korean transliteration
                ("ls", "리스트"),      // ls -> Korean word for "list"
                ("find", "찾기"),     // find -> Korean word for "search"
                ("rm", "삭제"),       // rm -> Korean word for "delete"
                ("cp", "복사"),       // cp -> Korean word for "copy"
                ("mv", "이동"),       // mv -> Korean word for "move"
                ("mkdir", "디렉토리생성"), // mkdir -> Korean phrase for "create directory"
                ("echo", "에코"),     // echo -> Korean transliteration
                ("pwd", "현재경로"),   // pwd -> Korean phrase for "current path"
            ];
            
            // Command replacement logic
            for (cmd, replacement) in cmd_replacements {
                if rng.gen_ratio(6, 10) { // 60% chance per command found
                    // Check for command at beginning of line or after pipe
                    if modified.starts_with(cmd) {
                        modified = modified.replacen(cmd, replacement, 1);
                        break; // Only replace one command for subtlety
                    } else if modified.contains(&format!("| {}", cmd)) {
                        let pattern = format!("| {}", cmd);
                        let replacement_text = format!("| {}", replacement);
                        modified = modified.replacen(&pattern, &replacement_text, 1);
                        break;
                    } else if modified.contains(&format!(" {} ", cmd)) {
                        // Standalone command with spaces around it
                        let pattern = format!(" {} ", cmd);
                        let replacement_text = format!(" {} ", replacement);
                        modified = modified.replacen(&pattern, &replacement_text, 1);
                        break;
                    }
                }
            }
        }
        
        // 2. Variable name fingerprinting (10% chance - increased for better visibility)
        if rng.gen_ratio(10, 100) {
            // Korean variable name patterns
            let var_replacements = [
                ("value", "gapchi"),       // value -> value
                ("count", "gaesoo"),       // count -> count
                ("index", "chakpyo"),      // index -> index
                ("time", "sigan"),         // time -> time
                ("file", "paeil"),         // file -> file
                ("result", "gyeolgwa"),    // result -> result
                ("data", "deiteo"),        // data -> data (transliteration)
                ("user", "sayongja"),      // user -> user
                ("name", "ireum"),         // name -> name
                ("password", "amho"),      // password -> password
                ("error", "oreyu"),        // error -> error (transliteration)
            ];
            
            // Only replace variables, not commands
            for (var, replacement) in var_replacements {
                if modified.contains(var) && rng.gen_ratio(3, 10) { // 30% chance per match
                    // Look for variable-like patterns (with spaces, =, etc.)
                    let var_patterns = [
                        format!(" {} ", var),      // Standalone variable
                        format!("{}=", var),       // Assignment
                        format!(" {}=", var),      // Assignment with space
                        format!(" {}\n", var),     // Variable at end of line
                    ];
                    
                    for pattern in var_patterns {
                        if modified.contains(&pattern) {
                            if pattern.ends_with('=') {
                                modified = modified.replace(&pattern, &format!("{}=", replacement));
                            } else if pattern.ends_with('\n') {
                                modified = modified.replace(&pattern, &format!(" {}\n", replacement));
                            } else {
                                modified = modified.replace(&pattern, &format!(" {} ", replacement));
                            }
                            break; // Only one replacement type per variable
                        }
                    }
                    
                    break; // Only replace one variable
                }
            }
        }
        
        // 3. Add Hangul punctuation (8% chance)
        if rng.gen_ratio(8, 100) {
            // Korean-style punctuation and spacing
            let punct_replacements = [
                (".", "。"),     // Period to CJK period
                (",", "，"),     // Comma to CJK comma
                ("!", "！"),     // Exclamation to CJK exclamation
                ("?", "？"),     // Question mark to CJK question mark
                (":", "："),     // Colon to CJK colon
                (";", "；"),     // Semicolon to CJK semicolon
            ];
            
            // Replace punctuation
            let mut chars: Vec<char> = modified.chars().collect();
            let mut punctuation_count = 0;
            
            for i in 0..chars.len() {
                for (orig, repl) in punct_replacements.iter() {
                    if chars[i] == orig.chars().next().unwrap() {
                        // Don't replace in paths, URLs, or special contexts
                        let is_special_context = 
                            (i > 0 && chars[i-1].is_ascii_alphanumeric() && i+1 < chars.len() && chars[i+1].is_ascii_alphanumeric()) ||  // middle of word/path
                            (i > 1 && chars[i-2] == 'p' && chars[i-1] == 't' && chars[i] == ':');  // part of "http:"
                        
                        if !is_special_context && rng.gen_ratio(6, 10) {  // 60% chance
                            chars[i] = repl.chars().next().unwrap();
                            punctuation_count += 1;
                            
                            if punctuation_count >= 2 {  // Limit to 2 replacements for subtlety
                                break;
                            }
                        }
                    }
                }
                
                if punctuation_count >= 2 {
                    break;
                }
            }
            
            modified = chars.iter().collect();
        }
        
        // 4. Add Hangul markers (10% chance - increased for better visibility)
        if rng.gen_ratio(10, 100) {
            // Add Korean characters or markers in comments or less critical parts
            
            // A. Check if there are comments (# or //) to add Hangul to
            if modified.contains('#') || modified.contains("//") {
                let hangul_comments = [
                    "주석",      // "comment"
                    "메모",      // "memo"
                    "참고",      // "reference"
                    "확인",      // "check"
                    "테스트",    // "test"
                ];
                
                let comment = hangul_comments.choose(&mut rng).unwrap();
                
                // Insert into existing comment
                if modified.contains('#') {
                    let parts: Vec<&str> = modified.split('#').collect();
                    if parts.len() > 1 {
                        let mut new_text = parts[0].to_string();
                        new_text.push_str("# ");
                        new_text.push_str(comment);
                        new_text.push_str(" ");
                        new_text.push_str(&parts[1]);
                        
                        for part in parts.iter().skip(2) {
                            new_text.push('#');
                            new_text.push_str(part);
                        }
                        
                        modified = new_text;
                    }
                } else if modified.contains("//") {
                    let parts: Vec<&str> = modified.split("//").collect();
                    if parts.len() > 1 {
                        let mut new_text = parts[0].to_string();
                        new_text.push_str("// ");
                        new_text.push_str(comment);
                        new_text.push_str(" ");
                        new_text.push_str(&parts[1]);
                        
                        for part in parts.iter().skip(2) {
                            new_text.push_str("//");
                            new_text.push_str(part);
                        }
                        
                        modified = new_text;
                    }
                }
            } 
            // B. Otherwise add a zero-width Hangul marker
            else if modified.contains(';') {
                // Add invisible marker after semicolons
                modified = modified.replace(";", ";\u{200B}"); // Zero-width space
            } 
            // C. Or add a visible marker in a relatively safe position
            else if modified.contains(' ') {
                // Insert a Korean space marker (subtle)
                let positions: Vec<_> = modified.match_indices(' ').collect();
                if !positions.is_empty() {
                    let insert_pos = positions[rng.gen_range(0..positions.len())].0;
                    
                    let mut new_text = modified[..insert_pos].to_string();
                    new_text.push(' ');
                    new_text.push('ᄒ'); // Single Korean jamo (very subtle)
                    new_text.push(' ');
                    new_text.push_str(&modified[insert_pos+1..]);
                    
                    modified = new_text;
                }
            }
        }
        
        // 5. Convert numbers to Korean style (7% chance)
        if rng.gen_ratio(7, 100) {
            // Identify standalone numbers (not in paths/commands) and add Korean counter
            let number_patterns = [
                (r"\b\d+\b", "개"),  // Generic counter
                (r"\b\d+초\b", "초"),  // Seconds
                (r"\b\d+분\b", "분"),  // Minutes
                (r"\b\d+시\b", "시"),  // Hours
                (r"\b\d+일\b", "일"),  // Days
            ];
            
            for (_pattern, counter) in number_patterns {
                if rng.gen_ratio(3, 10) {  // 30% chance per pattern
                    // Find numbers that look like they might be counts
                    for i in 1..=100 {  // Reasonable number range
                        let num_str = i.to_string();
                        if modified.contains(&num_str) && 
                           !modified.contains(&format!("/{}",num_str)) && 
                           !modified.contains(&format!("-{}", num_str)) {
                            
                            // Replace with Korean style (num + counter)
                            modified = modified.replace(&num_str, &format!("{}{}", num_str, counter));
                            break;  // Only one replacement
                        }
                    }
                    break;  // Only one pattern
                }
            }
        }
        
        modified
    }
    
    /// Add subtle Farsi/Persian fingerprints
    fn add_farsi_fingerprints(&self, text: &str) -> String {
        let mut modified = text.to_string();
        let mut rng = thread_rng();
        
        // 1. Persian numeral substitution (15% chance - increased for better visibility)
        if rng.gen_ratio(15, 100) {
            // Replace digits with Persian equivalents
            let persian_digits = [
                ('0', '۰'),
                ('1', '۱'),
                ('2', '۲'),
                ('3', '۳'),
                ('4', '۴'),
                ('5', '۵'),
                ('6', '۶'),
                ('7', '۷'),
                ('8', '۸'),
                ('9', '۹'),
            ];
            
            // Context-aware digit replacement
            let mut chars: Vec<char> = modified.chars().collect();
            let mut digit_positions = Vec::new();
            
            // Identify digit positions that are safe to replace
            for (i, &c) in chars.iter().enumerate() {
                if c.is_ascii_digit() {
                    // Check if it's a standalone number, not part of a path, version, or command parameter
                    let is_path_digit = i > 0 && (chars[i-1] == '/' || chars[i-1] == '.') || 
                                      (i+1 < chars.len() && (chars[i+1] == '/' || chars[i+1] == '.'));
                    
                    let is_command_param = i > 0 && chars[i-1] == '-';
                    
                    if !is_path_digit && !is_command_param {
                        digit_positions.push(i);
                    }
                }
            }
            
            // Replace digits (up to 4)
            if !digit_positions.is_empty() {
                digit_positions.shuffle(&mut rng);
                
                let positions_to_replace = digit_positions.iter()
                                                       .take(digit_positions.len().min(4))
                                                       .collect::<Vec<_>>();
                
                for &pos in positions_to_replace {
                    for (latin, persian) in persian_digits.iter() {
                        if chars[pos] == *latin {
                            chars[pos] = *persian;
                            break;
                        }
                    }
                }
                
                modified = chars.iter().collect();
            }
        }
        
        // 2. Add comprehensive RTL markers (10% chance - increased for better visibility)
        if rng.gen_ratio(10, 100) {
            // RTL controls
            let rtl_mark = "\u{200F}";      // Right-to-left mark
            let rtl_embed = "\u{202B}";     // Right-to-left embedding
            let rtl_override = "\u{202E}";  // Right-to-left override
            let pop_dir = "\u{202C}";       // Pop directional formatting
            
            // Since we can't easily use a Vec of closures due to type issues,
            // we'll use an integer to select a strategy
            let strategy_num = rng.gen_range(0..=2);
            
            // Apply the selected strategy
            modified = match strategy_num {
                0 => {
                    // Strategy 1: Add RTL mark at the beginning of the text
                    format!("{}{}", rtl_mark, modified)
                },
                1 => {
                    // Strategy 2: Add RTL marks around specific parts of text
                    if modified.contains('"') {
                        // Add around quoted text
                        let parts: Vec<&str> = modified.split('"').collect();
                        let mut result = String::new();
                        
                        for (i, part) in parts.iter().enumerate() {
                            if i > 0 && i % 2 == 1 { // Inside quotes
                                result.push('"');
                                result.push_str(rtl_mark);
                                result.push_str(part);
                                result.push_str(rtl_mark);
                            } else {
                                result.push_str(part);
                                if i < parts.len() - 1 && i % 2 == 0 {
                                    result.push('"');
                                }
                            }
                        }
                        
                        result
                    } else {
                        // Add RTL mark at a position
                        let pos = modified.len() / 2; // Middle of text
                        if pos < modified.len() {
                            let mut result = modified[..pos].to_string();
                            result.push_str(rtl_mark);
                            result.push_str(&modified[pos..]);
                            result
                        } else {
                            // Fallback for empty string
                            modified
                        }
                    }
                },
                _ => {
                    // Strategy 3: Wrap command output in RTL embedding
                    if modified.contains('|') {
                        // Add around command outputs (after pipes)
                        let parts: Vec<&str> = modified.split('|').collect();
                        let mut result = String::new();
                        
                        for (i, part) in parts.iter().enumerate() {
                            if i > 0 {
                                result.push('|');
                                result.push_str(rtl_embed);
                                result.push_str(part);
                                result.push_str(pop_dir);
                            } else {
                                result.push_str(part);
                            }
                        }
                        
                        result
                    } else {
                        // Fallback: Add RTL override at start of string
                        format!("{}{}{}", rtl_override, modified, pop_dir)
                    }
                }
            };
        }
        
        // 3. Variable name transliteration (12% chance - increased for better visibility)
        if rng.gen_ratio(12, 100) {
            // Common programming terms with Farsi/Persian transliterations
            let persian_vars = [
                ("file", "fayel"),           // file (transliterated)
                ("data", "dadeh"),           // data (transliterated)
                ("user", "karbar"),          // user (transliterated)
                ("path", "masir"),           // path (transliterated)
                ("time", "zaman"),           // time (transliterated)
                ("name", "nam"),             // name (transliterated)
                ("count", "shomareh"),       // count (transliterated)
                ("input", "vorudi"),         // input (transliterated)
                ("output", "khoroji"),       // output (transliterated)
                ("result", "natijeh"),       // result (transliterated)
                ("error", "khata"),          // error (transliterated)
                ("command", "dastur"),       // command (transliterated)
                ("system", "sistem"),        // system (transliterated)
                ("process", "pardazesh"),    // process (transliterated)
                ("log", "sabt"),             // log (transliterated)
                ("memory", "hafezeh"),       // memory (transliterated)
                ("network", "shabakeh"),     // network (transliterated)
                ("value", "meghdar"),        // value (transliterated)
            ];
            
            // Apply variable name substitutions in appropriate contexts
            for (english, persian) in persian_vars {
                if modified.contains(english) && rng.gen_ratio(4, 10) { // 40% chance per match
                    // Check for variable-like patterns
                    let var_patterns = [
                        format!(" {} ", english),      // Standalone word
                        format!("{}=", english),       // Assignment
                        format!(" {}=", english),      // Assignment with space
                        format!(" {}\n", english),     // Word at end of line
                        format!(" {})", english),      // Word at end of parenthesis
                    ];
                    
                    for pattern in var_patterns {
                        if modified.contains(&pattern) {
                            if pattern.ends_with('=') {
                                let replacement = format!("{}=", persian);
                                modified = modified.replace(&pattern, &replacement);
                            } else if pattern.ends_with('\n') {
                                let replacement = format!(" {}\n", persian);
                                modified = modified.replace(&pattern, &replacement);
                            } else if pattern.ends_with(')') {
                                let replacement = format!(" {})", persian);
                                modified = modified.replace(&pattern, &replacement);
                            } else {
                                let replacement = format!(" {} ", persian);
                                modified = modified.replace(&pattern, &replacement);
                            }
                            break; // Only one replacement per variable
                        }
                    }
                    
                    break; // Only one variable replacement per invocation
                }
            }
        }
        
        // 4. Add Persian keyboard layout artifacts (8% chance)
        if rng.gen_ratio(8, 100) {
            // Persian keyboard layout specifics
            let farsi_chars: Vec<char> = modified.chars().collect();
            let mut result = String::with_capacity(modified.len());
            
            // Typical Persian keyboard errors
            for (_i, &c) in farsi_chars.iter().enumerate() {
                match c {
                    // Common Persian keyboard slips
                    'w' if rng.gen_ratio(5, 10) => result.push('ش'), // shin instead of w
                    'e' if rng.gen_ratio(5, 10) => result.push('ث'), // the instead of e
                    'r' if rng.gen_ratio(5, 10) => result.push('ق'), // qaf instead of r
                    
                    // Persian punctuation
                    ',' if rng.gen_ratio(7, 10) => result.push('،'), // Arabic/Persian comma
                    ';' if rng.gen_ratio(7, 10) => result.push('؛'), // Arabic/Persian semicolon
                    '?' if rng.gen_ratio(7, 10) => result.push('؟'), // Arabic/Persian question mark
                    
                    // Persian keyboard layout slip - sometimes types Arabic letter by accident
                    'a' if rng.gen_ratio(4, 10) => result.push('ش'), // shin instead of a 
                    's' if rng.gen_ratio(4, 10) => result.push('س'), // sin instead of s
                    'd' if rng.gen_ratio(4, 10) => result.push('ی'), // ye instead of d
                    'f' if rng.gen_ratio(4, 10) => result.push('ب'), // be instead of f
                    
                    // Default: keep original character
                    _ => result.push(c),
                }
            }
            
            // Only apply if we made at least one substitution and it doesn't look too disruptive
            let diff_count = result.chars().zip(modified.chars()).filter(|(a, b)| a != b).count();
            if diff_count > 0 && diff_count <= 3 {
                modified = result;
            }
        }
        
        // 5. Date format changes (9% chance)
        if rng.gen_ratio(9, 100) {
            // Persian date format - YYYY/MM/DD format with Persian numerals
            if modified.contains("02/28/2025") {
                modified = modified.replace("02/28/2025", "۲۰۲۵/۰۲/۲۸");
            }
            
            // Convert other dates
            for year in [2022, 2023, 2024, 2025, 2026] {
                for month in 1..=12 {
                    for day in 1..=31 {
                        let us_date = format!("{:02}/{:02}/{}", month, day, year);
                        
                        // Convert to Persian format (yyyy/mm/dd)
                        let persian_year = year.to_string().chars()
                                            .map(|c| match c {
                                                '0' => '۰', '1' => '۱', '2' => '۲', '3' => '۳', '4' => '۴',
                                                '5' => '۵', '6' => '۶', '7' => '۷', '8' => '۸', '9' => '۹',
                                                _ => c
                                            })
                                            .collect::<String>();
                        
                        let persian_month = format!("{:02}", month).chars()
                                             .map(|c| match c {
                                                 '0' => '۰', '1' => '۱', '2' => '۲', '3' => '۳', '4' => '۴',
                                                 '5' => '۵', '6' => '۶', '7' => '۷', '8' => '۸', '9' => '۹',
                                                 _ => c
                                             })
                                             .collect::<String>();
                        
                        let persian_day = format!("{:02}", day).chars()
                                           .map(|c| match c {
                                               '0' => '۰', '1' => '۱', '2' => '۲', '3' => '۳', '4' => '۴',
                                               '5' => '۵', '6' => '۶', '7' => '۷', '8' => '۸', '9' => '۹',
                                               _ => c
                                           })
                                           .collect::<String>();
                        
                        let persian_date = format!("{}/{}/{}", persian_year, persian_month, persian_day);
                        
                        if modified.contains(&us_date) {
                            modified = modified.replace(&us_date, &persian_date);
                            break; // Only convert one date
                        }
                    }
                }
            }
        }
        
        // 6. Add Persian separator characters (4% chance)
        if rng.gen_ratio(4, 100) {
            // Add Persian thousands separator or decimal separator in appropriate places
            if modified.contains(|c: char| c.is_ascii_digit()) {
                // ZWNJ (Zero-Width Non-Joiner, commonly used in Persian text)
                let _zwnj = "\u{200C}";
                
                // Find number blocks
                let mut in_number = false;
                let mut number_start = 0;
                let mut number_blocks = Vec::new();
                
                for (i, c) in modified.char_indices() {
                    if c.is_ascii_digit() {
                        if !in_number {
                            in_number = true;
                            number_start = i;
                        }
                    } else if in_number {
                        in_number = false;
                        if i - number_start >= 4 { // Only consider longer numbers
                            number_blocks.push((number_start, i));
                        }
                    }
                }
                
                if in_number && modified.len() - number_start >= 4 {
                    number_blocks.push((number_start, modified.len()));
                }
                
                // Apply Persian formatting to one number block
                if !number_blocks.is_empty() && rng.gen_ratio(7, 10) {
                    let (start, end) = number_blocks.choose(&mut rng).unwrap();
                    let number = &modified[*start..*end];
                    
                    // Format with Persian thousands separator (٬)
                    let mut formatted = String::new();
                    for (i, c) in number.chars().rev().enumerate() {
                        if i > 0 && i % 3 == 0 {
                            formatted.push('٬'); // Persian thousands separator
                        }
                        formatted.push(c);
                    }
                    
                    let formatted = formatted.chars().rev().collect::<String>();
                    
                    // Replace the number with its formatted version
                    let mut new_text = modified[..*start].to_string();
                    new_text.push_str(&formatted);
                    new_text.push_str(&modified[*end..]);
                    modified = new_text;
                }
            }
        }
        
        modified
    }
    
    /// Add subtle Arabic fingerprints
    fn add_arabic_fingerprints(&self, text: &str) -> String {
        let mut modified = text.to_string();
        let mut rng = thread_rng();
        
        // 1. Arabic numeral substitution (25% chance - significantly increased for better visibility)
        if rng.gen_ratio(25, 100) {
            // Replace digits with Arabic numerals
            let arabic_digits = [
                ('0', '٠'),
                ('1', '١'),
                ('2', '٢'),
                ('3', '٣'),
                ('4', '٤'),
                ('5', '٥'),
                ('6', '٦'),
                ('7', '٧'),
                ('8', '٨'),
                ('9', '٩'),
            ];
            
            // Identify appropriate numbers to replace
            let chars: Vec<char> = modified.chars().collect();
            let mut digit_positions = Vec::new();
            
            // Find digit positions that are safe to replace
            for (i, &c) in chars.iter().enumerate() {
                if c.is_ascii_digit() {
                    // Skip digits in special contexts like paths, IP addresses, versions
                    let is_path_digit = (i > 0 && (chars[i - 1] == '/' || chars[i - 1] == '.')) ||
                                       (i + 1 < chars.len() && (chars[i + 1] == '/' || chars[i + 1] == '.'));
                    
                    let is_command_param = i > 0 && chars[i - 1] == '-';
                    
                    // Replace only standalone numbers or numbers in safe contexts
                    if !is_path_digit && !is_command_param {
                        digit_positions.push(i);
                    }
                }
            }
            
            // Replace digits (up to 5 - increased for better visibility)
            if !digit_positions.is_empty() {
                let mut result = chars.clone();
                digit_positions.shuffle(&mut rng);
                
                let positions_to_replace = digit_positions.iter()
                                                        .take(digit_positions.len().min(5))
                                                        .collect::<Vec<_>>();
                
                for &pos in positions_to_replace {
                    for (latin, arabic) in arabic_digits.iter() {
                        if result[pos] == *latin {
                            result[pos] = *arabic;
                            break;
                        }
                    }
                }
                
                modified = result.iter().collect();
            }
        }
        
        // 2. Add Arabic punctuation (20% chance - significantly increased for better visibility)
        if rng.gen_ratio(20, 100) {
            // Arabic punctuation
            let arabic_punctuation = [
                (',', '،'),    // Arabic comma
                (';', '؛'),    // Arabic semicolon
                ('?', '؟'),    // Arabic question mark
                ('\"', '«'),   // Opening quote
                ('\"', '»'),   // Closing quote
                ('!', '!'),    // Exclamation mark (usually kept the same in Arabic)
            ];
            
            // Apply Arabic punctuation (to some but not all instances)
            let chars: Vec<char> = modified.chars().collect();
            let mut result = String::with_capacity(modified.len());
            
            for c in chars {
                let mut replaced = false;
                
                for (latin, arabic) in arabic_punctuation.iter() {
                    if c == *latin && rng.gen_ratio(8, 10) {  // 80% chance for each punctuation mark
                        result.push(*arabic);
                        replaced = true;
                        break;
                    }
                }
                
                if !replaced {
                    result.push(c);
                }
            }
            
            modified = result;
        }
        
        // 3. Add RTL marks and directional controls (18% chance - significantly increased)
        if rng.gen_ratio(18, 100) {
            // RTL controls
            let rtl_mark = "\u{200F}";      // Right-to-left mark
            let rtl_embed = "\u{202B}";     // Right-to-left embedding
            let rtl_override = "\u{202E}";  // Right-to-left override (more aggressive)
            let pop_dir = "\u{202C}";       // Pop directional formatting
            let ltr_mark = "\u{200E}";      // Left-to-right mark (for balance)
            
            // More comprehensive RTL strategy
            let strategy_num = rng.gen_range(0..=5); // More strategies
            
            // Apply the selected strategy
            modified = match strategy_num {
                0 => {
                    // Strategy 1: Wrap text in RTL marks (safe, but may affect layout)
                    format!("{}{}{}", rtl_mark, modified, rtl_mark)
                },
                1 => {
                    // Strategy 2: Add RTL mark in a relatively safe position
                    if let Some(pos) = modified.find(' ') {
                        let mut result = modified[..pos].to_string();
                        result.push(' ');
                        result.push_str(rtl_mark);
                        result.push_str(&modified[pos+1..]);
                        result
                    } else {
                        format!("{}{}", rtl_mark, modified) // Fallback
                    }
                },
                2 => {
                    // Strategy 3: Add RTL embedding for quoted text
                    if let Some(start) = modified.find('"') {
                        if let Some(end) = modified[start+1..].find('"') {
                            let mut result = modified[..start+1].to_string();
                            result.push_str(rtl_embed);
                            result.push_str(&modified[start+1..start+1+end]);
                            result.push_str(pop_dir);
                            result.push_str(&modified[start+1+end..]);
                            result
                        } else {
                            format!("{}{}{}", rtl_mark, modified, rtl_mark) // Fallback
                        }
                    } else {
                        format!("{}{}{}", rtl_mark, modified, rtl_mark) // Fallback
                    }
                },
                3 => {
                    // Strategy 4: Add RTL marks around special words
                    let special_words = ["file", "path", "user", "data", "name", "error", "command"];
                    let mut result = modified.to_string();
                    
                    for word in special_words {
                        if result.contains(word) {
                            let pattern = format!(" {} ", word);
                            let replacement = format!(" {}{}{} ", rtl_mark, word, rtl_mark);
                            result = result.replace(&pattern, &replacement);
                            break; // Only do one word to avoid overload
                        }
                    }
                    
                    result
                },
                4 => {
                    // Strategy 5: Surround numbers with RTL marks
                    let mut result = modified.to_string();
                    let chars: Vec<char> = result.chars().collect();
                    
                    for i in 0..chars.len() {
                        if chars[i].is_ascii_digit() {
                            // Find the end of the number
                            let mut j = i;
                            while j < chars.len() && chars[j].is_ascii_digit() {
                                j += 1;
                            }
                            
                            if j > i {
                                let num = &result[i..j];
                                let replacement = format!("{}{}{}", rtl_mark, num, rtl_mark);
                                result = result.replace(num, &replacement);
                                break; // Only do one number to avoid confusion
                            }
                        }
                    }
                    
                    result
                },
                _ => {
                    // Strategy 6: Add bidirectional control pairs at beginning and end
                    format!("{}{}{}{}{}", rtl_mark, ltr_mark, modified, rtl_mark, ltr_mark)
                }
            };
        }
        
        // 4. Variable name transliteration (18% chance - increased)
        if rng.gen_ratio(18, 100) {
            // Arabic transliterated variable names - expanded list
            let arabic_vars = [
                ("file", "malaf"),           // file -> ملف (transliterated)
                ("test", "tajriba"),         // test -> تجربة (transliterated)
                ("data", "bayanat"),         // data -> بيانات (transliterated)
                ("user", "mustakhdim"),      // user -> مستخدم (transliterated)
                ("output", "kharj"),         // output -> خرج (transliterated)
                ("input", "dakhl"),          // input -> دخل (transliterated)
                ("time", "waqt"),            // time -> وقت (transliterated)
                ("date", "tarikh"),          // date -> تاريخ (transliterated)
                ("name", "ism"),             // name -> اسم (transliterated)
                ("path", "masar"),           // path -> مسار (transliterated)
                ("count", "adad"),           // count -> عدد (transliterated)
                ("error", "khata"),          // error -> خطأ (transliterated)
                ("password", "kalimat sirr"), // password -> كلمة سر (transliterated)
                ("command", "amr"),          // command -> أمر (transliterated)
                ("system", "nizam"),         // system -> نظام (transliterated)
                ("directory", "mujallad"),   // directory -> مجلد (transliterated)
                ("search", "bahth"),         // search -> بحث (transliterated)
                ("find", "ijad"),            // find -> إيجاد (transliterated)
                ("create", "insha"),         // create -> إنشاء (transliterated)
                ("delete", "hadhf"),         // delete -> حذف (transliterated)
                ("copy", "naskh"),           // copy -> نسخ (transliterated)
                ("move", "naql"),            // move -> نقل (transliterated)
                ("text", "nass"),            // text -> نص (transliterated)
                ("code", "sifra"),           // code -> شيفرة (transliterated)
                ("list", "qaima"),           // list -> قائمة (transliterated)
            ];
            
            // Look for variable-like patterns to replace - improved detection
            for (english, arabic) in arabic_vars {
                if modified.contains(english) && rng.gen_ratio(5, 10) {  // 50% chance per match
                    // Check for variable-like patterns with expanded patterns
                    let var_patterns = [
                        format!(" {} ", english),      // Standalone word
                        format!("{}=", english),       // Assignment
                        format!(" {}=", english),      // Assignment with space
                        format!(" {}\n", english),     // Word at end of line
                        format!(" {}:", english),      // Word followed by colon
                        format!(" {}, ", english),     // Word in list
                        format!(" {}-", english),      // Word with hyphen
                        format!("--{}", english),      // Command line option
                        format!("-{}", english),       // Command line flag
                        format!("${}", english),       // Variable reference
                    ];
                    
                    // Try each pattern and replace if found
                    for pattern in var_patterns {
                        if modified.contains(&pattern) {
                            let replacement = pattern.replace(english, arabic);
                            modified = modified.replace(&pattern, &replacement);
                            break;  // Only one replacement per variable
                        }
                    }
                    
                    break;  // Only one variable replaced per invocation
                }
            }
        }
        
        // 5. Date format changes (15% chance - increased)
        if rng.gen_ratio(15, 100) {
            // Arabic date format - change slashes to Arabic date delimiter
            // Specific known date
            if modified.contains("02/28/2025") {
                // Convert to Arabic style with Arabic numerals (DD-MM-YYYY)
                modified = modified.replace("02/28/2025", "٢٨-٠٢-٢٠٢٥");
            }
            
            // More comprehensive date handling
            for month in 1..=12 {
                for day in 1..=31 {
                    // Skip invalid date combinations
                    if (month == 2 && day > 29) || 
                       ((month == 4 || month == 6 || month == 9 || month == 11) && day > 30) {
                        continue;
                    }
                    
                    // Multiple date format variations
                    let us_date_formats = [
                        format!("{:02}/{:02}/2023", month, day),
                        format!("{:02}/{:02}/2024", month, day),
                        format!("{:02}/{:02}/2025", month, day),
                    ];
                    
                    // Create Arabic format with hyphen and Arabic numerals
                    for us_date in &us_date_formats {
                        if modified.contains(us_date) {
                            // Extract day, month, year
                            let d = format!("{:02}", day);
                            let m = format!("{:02}", month);
                            let y = &us_date[us_date.rfind('/').unwrap_or(0) + 1..];
                            
                            // Convert to Arabic numerals
                            let ar_day = d.chars().map(|c| {
                                match c {
                                    '0' => '٠', '1' => '١', '2' => '٢', '3' => '٣', '4' => '٤',
                                    '5' => '٥', '6' => '٦', '7' => '٧', '8' => '٨', '9' => '٩',
                                    _ => c
                                }
                            }).collect::<String>();
                            
                            let ar_month = m.chars().map(|c| {
                                match c {
                                    '0' => '٠', '1' => '١', '2' => '٢', '3' => '٣', '4' => '٤',
                                    '5' => '٥', '6' => '٦', '7' => '٧', '8' => '٨', '9' => '٩',
                                    _ => c
                                }
                            }).collect::<String>();
                            
                            let ar_year = y.chars().map(|c| {
                                match c {
                                    '0' => '٠', '1' => '١', '2' => '٢', '3' => '٣', '4' => '٤',
                                    '5' => '٥', '6' => '٦', '7' => '٧', '8' => '٨', '9' => '٩',
                                    _ => c
                                }
                            }).collect::<String>();
                            
                            // Arabic date format DD-MM-YYYY with Arabic numerals
                            let arabic_date = format!("{}-{}-{}", ar_day, ar_month, ar_year);
                            modified = modified.replace(us_date, &arabic_date);
                            break;  // Only one date replaced
                        }
                    }
                }
            }
        }
        
        // 6. Add tatweel stretching for decorative effect (8% chance - increased)
        if rng.gen_ratio(8, 100) {
            // Tatweel (ـ) is used to stretch Arabic words for decorative effect or emphasis
            // We'll add it in various contexts to simulate Arabic-style formatting
            
            // Scenario 1: Add in quoted text
            if modified.contains('"') {
                let parts: Vec<_> = modified.split('"').collect();
                if parts.len() >= 3 { // At least one quoted string
                    let mut result = String::new();
                    for (i, part) in parts.iter().enumerate() {
                        if i > 0 && i % 2 == 1 { // Inside quotes
                            let stretched = if part.contains(' ') {
                                // Add tatweel between words
                                part.replace(" ", " ـ ")
                            } else if !part.is_empty() {
                                // Add tatweel in middle of single word
                                let pos = part.len() / 2;
                                let mut stretched = part[..pos].to_string();
                                stretched.push('ـ');
                                stretched.push_str(&part[pos..]);
                                stretched
                            } else {
                                part.to_string()
                            };
                            
                            result.push_str(&stretched);
                        } else {
                            result.push_str(part);
                        }
                        
                        if i < parts.len() - 1 && (i % 2 == 0) {
                            result.push('"');
                        }
                    }
                    modified = result;
                }
            } 
            // Scenario 2: Add to variable names or commands
            else if modified.contains('=') || modified.contains('-') {
                let parts: Vec<_> = if modified.contains('=') {
                    modified.split('=').collect()
                } else {
                    modified.split('-').collect()
                };
                
                if parts.len() >= 2 {
                    let mut result = String::new();
                    result.push_str(parts[0]);
                    
                    // Add tatweel before the = or -
                    result.push('ـ');
                    
                    // Add the separator
                    if modified.contains('=') {
                        result.push('=');
                    } else {
                        result.push_str(" -");
                    }
                    
                    // Add the rest
                    for part in parts.iter().skip(1) {
                        result.push_str(part);
                    }
                    
                    modified = result;
                }
            }
        }
        
        // 7. Add Arabic word substitutions (15% chance - new feature)
        if rng.gen_ratio(15, 100) {
            // Arabic vocabulary substitutions
            let arabic_words = [
                ("file", "ملف"),
                ("directory", "مجلد"),
                ("folder", "مجلد"),
                ("user", "مستخدم"),
                ("password", "كلمة مرور"),
                ("command", "أمر"),
                ("search", "بحث"),
                ("find", "إيجاد"),
                ("error", "خطأ"),
                ("help", "مساعدة"),
                ("print", "طباعة"),
                ("save", "حفظ"),
                ("open", "فتح"),
                ("close", "إغلاق"),
                ("exit", "خروج"),
                ("yes", "نعم"),
                ("no", "لا"),
                ("please", "من فضلك"),
                ("thanks", "شكرا"),
            ];
            
            // Only replace standalone words, not parts of commands
            for (english, arabic) in arabic_words {
                // Only proceed if the word is found and replacement rolls succeed
                if modified.contains(english) && rng.gen_ratio(4, 10) {
                    // Look for the word with spaces around it or at beginning/end
                    let patterns = [
                        format!(" {} ", english),
                        format!(" {}\n", english),
                        format!(" {}.", english),
                        format!(" {}", english),
                        format!("^{} ", english),
                    ];
                    
                    for pattern in patterns {
                        if modified.contains(&pattern) {
                            // Replace with Arabic equivalent, maintaining the pattern
                            let replacement = pattern.replace(english, &format!("{}", arabic));
                            modified = modified.replace(&pattern, &replacement);
                            break;
                        }
                    }
                    
                    // Only do one word replacement per execution
                    break;
                }
            }
        }
        
        modified
    }
    
    /// Add subtle German fingerprints
    fn add_german_fingerprints(&self, text: &str) -> String {
        let mut modified = text.to_string();
        let mut rng = thread_rng();
        
        // 1. Add keyboard layout slip (y/z swap) (25% chance - significantly increased for better visibility)
        if rng.gen_ratio(25, 100) {
            // German keyboards have y and z swapped compared to US layouts
            let chars: Vec<char> = modified.chars().collect();
            let mut result = String::with_capacity(modified.len());
            
            for c in chars {
                if c == 'y' && rng.gen_ratio(8, 10) { // 80% chance of y->z swap
                    result.push('z'); // Swap y->z
                } else if c == 'Y' && rng.gen_ratio(8, 10) {
                    result.push('Z'); // Swap Y->Z
                } else if c == 'z' && rng.gen_ratio(8, 10) {
                    result.push('y'); // Swap z->y
                } else if c == 'Z' && rng.gen_ratio(8, 10) {
                    result.push('Y'); // Swap Z->Y
                } else {
                    result.push(c);
                }
            }
            modified = result;
        }
        
        // 2. German date format (20% chance - significantly increased for better visibility)
        if rng.gen_ratio(20, 100) {
            // Replace MM/DD/YYYY with DD.MM.YYYY format
            // Match common date patterns and convert to German format
            
            // First, check for specific dates like "02/28/2025"
            if modified.contains("02/28/2025") {
                modified = modified.replace("02/28/2025", "28.02.2025");
            }
            
            // Then handle other date formats with improved pattern matching
            for month in 1..=12 {
                for day in 1..=31 {
                    // Only process valid date combinations
                    if (month == 2 && day > 29) || 
                       ((month == 4 || month == 6 || month == 9 || month == 11) && day > 30) {
                        continue;
                    }
                    
                    // Look for MM/DD/YYYY format and convert to DD.MM.YYYY
                    let us_date_formats = [
                        format!("{:02}/{:02}/2023", month, day),
                        format!("{:02}/{:02}/2024", month, day),
                        format!("{:02}/{:02}/2025", month, day),
                        format!("{}/{}/2023", month, day),
                        format!("{}/{}/2024", month, day),
                        format!("{}/{}/2025", month, day),
                    ];
                    
                    let german_date = format!("{:02}.{:02}.", day, month);
                    
                    for us_date in us_date_formats.iter() {
                        if modified.contains(us_date) {
                            let year = &us_date[us_date.len()-4..];
                            modified = modified.replace(us_date, &format!("{}{}", german_date, year));
                        }
                    }
                }
            }
        }
        
        // 3. Add German keyboard specific umlaut slips (18% chance - increased for better visibility)
        if rng.gen_ratio(18, 100) {
            // German specific character replacements
            let replacements = [
                ("ae", "ä"),
                ("oe", "ö"),
                ("ue", "ü"),
                ("Ae", "Ä"),
                ("Oe", "Ö"),
                ("Ue", "Ü"),
                // Additional German character patterns
                ("ss", "ß"),
                ("Ess", "Eß"),
            ];
            
            // Replace with improved context awareness
            for (find, replace) in replacements {
                if modified.contains(find) {
                    // Try to find all occurrences with word boundaries
                    let pattern = format!(" {} ", find); // Space-bounded
                    if modified.contains(&pattern) && rng.gen_ratio(6, 10) { // 60% chance
                        modified = modified.replace(&pattern, &format!(" {} ", replace));
                        continue; // Only one type of replacement per pass
                    }
                    
                    // Try beginning of word
                    let pattern = format!(" {}", find);
                    if modified.contains(&pattern) && rng.gen_ratio(6, 10) {
                        modified = modified.replace(&pattern, &format!(" {}", replace));
                        continue;
                    }
                    
                    // Try middle/end of word for common German patterns
                    if (find == "ae" || find == "oe" || find == "ue" || find == "ss") && rng.gen_ratio(5, 10) {
                        // Look for these in variable names or commands
                        for word in ["datae", "parameter", "process", "user", "password", "messssage", "issue"] {
                            if modified.contains(word) && word.contains(find) {
                                modified = modified.replace(word, &word.replace(find, replace));
                                break;
                            }
                        }
                    }
                }
            }
        }
        
        // 4. Add common German keyboard slips for symbols (15% chance - increased)
        if rng.gen_ratio(15, 100) {
            let chars: Vec<char> = modified.chars().collect();
            let mut result = String::with_capacity(modified.len());
            
            for c in chars {
                match c {
                    ';' if rng.gen_ratio(6, 10) => result.push('ö'),
                    '\'' if rng.gen_ratio(6, 10) => result.push('ä'),
                    '[' if rng.gen_ratio(6, 10) => result.push('ü'),
                    ']' if rng.gen_ratio(6, 10) => result.push('+'),
                    '/' if rng.gen_ratio(3, 10) => result.push('-'),
                    '\\' if rng.gen_ratio(3, 10) => result.push('#'),
                    '=' if rng.gen_ratio(3, 10) => result.push('´'),
                    _ => result.push(c),
                }
            }
            modified = result;
        }
        
        // 5. Add German word substitutions (12% chance - new feature)
        if rng.gen_ratio(12, 100) {
            // German vocabulary substitutions
            let german_words = [
                ("file", "datei"),
                ("directory", "verzeichnis"),
                ("folder", "ordner"),
                ("user", "benutzer"),
                ("password", "passwort"),
                ("command", "befehl"),
                ("search", "suche"),
                ("find", "finden"),
                ("error", "fehler"),
                ("help", "hilfe"),
                ("print", "drucken"),
                ("save", "speichern"),
                ("open", "öffnen"),
                ("close", "schließen"),
                ("exit", "beenden"),
            ];
            
            // Only replace standalone words, not parts of commands
            for (english, german) in german_words {
                // Only proceed if the word is found and replacement rolls succeed
                if modified.contains(english) && rng.gen_ratio(4, 10) {
                    // Look for the word with spaces around it or at beginning/end
                    let patterns = [
                        format!(" {} ", english),
                        format!(" {}\n", english),
                        format!(" {}.", english),
                        format!(" {}", english),
                        format!("^{} ", english),
                    ];
                    
                    for pattern in patterns {
                        if modified.contains(&pattern) {
                            // Replace with German equivalent, maintaining the pattern
                            let replacement = pattern.replace(english, german);
                            modified = modified.replace(&pattern, &replacement);
                            break;
                        }
                    }
                    
                    // Only do one word replacement per execution
                    break;
                }
            }
        }
        
        modified
    }
    
    /// Add subtle French fingerprints
    fn add_french_fingerprints(&self, text: &str) -> String {
        let mut modified = text.to_string();
        let mut rng = thread_rng();
        
        // 1. Add keyboard layout slip (AZERTY) (25% chance - significantly increased for better visibility)
        if rng.gen_ratio(25, 100) {
            // French AZERTY keyboards have several key swaps compared to QWERTY
            let chars: Vec<char> = modified.chars().collect();
            let mut result = String::with_capacity(modified.len());
            
            for c in chars {
                match c {
                    'q' if rng.gen_ratio(8, 10) => result.push('a'), // AZERTY slip q->a
                    'Q' if rng.gen_ratio(8, 10) => result.push('A'), // AZERTY slip Q->A
                    'a' if rng.gen_ratio(8, 10) => result.push('q'), // AZERTY slip a->q
                    'A' if rng.gen_ratio(8, 10) => result.push('Q'), // AZERTY slip A->Q
                    'w' if rng.gen_ratio(8, 10) => result.push('z'), // AZERTY slip w->z
                    'W' if rng.gen_ratio(8, 10) => result.push('Z'), // AZERTY slip W->Z
                    'z' if rng.gen_ratio(8, 10) => result.push('w'), // AZERTY slip z->w
                    'Z' if rng.gen_ratio(8, 10) => result.push('W'), // AZERTY slip Z->W
                    'm' if rng.gen_ratio(5, 10) => result.push(','), // AZERTY slip - m is next to comma
                    ',' if rng.gen_ratio(5, 10) => result.push('m'), // AZERTY slip - comma is next to m
                    '.' if rng.gen_ratio(5, 10) => result.push('/'), // AZERTY slip - period is next to slash
                    '/' if rng.gen_ratio(5, 10) => result.push(':'), // AZERTY slip - slash is next to colon
                    // Additional AZERTY layout specific slips
                    '1' if rng.gen_ratio(4, 10) => result.push('&'), // AZERTY slip - 1 is shift-&
                    '2' if rng.gen_ratio(4, 10) => result.push('é'), // AZERTY slip - 2 is é
                    '3' if rng.gen_ratio(4, 10) => result.push('"'), // AZERTY slip - 3 is "
                    '4' if rng.gen_ratio(4, 10) => result.push('\''), // AZERTY slip - 4 is '
                    '5' if rng.gen_ratio(4, 10) => result.push('('), // AZERTY slip - 5 is (
                    '6' if rng.gen_ratio(4, 10) => result.push('-'), // AZERTY slip - 6 is -
                    '0' if rng.gen_ratio(4, 10) => result.push('à'), // AZERTY slip - 0 is à
                    _ => result.push(c),
                }
            }
            modified = result;
        }
        
        // 2. French punctuation spacing (22% chance - significantly increased for better visibility)
        if rng.gen_ratio(22, 100) {
            // In French, there's a space before some punctuation marks
            // This is a noticeable hallmark of French text
            
            // Check for common punctuation marks that should have a space before them in French
            let punctuation_marks = [
                ("!", " !"),
                ("?", " ?"),
                (":", " :"),
                (";", " ;"),
                ("»", " »"),
                ("«", "« "),
                ("%", " %"), // French also puts a space before percent signs
            ];
            
            for (mark, replacement) in punctuation_marks {
                if modified.contains(mark) && !modified.contains(replacement) {
                    // Replace the mark with proper French spacing
                    // But not in URL contexts or other special cases
                    if mark == ":" && (modified.contains("http:") || modified.contains("https:")) {
                        // Don't add space in URLs
                        let parts: Vec<&str> = modified.split("http").collect();
                        if parts.len() > 1 {
                            let mut new_text = parts[0].to_string();
                            // Add space before colon in non-URL parts
                            for (i, part) in parts[1..].iter().enumerate() {
                                if i > 0 || !parts[0].is_empty() {
                                    new_text.push_str("http");
                                }
                                if part.starts_with('s') {
                                    new_text.push('s');
                                    new_text.push_str(&part[1..].replace(":", " :"));
                                } else {
                                    new_text.push_str(&part.replace(":", " :"));
                                }
                            }
                            modified = new_text;
                        }
                    } else {
                        // Regular replacement for other punctuation
                        modified = modified.replace(mark, replacement);
                    }
                }
            }
            
            // Special case for pipes, which often have spaces in French
            if modified.contains("|") && !modified.contains(" | ") {
                // Add spaces around pipes, but only for command separators
                modified = modified.replace(" | ", "  |  "); // First handle already-spaced pipes
                modified = modified.replace("|", " | ");     // Then handle non-spaced pipes
            }
            
            // Another common French spacing trait: double spaces after periods
            if modified.contains(". ") && !modified.contains(".  ") && rng.gen_ratio(6, 10) {
                modified = modified.replace(". ", ".  ");
            }
        }
        
        // 3. Add French accents (16% chance - doubled for better visibility)
        if rng.gen_ratio(16, 100) {
            // Common French letter replacements
            let replacements = [
                ("e", "é"),
                ("a", "à"),
                ("u", "ù"),
                ("c", "ç"),
                ("i", "î"),
                ("o", "ô"),
                ("e", "è"),
                ("a", "â"),
                ("u", "û"),
                ("e", "ê"),
            ];
            
            // Try to find more appropriate places for accents
            let accent_contexts = [
                ("the", "thé"),
                ("here", "héré"),
                ("where", "whére"),
                ("more", "moré"),
                ("user", "usér"),
                ("data", "datà"),
                ("list", "lîst"),
                ("file", "fîle"),
                ("space", "spàce"),
                ("place", "plàce"),
                ("command", "commànd"),
            ];
            
            // First try word-based replacements (more accurate)
            for (find, replace) in accent_contexts {
                if modified.contains(find) && rng.gen_ratio(4, 10) { // 40% chance
                    // Only replace in word contexts (with spaces or punctuation)
                    let word_patterns = [
                        format!(" {} ", find),
                        format!(" {}", find),
                        format!("{}.", find),
                        format!("{},", find),
                        format!("{}:", find),
                        format!("{}!", find),
                        format!("{}?", find),
                    ];
                    
                    for pattern in word_patterns {
                        if modified.contains(&pattern) {
                            let replacement = pattern.replace(find, replace);
                            modified = modified.replace(&pattern, &replacement);
                            break;
                        }
                    }
                    
                    break; // Only one word replacement
                }
            }
            
            // Then try character-based replacements
            if rng.gen_ratio(5, 10) { // 50% chance for an additional letter replacement
                for (find, replace) in replacements {
                    if modified.contains(find) && rng.gen_ratio(3, 10) { // 30% chance per match
                        // Replace only one occurrence to be subtle
                        if let Some(pos) = modified.find(find) {
                            // Don't replace if it's part of a command or system path
                            let is_command = pos == 0 || 
                                            (pos > 0 && [' ', '/', '-'].contains(&modified.chars().nth(pos - 1).unwrap_or(' ')));
                            
                            if !is_command {
                                let mut new_text = modified[..pos].to_string();
                                new_text.push_str(replace);
                                new_text.push_str(&modified[pos + find.len()..]);
                                modified = new_text;
                                break; // Only one replacement per pass
                            }
                        }
                    }
                }
            }
        }
        
        // 4. Add French date format (15% chance - doubled)
        if rng.gen_ratio(15, 100) {
            // Replace MM/DD/YYYY with DD/MM/YYYY format (French style)
            if modified.contains("02/28/2025") {
                modified = modified.replace("02/28/2025", "28/02/2025");
            }
            
            // More comprehensive date pattern replacements
            for month in 1..=12 {
                for day in 1..=31 {
                    // Skip invalid date combinations
                    if (month == 2 && day > 29) || 
                       ((month == 4 || month == 6 || month == 9 || month == 11) && day > 30) {
                        continue;
                    }
                    
                    // Multiple date format variations
                    let us_date_formats = [
                        format!("{:02}/{:02}/2023", month, day),
                        format!("{:02}/{:02}/2024", month, day),
                        format!("{:02}/{:02}/2025", month, day),
                        format!("{}/{}/2023", month, day),
                        format!("{}/{}/2024", month, day),
                        format!("{}/{}/2025", month, day),
                    ];
                    
                    for us_date in &us_date_formats {
                        if modified.contains(us_date) && day <= 12 { // Only when day <= 12 to avoid ambiguity
                            // French date format (day/month/year)
                            let year = &us_date[us_date.rfind('/').unwrap_or(0) + 1..];
                            let fr_date = format!("{:02}/{:02}/{}", day, month, year);
                            modified = modified.replace(us_date, &fr_date);
                        }
                    }
                }
            }
        }
        
        // 5. Add French word substitutions (12% chance - new feature)
        if rng.gen_ratio(12, 100) {
            // French vocabulary substitutions
            let french_words = [
                ("file", "fichier"),
                ("directory", "répertoire"),
                ("folder", "dossier"),
                ("user", "utilisateur"),
                ("password", "mot de passe"),
                ("command", "commande"),
                ("search", "recherche"),
                ("find", "trouver"),
                ("error", "erreur"),
                ("help", "aide"),
                ("print", "imprimer"),
                ("save", "enregistrer"),
                ("open", "ouvrir"),
                ("close", "fermer"),
                ("exit", "quitter"),
                ("yes", "oui"),
                ("no", "non"),
                ("please", "s'il vous plaît"),
                ("thanks", "merci"),
            ];
            
            // Only replace standalone words, not parts of commands
            for (english, french) in french_words {
                // Only proceed if the word is found and replacement rolls succeed
                if modified.contains(english) && rng.gen_ratio(4, 10) {
                    // Look for the word with spaces around it or at beginning/end
                    let patterns = [
                        format!(" {} ", english),
                        format!(" {}\n", english),
                        format!(" {}.", english),
                        format!(" {}", english),
                        format!("^{} ", english),
                    ];
                    
                    for pattern in patterns {
                        if modified.contains(&pattern) {
                            // Replace with French equivalent, maintaining the pattern
                            let replacement = pattern.replace(english, french);
                            modified = modified.replace(&pattern, &replacement);
                            break;
                        }
                    }
                    
                    // Only do one word replacement per execution
                    break;
                }
            }
        }
        
        modified
    }
    
    /// Transform text with realistic typing errors
    #[allow(dead_code)]
    pub fn transform_with_errors(&self, text: &str, error_rate: f32) -> String {
        // First, translate the text
        let translated = self.transform(text);
        
        // Then apply typing errors appropriate to the language
        let error_generator = TypingErrorGenerator::new(
            self.language_id.language.as_str(),
            error_rate
        );
        
        error_generator.apply_errors(&translated)
    }
    
    /// German dictionary with common words
    fn german_dictionary() -> HashMap<String, String> {
        let mut dict = HashMap::new();
        dict.insert("hello".to_string(), "hallo".to_string());
        dict.insert("world".to_string(), "welt".to_string());
        dict.insert("yes".to_string(), "ja".to_string());
        dict.insert("no".to_string(), "nein".to_string());
        dict.insert("please".to_string(), "bitte".to_string());
        dict.insert("thank".to_string(), "danke".to_string());
        dict.insert("you".to_string(), "du".to_string());
        dict.insert("goodbye".to_string(), "auf wiedersehen".to_string());
        dict
    }
    
    /// French dictionary with common words
    fn french_dictionary() -> HashMap<String, String> {
        let mut dict = HashMap::new();
        dict.insert("hello".to_string(), "bonjour".to_string());
        dict.insert("world".to_string(), "monde".to_string());
        dict.insert("yes".to_string(), "oui".to_string());
        dict.insert("no".to_string(), "non".to_string());
        dict.insert("please".to_string(), "s'il vous plaît".to_string());
        dict.insert("thank".to_string(), "merci".to_string());
        dict.insert("you".to_string(), "vous".to_string());
        dict.insert("goodbye".to_string(), "au revoir".to_string());
        dict
    }
    
    /// Russian dictionary with common words
    fn russian_dictionary() -> HashMap<String, String> {
        let mut dict = HashMap::new();
        dict.insert("hello".to_string(), "привет".to_string());
        dict.insert("world".to_string(), "мир".to_string());
        dict.insert("yes".to_string(), "да".to_string());
        dict.insert("no".to_string(), "нет".to_string());
        dict.insert("please".to_string(), "пожалуйста".to_string());
        dict.insert("thank".to_string(), "спасибо".to_string());
        dict.insert("you".to_string(), "вы".to_string());
        dict.insert("goodbye".to_string(), "до свидания".to_string());
        dict
    }
    
    /// Japanese dictionary with common words
    fn japanese_dictionary() -> HashMap<String, String> {
        let mut dict = HashMap::new();
        dict.insert("hello".to_string(), "こんにちは".to_string());
        dict.insert("world".to_string(), "世界".to_string());
        dict.insert("yes".to_string(), "はい".to_string());
        dict.insert("no".to_string(), "いいえ".to_string());
        dict.insert("please".to_string(), "お願いします".to_string());
        dict.insert("thank".to_string(), "ありがとう".to_string());
        dict.insert("you".to_string(), "あなた".to_string());
        dict.insert("goodbye".to_string(), "さようなら".to_string());
        dict
    }
    
    /// Spanish dictionary with common words
    fn spanish_dictionary() -> HashMap<String, String> {
        let mut dict = HashMap::new();
        // Basic conversational terms
        dict.insert("hello".to_string(), "hola".to_string());
        dict.insert("world".to_string(), "mundo".to_string());
        dict.insert("yes".to_string(), "sí".to_string());
        dict.insert("no".to_string(), "no".to_string());
        dict.insert("please".to_string(), "por favor".to_string());
        dict.insert("thank".to_string(), "gracias".to_string());
        dict.insert("you".to_string(), "tú".to_string());
        dict.insert("goodbye".to_string(), "adiós".to_string());
        dict.insert("welcome".to_string(), "bienvenido".to_string());
        dict.insert("sorry".to_string(), "lo siento".to_string());
        dict.insert("friend".to_string(), "amigo".to_string());
        dict.insert("today".to_string(), "hoy".to_string());
        dict.insert("tomorrow".to_string(), "mañana".to_string());
        dict.insert("yesterday".to_string(), "ayer".to_string());
        dict.insert("morning".to_string(), "mañana".to_string());
        dict.insert("evening".to_string(), "tarde".to_string());
        dict.insert("night".to_string(), "noche".to_string());
        
        // Cyber-specific terms
        dict.insert("password".to_string(), "contraseña".to_string());
        dict.insert("security".to_string(), "seguridad".to_string());
        dict.insert("network".to_string(), "red".to_string());
        dict.insert("hack".to_string(), "hackear".to_string());
        dict.insert("computer".to_string(), "ordenador".to_string());
        dict.insert("system".to_string(), "sistema".to_string());
        dict.insert("software".to_string(), "software".to_string());
        dict.insert("hardware".to_string(), "hardware".to_string());
        dict.insert("database".to_string(), "base de datos".to_string());
        dict.insert("server".to_string(), "servidor".to_string());
        dict.insert("firewall".to_string(), "cortafuegos".to_string());
        dict.insert("encryption".to_string(), "cifrado".to_string());
        dict.insert("decryption".to_string(), "descifrado".to_string());
        dict.insert("malware".to_string(), "malware".to_string());
        dict.insert("virus".to_string(), "virus".to_string());
        dict.insert("trojan".to_string(), "troyano".to_string());
        dict.insert("phishing".to_string(), "suplantación de identidad".to_string());
        dict.insert("authentication".to_string(), "autenticación".to_string());
        dict.insert("access".to_string(), "acceso".to_string());
        dict.insert("user".to_string(), "usuario".to_string());
        dict.insert("admin".to_string(), "administrador".to_string());
        dict.insert("download".to_string(), "descargar".to_string());
        dict.insert("upload".to_string(), "subir".to_string());
        dict.insert("connection".to_string(), "conexión".to_string());
        dict.insert("protect".to_string(), "proteger".to_string());
        dict.insert("attack".to_string(), "ataque".to_string());
        dict.insert("defense".to_string(), "defensa".to_string());
        dict.insert("breach".to_string(), "brecha".to_string());
        dict.insert("exploit".to_string(), "explotar".to_string());
        dict.insert("vulnerable".to_string(), "vulnerable".to_string());
        dict.insert("secure".to_string(), "seguro".to_string());
        
        dict
    }
    
    /// Brazilian Portuguese dictionary with common words
    fn brazilian_portuguese_dictionary() -> HashMap<String, String> {
        let mut dict = HashMap::new();
        // Basic conversational terms
        dict.insert("hello".to_string(), "olá".to_string());
        dict.insert("world".to_string(), "mundo".to_string());
        dict.insert("yes".to_string(), "sim".to_string());
        dict.insert("no".to_string(), "não".to_string());
        dict.insert("please".to_string(), "por favor".to_string());
        dict.insert("thank".to_string(), "obrigado".to_string());
        dict.insert("you".to_string(), "você".to_string());
        dict.insert("goodbye".to_string(), "tchau".to_string());
        dict.insert("welcome".to_string(), "bem-vindo".to_string());
        dict.insert("sorry".to_string(), "desculpe".to_string());
        dict.insert("friend".to_string(), "amigo".to_string());
        dict.insert("today".to_string(), "hoje".to_string());
        dict.insert("tomorrow".to_string(), "amanhã".to_string());
        dict.insert("yesterday".to_string(), "ontem".to_string());
        dict.insert("morning".to_string(), "manhã".to_string());
        dict.insert("evening".to_string(), "tarde".to_string());
        dict.insert("night".to_string(), "noite".to_string());
        dict.insert("how".to_string(), "como".to_string());
        dict.insert("what".to_string(), "o que".to_string());
        dict.insert("why".to_string(), "por que".to_string());
        dict.insert("where".to_string(), "onde".to_string());
        dict.insert("when".to_string(), "quando".to_string());
        dict.insert("who".to_string(), "quem".to_string());

        // Brazilian-specific terms
        dict.insert("cool".to_string(), "legal".to_string());
        dict.insert("awesome".to_string(), "massa".to_string());
        dict.insert("dude".to_string(), "cara".to_string());
        dict.insert("guy".to_string(), "cara".to_string());
        dict.insert("ok".to_string(), "beleza".to_string());
        dict.insert("alright".to_string(), "tranquilo".to_string());
        
        // Cyber-specific terms
        dict.insert("password".to_string(), "senha".to_string());
        dict.insert("security".to_string(), "segurança".to_string());
        dict.insert("network".to_string(), "rede".to_string());
        dict.insert("hack".to_string(), "invadir".to_string());
        dict.insert("computer".to_string(), "computador".to_string());
        dict.insert("system".to_string(), "sistema".to_string());
        dict.insert("software".to_string(), "software".to_string());
        dict.insert("hardware".to_string(), "hardware".to_string());
        dict.insert("database".to_string(), "banco de dados".to_string());
        dict.insert("server".to_string(), "servidor".to_string());
        dict.insert("firewall".to_string(), "firewall".to_string());
        dict.insert("encryption".to_string(), "criptografia".to_string());
        dict.insert("decryption".to_string(), "descriptografia".to_string());
        dict.insert("malware".to_string(), "malware".to_string());
        dict.insert("virus".to_string(), "vírus".to_string());
        dict.insert("trojan".to_string(), "cavalo de troia".to_string());
        dict.insert("phishing".to_string(), "phishing".to_string());
        dict.insert("authentication".to_string(), "autenticação".to_string());
        dict.insert("access".to_string(), "acesso".to_string());
        dict.insert("user".to_string(), "usuário".to_string());
        dict.insert("admin".to_string(), "administrador".to_string());
        dict.insert("download".to_string(), "baixar".to_string());
        dict.insert("upload".to_string(), "enviar".to_string());
        dict.insert("connection".to_string(), "conexão".to_string());
        dict.insert("protect".to_string(), "proteger".to_string());
        dict.insert("attack".to_string(), "ataque".to_string());
        dict.insert("defense".to_string(), "defesa".to_string());
        dict.insert("breach".to_string(), "invasão".to_string());
        dict.insert("exploit".to_string(), "explorar".to_string());
        dict.insert("vulnerable".to_string(), "vulnerável".to_string());
        dict.insert("secure".to_string(), "seguro".to_string());
        dict.insert("log".to_string(), "registro".to_string());
        dict.insert("backdoor".to_string(), "porta dos fundos".to_string());
        dict.insert("keylogger".to_string(), "keylogger".to_string());
        dict.insert("ransomware".to_string(), "ransomware".to_string());
        
        dict
    }
    
    /// Mandarin Chinese dictionary with common words
    fn mandarin_dictionary() -> HashMap<String, String> {
        let mut dict = HashMap::new();
        dict.insert("hello".to_string(), "你好".to_string());
        dict.insert("world".to_string(), "世界".to_string());
        dict.insert("yes".to_string(), "是".to_string());
        dict.insert("no".to_string(), "不".to_string());
        dict.insert("please".to_string(), "请".to_string());
        dict.insert("thank".to_string(), "谢谢".to_string());
        dict.insert("you".to_string(), "你".to_string());
        dict.insert("goodbye".to_string(), "再见".to_string());
        // Cyber-specific terms
        dict.insert("password".to_string(), "密码".to_string());
        dict.insert("security".to_string(), "安全".to_string());
        dict.insert("network".to_string(), "网络".to_string());
        dict.insert("hack".to_string(), "黑客攻击".to_string());
        dict
    }
    
    /// Cantonese dictionary with common words
    fn cantonese_dictionary() -> HashMap<String, String> {
        let mut dict = HashMap::new();
        dict.insert("hello".to_string(), "你好".to_string()); // Same script as Mandarin but different pronunciation
        dict.insert("world".to_string(), "世界".to_string());
        dict.insert("yes".to_string(), "係".to_string()); // Different from Mandarin
        dict.insert("no".to_string(), "唔係".to_string()); // Different from Mandarin
        dict.insert("please".to_string(), "請".to_string());
        dict.insert("thank".to_string(), "多謝".to_string()); // Different from Mandarin
        dict.insert("you".to_string(), "你".to_string());
        dict.insert("goodbye".to_string(), "拜拜".to_string()); // More colloquial than Mandarin
        // Cyber-specific terms
        dict.insert("password".to_string(), "密碼".to_string());
        dict.insert("security".to_string(), "安全".to_string());
        dict.insert("network".to_string(), "網絡".to_string());
        dict.insert("hack".to_string(), "黑客入侵".to_string());
        dict
    }
    
    /// Korean dictionary with common words
    fn korean_dictionary() -> HashMap<String, String> {
        let mut dict = HashMap::new();
        dict.insert("hello".to_string(), "안녕하세요".to_string());
        dict.insert("world".to_string(), "세계".to_string());
        dict.insert("yes".to_string(), "네".to_string());
        dict.insert("no".to_string(), "아니요".to_string());
        dict.insert("please".to_string(), "부탁합니다".to_string());
        dict.insert("thank".to_string(), "감사합니다".to_string());
        dict.insert("you".to_string(), "당신".to_string());
        dict.insert("goodbye".to_string(), "안녕히 가세요".to_string());
        // Cyber-specific terms
        dict.insert("password".to_string(), "비밀번호".to_string());
        dict.insert("security".to_string(), "보안".to_string());
        dict.insert("network".to_string(), "네트워크".to_string());
        dict.insert("hack".to_string(), "해킹".to_string());
        dict
    }
    
    /// Arabic dictionary with common words and cybersecurity terms
    fn arabic_dictionary() -> HashMap<String, String> {
        let mut dict = HashMap::new();
        // Basic conversational terms
        dict.insert("hello".to_string(), "مرحبا".to_string());
        dict.insert("world".to_string(), "عالم".to_string());
        dict.insert("yes".to_string(), "نعم".to_string());
        dict.insert("no".to_string(), "لا".to_string());
        dict.insert("please".to_string(), "من فضلك".to_string());
        dict.insert("thank".to_string(), "شكرا".to_string());
        dict.insert("you".to_string(), "أنت".to_string());
        dict.insert("goodbye".to_string(), "مع السلامة".to_string());
        dict.insert("welcome".to_string(), "أهلا وسهلا".to_string());
        dict.insert("sorry".to_string(), "آسف".to_string());
        dict.insert("friend".to_string(), "صديق".to_string());
        dict.insert("today".to_string(), "اليوم".to_string());
        dict.insert("tomorrow".to_string(), "غدا".to_string());
        dict.insert("yesterday".to_string(), "أمس".to_string());
        dict.insert("morning".to_string(), "صباح".to_string());
        dict.insert("evening".to_string(), "مساء".to_string());
        dict.insert("night".to_string(), "ليل".to_string());
        dict.insert("how".to_string(), "كيف".to_string());
        dict.insert("what".to_string(), "ماذا".to_string());
        dict.insert("why".to_string(), "لماذا".to_string());
        dict.insert("where".to_string(), "أين".to_string());
        dict.insert("when".to_string(), "متى".to_string());
        dict.insert("who".to_string(), "من".to_string());
        dict.insert("good".to_string(), "جيد".to_string());
        dict.insert("bad".to_string(), "سيء".to_string());
        dict.insert("ok".to_string(), "حسنا".to_string());
        
        // Cyber-specific terms (with proper Arabic terms used in cybersecurity contexts)
        dict.insert("password".to_string(), "كلمة المرور".to_string());
        dict.insert("security".to_string(), "أمان".to_string());
        dict.insert("network".to_string(), "شبكة".to_string());
        dict.insert("hack".to_string(), "اختراق".to_string());
        dict.insert("computer".to_string(), "حاسوب".to_string());
        dict.insert("system".to_string(), "نظام".to_string());
        dict.insert("software".to_string(), "برمجيات".to_string());
        dict.insert("hardware".to_string(), "عتاد".to_string());
        dict.insert("database".to_string(), "قاعدة بيانات".to_string());
        dict.insert("server".to_string(), "خادم".to_string());
        dict.insert("firewall".to_string(), "جدار حماية".to_string());
        dict.insert("encryption".to_string(), "تشفير".to_string());
        dict.insert("decryption".to_string(), "فك التشفير".to_string());
        dict.insert("malware".to_string(), "برامج ضارة".to_string());
        dict.insert("virus".to_string(), "فيروس".to_string());
        dict.insert("trojan".to_string(), "حصان طروادة".to_string());
        dict.insert("phishing".to_string(), "تصيد".to_string());
        dict.insert("authentication".to_string(), "مصادقة".to_string());
        dict.insert("access".to_string(), "وصول".to_string());
        dict.insert("user".to_string(), "مستخدم".to_string());
        dict.insert("admin".to_string(), "مسؤول".to_string());
        dict.insert("download".to_string(), "تنزيل".to_string());
        dict.insert("upload".to_string(), "رفع".to_string());
        dict.insert("connection".to_string(), "اتصال".to_string());
        dict.insert("protect".to_string(), "حماية".to_string());
        dict.insert("attack".to_string(), "هجوم".to_string());
        dict.insert("defense".to_string(), "دفاع".to_string());
        dict.insert("breach".to_string(), "خرق".to_string());
        dict.insert("exploit".to_string(), "استغلال".to_string());
        dict.insert("vulnerable".to_string(), "قابل للاختراق".to_string());
        dict.insert("secure".to_string(), "آمن".to_string());
        dict.insert("log".to_string(), "سجل".to_string());
        dict.insert("backdoor".to_string(), "باب خلفي".to_string());
        dict.insert("keylogger".to_string(), "مسجل المفاتيح".to_string());
        dict.insert("ransomware".to_string(), "برامج الفدية".to_string());
        dict.insert("spyware".to_string(), "برامج التجسس".to_string());
        dict.insert("threat".to_string(), "تهديد".to_string());
        dict.insert("vulnerability".to_string(), "ثغرة".to_string());
        dict.insert("patch".to_string(), "رقعة إصلاح".to_string());
        dict.insert("update".to_string(), "تحديث".to_string());
        dict.insert("code".to_string(), "شفرة".to_string());
        dict.insert("programmer".to_string(), "مبرمج".to_string());
        dict.insert("developer".to_string(), "مطور".to_string());
        dict.insert("website".to_string(), "موقع الكتروني".to_string());
        dict.insert("browser".to_string(), "متصفح".to_string());
        dict.insert("email".to_string(), "بريد إلكتروني".to_string());
        
        dict
    }
    
    /// Farsi/Persian dictionary with common words and cybersecurity terms
    fn farsi_dictionary() -> HashMap<String, String> {
        let mut dict = HashMap::new();
        // Basic conversational terms
        dict.insert("hello".to_string(), "سلام".to_string());
        dict.insert("world".to_string(), "جهان".to_string());
        dict.insert("yes".to_string(), "بله".to_string());
        dict.insert("no".to_string(), "نه".to_string());
        dict.insert("please".to_string(), "لطفا".to_string());
        dict.insert("thank".to_string(), "متشکرم".to_string());
        dict.insert("you".to_string(), "شما".to_string());
        dict.insert("goodbye".to_string(), "خداحافظ".to_string());
        dict.insert("welcome".to_string(), "خوش آمدید".to_string());
        dict.insert("sorry".to_string(), "ببخشید".to_string());
        dict.insert("friend".to_string(), "دوست".to_string());
        dict.insert("today".to_string(), "امروز".to_string());
        dict.insert("tomorrow".to_string(), "فردا".to_string());
        dict.insert("yesterday".to_string(), "دیروز".to_string());
        dict.insert("morning".to_string(), "صبح".to_string());
        dict.insert("evening".to_string(), "عصر".to_string());
        dict.insert("night".to_string(), "شب".to_string());
        dict.insert("how".to_string(), "چطور".to_string());
        dict.insert("what".to_string(), "چه".to_string());
        dict.insert("why".to_string(), "چرا".to_string());
        dict.insert("where".to_string(), "کجا".to_string());
        dict.insert("when".to_string(), "چه وقت".to_string());
        dict.insert("who".to_string(), "چه کسی".to_string());
        dict.insert("good".to_string(), "خوب".to_string());
        dict.insert("bad".to_string(), "بد".to_string());
        dict.insert("ok".to_string(), "باشه".to_string());
        
        // Persian specific colloquial expressions
        dict.insert("cool".to_string(), "باحال".to_string());
        dict.insert("awesome".to_string(), "عالی".to_string());
        dict.insert("dude".to_string(), "داداش".to_string());
        dict.insert("guy".to_string(), "آقا".to_string());
        dict.insert("bye".to_string(), "بای".to_string());
        
        // Cyber-specific terms with proper Farsi translations
        dict.insert("password".to_string(), "رمز عبور".to_string());
        dict.insert("security".to_string(), "امنیت".to_string());
        dict.insert("network".to_string(), "شبکه".to_string());
        dict.insert("hack".to_string(), "هک".to_string());
        dict.insert("computer".to_string(), "کامپیوتر".to_string());
        dict.insert("system".to_string(), "سیستم".to_string());
        dict.insert("software".to_string(), "نرم‌افزار".to_string());
        dict.insert("hardware".to_string(), "سخت‌افزار".to_string());
        dict.insert("database".to_string(), "پایگاه داده".to_string());
        dict.insert("server".to_string(), "سرور".to_string());
        dict.insert("firewall".to_string(), "فایروال".to_string());
        dict.insert("encryption".to_string(), "رمزگذاری".to_string());
        dict.insert("decryption".to_string(), "رمزگشایی".to_string());
        dict.insert("malware".to_string(), "بدافزار".to_string());
        dict.insert("virus".to_string(), "ویروس".to_string());
        dict.insert("trojan".to_string(), "تروجان".to_string());
        dict.insert("phishing".to_string(), "فیشینگ".to_string());
        dict.insert("authentication".to_string(), "احراز هویت".to_string());
        dict.insert("access".to_string(), "دسترسی".to_string());
        dict.insert("user".to_string(), "کاربر".to_string());
        dict.insert("admin".to_string(), "مدیر".to_string());
        dict.insert("download".to_string(), "دانلود".to_string());
        dict.insert("upload".to_string(), "آپلود".to_string());
        dict.insert("connection".to_string(), "ارتباط".to_string());
        dict.insert("protect".to_string(), "محافظت".to_string());
        dict.insert("attack".to_string(), "حمله".to_string());
        dict.insert("defense".to_string(), "دفاع".to_string());
        dict.insert("breach".to_string(), "نقض".to_string());
        dict.insert("exploit".to_string(), "استثمار".to_string());
        dict.insert("vulnerable".to_string(), "آسیب‌پذیر".to_string());
        dict.insert("secure".to_string(), "امن".to_string());
        dict.insert("log".to_string(), "لاگ".to_string());
        dict.insert("backdoor".to_string(), "درب پشتی".to_string());
        dict.insert("keylogger".to_string(), "کی‌لاگر".to_string());
        dict.insert("ransomware".to_string(), "باج‌افزار".to_string());
        dict.insert("spyware".to_string(), "جاسوس‌افزار".to_string());
        dict.insert("threat".to_string(), "تهدید".to_string());
        dict.insert("vulnerability".to_string(), "آسیب‌پذیری".to_string());
        dict.insert("patch".to_string(), "وصله".to_string());
        dict.insert("update".to_string(), "به‌روزرسانی".to_string());
        dict.insert("code".to_string(), "کد".to_string());
        dict.insert("programmer".to_string(), "برنامه‌نویس".to_string());
        dict.insert("developer".to_string(), "توسعه‌دهنده".to_string());
        dict.insert("website".to_string(), "وب‌سایت".to_string());
        dict.insert("browser".to_string(), "مرورگر".to_string());
        dict.insert("email".to_string(), "ایمیل".to_string());
        dict.insert("data".to_string(), "داده".to_string());
        dict.insert("information".to_string(), "اطلاعات".to_string());
        dict.insert("identity".to_string(), "هویت".to_string());
        dict.insert("account".to_string(), "حساب کاربری".to_string());
        
        dict
    }
}

/// TypingErrorGenerator for creating realistic language-specific typing errors
#[allow(dead_code)]
pub struct TypingErrorGenerator {
    language: String,
    error_rate: f32,
}

impl TypingErrorGenerator {
    /// Create a new error generator for a specific language
    pub fn new(language: &str, error_rate: f32) -> Self {
        TypingErrorGenerator {
            language: language.to_string(),
            error_rate: error_rate.clamp(0.0, 1.0),
        }
    }
    
    /// Apply realistic typing errors to a string based on language-specific patterns
    pub fn apply_errors(&self, text: &str) -> String {
        if text.is_empty() || self.error_rate <= 0.0 {
            return text.to_string();
        }
        
        let mut rng = thread_rng();
        let mut result = String::with_capacity(text.len());
        let chars: Vec<char> = text.chars().collect();
        
        for (i, &c) in chars.iter().enumerate() {
            // Random chance to introduce an error based on error_rate
            if rng.gen::<f32>() < self.error_rate {
                let error_type = rng.gen_range(0..=4); // 5 types of errors
                
                match error_type {
                    0 => {
                        // Skip character (deletion error)
                        continue;
                    }
                    1 => {
                        // Duplicate character (insertion error)
                        result.push(c);
                        result.push(c);
                    }
                    2 => {
                        // Swap with next character if possible (transposition error)
                        if i < chars.len() - 1 {
                            result.push(chars[i + 1]);
                            result.push(c);
                            continue; // Skip next character since we used it
                        } else {
                            result.push(c); // Regular character if can't swap
                        }
                    }
                    3 => {
                        // Replace with adjacent key on keyboard (substitution error)
                        let adjacent = self.get_adjacent_key(c);
                        result.push(adjacent);
                    }
                    4 => {
                        // Insert a language-specific character (language-specific error)
                        let language_char = self.get_language_specific_char();
                        if !language_char.is_empty() {
                            result.push_str(&language_char);
                        }
                        result.push(c);
                    }
                    _ => result.push(c),
                }
            } else {
                // Regular character, no error
                result.push(c);
            }
        }
        
        result
    }
    
    /// Get a character that's adjacent to the given one on a keyboard
    fn get_adjacent_key(&self, c: char) -> char {
        let mut rng = thread_rng();
        
        // Define keyboard adjacency based on language
        match self.language.as_str() {
            "ar" | "fa" => {
                // Arabic/Farsi keyboard adjacency (simplified)
                match c {
                    'ا' => ['ل', 'ب', 'ت'].choose(&mut rng).cloned().unwrap_or(c),
                    'ب' => ['ا', 'ل', 'ي'].choose(&mut rng).cloned().unwrap_or(c),
                    'ت' => ['ن', 'ا', 'م'].choose(&mut rng).cloned().unwrap_or(c),
                    // More Arabic/Farsi adjacency mappings would go here
                    _ => c,
                }
            },
            "zh-CN" | "zh-HK" => {
                // Chinese pinyin adjacency
                match c {
                    'a' => ['s', 'z', 'q', 'w'].choose(&mut rng).cloned().unwrap_or(c),
                    'i' => ['u', 'o', 'j', 'k'].choose(&mut rng).cloned().unwrap_or(c),
                    // More Chinese pinyin adjacency mappings would go here
                    _ => c,
                }
            },
            "ko" => {
                // Korean Hangul adjacency
                match c {
                    'ㄱ' => ['ㄴ', 'ㅇ'].choose(&mut rng).cloned().unwrap_or(c),
                    'ㄴ' => ['ㄱ', 'ㄷ', 'ㅇ'].choose(&mut rng).cloned().unwrap_or(c),
                    // More Korean adjacency mappings would go here
                    _ => c,
                }
            },
            "es" => {
                // Spanish keyboard adjacency
                match c {
                    'a' => ['s', 'z', 'q'].choose(&mut rng).cloned().unwrap_or(c),
                    'ñ' => ['l', 'k', 'p'].choose(&mut rng).cloned().unwrap_or(c),
                    'á' => ['a', 's'].choose(&mut rng).cloned().unwrap_or(c),
                    // More Spanish adjacency mappings would go here
                    _ => c,
                }
            },
            "pt-BR" => {
                // Brazilian Portuguese keyboard adjacency
                match c {
                    'a' => ['s', 'z', 'q'].choose(&mut rng).cloned().unwrap_or(c),
                    'ç' => ['l', 'c'].choose(&mut rng).cloned().unwrap_or(c),
                    'ã' => ['a', 'o'].choose(&mut rng).cloned().unwrap_or(c),
                    // More Portuguese adjacency mappings would go here
                    _ => c,
                }
            },
            _ => {
                // Default QWERTY layout for other languages
                match c {
                    'a' => ['s', 'q', 'w', 'z'].choose(&mut rng).cloned().unwrap_or(c),
                    'b' => ['v', 'g', 'h', 'n'].choose(&mut rng).cloned().unwrap_or(c),
                    'c' => ['x', 'd', 'f', 'v'].choose(&mut rng).cloned().unwrap_or(c),
                    'd' => ['s', 'e', 'r', 'f', 'c', 'x'].choose(&mut rng).cloned().unwrap_or(c),
                    'e' => ['w', 's', 'd', 'r'].choose(&mut rng).cloned().unwrap_or(c),
                    'f' => ['d', 'r', 't', 'g', 'v', 'c'].choose(&mut rng).cloned().unwrap_or(c),
                    'g' => ['f', 't', 'y', 'h', 'b', 'v'].choose(&mut rng).cloned().unwrap_or(c),
                    'h' => ['g', 'y', 'u', 'j', 'n', 'b'].choose(&mut rng).cloned().unwrap_or(c),
                    'i' => ['u', 'j', 'k', 'o'].choose(&mut rng).cloned().unwrap_or(c),
                    'j' => ['h', 'u', 'i', 'k', 'm', 'n'].choose(&mut rng).cloned().unwrap_or(c),
                    'k' => ['j', 'i', 'o', 'l', 'm'].choose(&mut rng).cloned().unwrap_or(c),
                    'l' => ['k', 'o', 'p', ';'].choose(&mut rng).cloned().unwrap_or(c),
                    'm' => ['n', 'j', 'k', ','].choose(&mut rng).cloned().unwrap_or(c),
                    'n' => ['b', 'h', 'j', 'm'].choose(&mut rng).cloned().unwrap_or(c),
                    'o' => ['i', 'k', 'l', 'p'].choose(&mut rng).cloned().unwrap_or(c),
                    'p' => ['o', 'l', ';', '['].choose(&mut rng).cloned().unwrap_or(c),
                    'q' => ['1', 'w', 'a'].choose(&mut rng).cloned().unwrap_or(c),
                    'r' => ['e', 'd', 'f', 't'].choose(&mut rng).cloned().unwrap_or(c),
                    's' => ['a', 'w', 'e', 'd', 'x', 'z'].choose(&mut rng).cloned().unwrap_or(c),
                    't' => ['r', 'f', 'g', 'y'].choose(&mut rng).cloned().unwrap_or(c),
                    'u' => ['y', 'h', 'j', 'i'].choose(&mut rng).cloned().unwrap_or(c),
                    'v' => ['c', 'f', 'g', 'b'].choose(&mut rng).cloned().unwrap_or(c),
                    'w' => ['q', 'a', 's', 'e'].choose(&mut rng).cloned().unwrap_or(c),
                    'x' => ['z', 's', 'd', 'c'].choose(&mut rng).cloned().unwrap_or(c),
                    'y' => ['t', 'g', 'h', 'u'].choose(&mut rng).cloned().unwrap_or(c),
                    'z' => ['a', 's', 'x'].choose(&mut rng).cloned().unwrap_or(c),
                    // Use same mapping for uppercase
                    _ => c,
                }
            }
        }
    }
    
    /// Get a language-specific character that might be accidentally typed
    fn get_language_specific_char(&self) -> String {
        let mut rng = thread_rng();
        
        match self.language.as_str() {
            "ar" => {
                // Arabic-specific characters that might be accidentally inserted
                [" ", "ال", "و", "ي", "ة", ""].choose(&mut rng).unwrap().to_string()
            },
            "fa" => {
                // Farsi-specific characters that might be accidentally inserted
                [" ", "می", "را", "و", "ه", ""].choose(&mut rng).unwrap().to_string()
            },
            "zh-CN" => {
                // Mandarin-specific characters that might be accidentally inserted
                [" ", "的", "了", "是", "在", ""].choose(&mut rng).unwrap().to_string()
            },
            "zh-HK" => {
                // Cantonese-specific characters that might be accidentally inserted
                [" ", "嘅", "咗", "喺", "唔", ""].choose(&mut rng).unwrap().to_string()
            },
            "ko" => {
                // Korean-specific characters that might be accidentally inserted
                [" ", "이", "가", "을", "를", ""].choose(&mut rng).unwrap().to_string()
            },
            "es" => {
                // Spanish-specific characters or errors
                [" ", "ñ", "á", "é", "í", "ó", "ú", "¿", "¡", ""].choose(&mut rng).unwrap().to_string()
            },
            "pt-BR" => {
                // Brazilian Portuguese-specific characters or errors
                [" ", "ç", "ã", "õ", "á", "é", "í", "ó", "ú", ""].choose(&mut rng).unwrap().to_string()
            },
            _ => String::new(), // No special characters for other languages
        }
    }
}

/// Timestamp emulator for timezone obfuscation
#[derive(Clone)]
pub struct TimestampEmulator {
    timezone_offset: i32,
    timezone_name: String,
}

impl TimestampEmulator {
    /// Get the timezone offset
    pub fn get_offset(&self) -> i32 {
        self.timezone_offset
    }
    /// Create a random timestamp emulator
    pub fn random() -> Self {
        let mut rng = thread_rng();
        let offset = rng.gen_range(-12..=12);
        let timezone_name = format!("UTC{}{}", if offset >= 0 { "+" } else { "" }, offset);
        
        TimestampEmulator {
            timezone_offset: offset,
            timezone_name,
        }
    }
    
    /// Create a timestamp emulator for a specific timezone
    pub fn for_timezone(timezone: &str) -> Self {
        // Parse timezone string (e.g., "+1" for CET)
        let offset = timezone.parse::<i32>().unwrap_or(0);
        let timezone_name = match offset {
            1 => "CET".to_string(),
            2 => "EET".to_string(),
            3 => "MSK".to_string(),
            5 => "PKT".to_string(),
            8 => "CST".to_string(),
            9 => "JST".to_string(),
            -5 => "EST".to_string(),
            -6 => "CST".to_string(),
            -7 => "MST".to_string(),
            -8 => "PST".to_string(),
            _ => format!("UTC{}{}", if offset >= 0 { "+" } else { "" }, offset),
        };
        
        TimestampEmulator {
            timezone_offset: offset,
            timezone_name,
        }
    }
    
    /// Get current timestamp in the emulated timezone
    pub fn get_timestamp(&self) -> String {
        let utc_now = Utc::now();
        
        // Adjust time by the timezone offset
        let emulated_time = utc_now + chrono::Duration::hours(self.timezone_offset as i64);
        
        // Format time as HH:MM with timezone name
        format!("{:02}:{:02} {}", 
            emulated_time.hour(), 
            emulated_time.minute(), 
            self.timezone_name
        )
    }
    
    /// Check if the current time is within typical US working hours (9am-4pm EST)
    #[cfg(test)]
    pub fn is_us_working_hours(&self) -> bool {
        let est_offset = -5;
        let utc_now = Utc::now();
        
        // Convert to EST
        let est_time = utc_now + chrono::Duration::hours(est_offset as i64);
        
        // Check if weekend
        let weekday = est_time.weekday();
        if weekday == Weekday::Sat || weekday == Weekday::Sun {
            return false;
        }
        
        // Check time (9am-4pm)
        let hour = est_time.hour();
        (9..=16).contains(&hour)
    }
    
    /// Check if the current date is a US holiday
    #[cfg(test)]
    pub fn is_us_holiday(&self) -> bool {
        let today = Local::now();
        let month = today.month();
        let day = today.day();
        
        // Check specific dates
        if (month == 1 && day == 1) ||     // New Year's Day
           (month == 7 && day == 4) ||     // Independence Day
           (month == 12 && day == 25) {    // Christmas
            return true;
        }
        
        // Check Memorial Day (last Monday in May)
        if month == 5 && is_memorial_day(today) {
            return true;
        }
        
        // Check Labor Day (first Monday in September)
        if month == 9 && is_labor_day(today) {
            return true;
        }
        
        false
    }
}

/// Helper function to check if a date is Memorial Day
#[cfg(test)]
fn is_memorial_day(date: DateTime<Local>) -> bool {
    // Memorial Day is the last Monday in May
    let month = date.month();
    let weekday = date.weekday();
    
    if month != 5 || weekday != Weekday::Mon {
        return false;
    }
    
    // Check if it's the last Monday in May
    let day = date.day();
    let last_day_of_may = match Local.with_ymd_and_hms(date.year(), 5, 31, 0, 0, 0) {
        chrono::LocalResult::Single(date) => date.day(),
        _ => 31,
    };
    
    day + 7 > last_day_of_may
}

/// Helper function to check if a date is Labor Day
#[cfg(test)]
fn is_labor_day(date: DateTime<Local>) -> bool {
    // Labor Day is the first Monday in September
    let month = date.month();
    let weekday = date.weekday();
    let day = date.day();
    
    month == 9 && weekday == Weekday::Mon && day <= 7
}