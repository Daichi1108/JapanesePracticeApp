use yew::prelude::*;
use web_sys::HtmlInputElement;
use rand::Rng;
use std::collections::HashMap;

fn init_kana_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Normal hiragana
    map.insert("あ", "a");
    map.insert("い", "i");
    map.insert("う", "u");
    map.insert("え", "e");
    map.insert("お", "o");
    map.insert("か", "ka");
    map.insert("き", "ki");
    map.insert("く", "ku");
    map.insert("け", "ke");
    map.insert("こ", "ko");
    map.insert("さ", "sa");
    map.insert("し", "shi");
    map.insert("す", "su");
    map.insert("せ", "se");
    map.insert("そ", "so");
    map.insert("た", "ta");
    map.insert("ち", "chi");
    map.insert("つ", "tsu");
    map.insert("て", "te");
    map.insert("と", "to");
    map.insert("な", "na");
    map.insert("に", "ni");
    map.insert("ぬ", "nu");
    map.insert("ね", "ne");
    map.insert("の", "no");
    map.insert("は", "ha");
    map.insert("ひ", "hi");
    map.insert("ふ", "fu");
    map.insert("へ", "he");
    map.insert("ほ", "ho");
    map.insert("ま", "ma");
    map.insert("み", "mi");
    map.insert("む", "mu");
    map.insert("め", "me");
    map.insert("も", "mo");
    map.insert("や", "ya");
    map.insert("ゆ", "yu");
    map.insert("よ", "yo");
    map.insert("ら", "ra");
    map.insert("り", "ri");
    map.insert("る", "ru");
    map.insert("れ", "re");
    map.insert("ろ", "ro");
    map.insert("わ", "wa");
    map.insert("を", "wo");
    map.insert("ん", "n");

    // Dakuon (hiragana with dakuten)
    map.insert("が", "ga");
    map.insert("ぎ", "gi");
    map.insert("ぐ", "gu");
    map.insert("げ", "ge");
    map.insert("ご", "go");
    map.insert("ざ", "za");
    map.insert("じ", "ji");
    map.insert("ず", "zu");
    map.insert("ぜ", "ze");
    map.insert("ぞ", "zo");
    map.insert("だ", "da");
    map.insert("ぢ", "ji");
    map.insert("づ", "zu");
    map.insert("で", "de");
    map.insert("ど", "do");

    // Combo (hiragana with combination)
    map.insert("きゃ", "kya");
    map.insert("きゅ", "kyu");
    map.insert("きょ", "kyo");
    map.insert("しゃ", "sha");
    map.insert("しゅ", "shu");
    map.insert("しょ", "sho");
    map.insert("ちゃ", "cha");
    map.insert("ちゅ", "chu");
    map.insert("ちょ", "cho");
    map.insert("にゃ", "nya");
    map.insert("にゅ", "nyu");
    map.insert("にょ", "nyo");
    map.insert("ひゃ", "hya");
    map.insert("ひゅ", "hyu");
    map.insert("ひょ", "hyo");
    map.insert("みゃ", "mya");
    map.insert("みゅ", "myu");
    map.insert("みょ", "myo");
    map.insert("りゃ", "rya");
    map.insert("りゅ", "ryu");
    map.insert("りょ", "ryo");

    // Small (hiragana with small kana)
    map.insert("ぁ", "a");
    map.insert("ぃ", "i");
    map.insert("ぅ", "u");
    map.insert("ぇ", "e");
    map.insert("ぉ", "o");
    map.insert("っ", "tsu");

    // Long (hiragana with long vowel)
    map.insert("あー", "a");
    map.insert("いー", "i");
    map.insert("うー", "u");
    map.insert("えー", "e");
    map.insert("おー", "o");

    // Normal katakana
    map.insert("ア", "a");
    map.insert("イ", "i");
    map.insert("ウ", "u");
    map.insert("エ", "e");
    map.insert("オ", "o");
    map.insert("カ", "ka");
    map.insert("キ", "ki");
    map.insert("ク", "ku");
    map.insert("ケ", "ke");
    map.insert("コ", "ko");
    map.insert("サ", "sa");
    map.insert("シ", "shi");
    map.insert("ス", "su");
    map.insert("セ", "se");
    map.insert("ソ", "so");
    map.insert("タ", "ta");
    map.insert("チ", "chi");
    map.insert("ツ", "tsu");
    map.insert("テ", "te");
    map.insert("ト", "to");
    map.insert("ナ", "na");
    map.insert("ニ", "ni");
    map.insert("ヌ", "nu");
    map.insert("ネ", "ne");
    map.insert("ノ", "no");
    map.insert("ハ", "ha");
    map.insert("ヒ", "hi");
    map.insert("フ", "fu");
    map.insert("ヘ", "he");
    map.insert("ホ", "ho");
    map.insert("マ", "ma");
    map.insert("ミ", "mi");
    map.insert("ム", "mu");
    map.insert("メ", "me");
    map.insert("モ", "mo");
    map.insert("ヤ", "ya");
    map.insert("ユ", "yu");
    map.insert("ヨ", "yo");
    map.insert("ラ", "ra");
    map.insert("リ", "ri");
    map.insert("ル", "ru");
    map.insert("レ", "re");
    map.insert("ロ", "ro");
    map.insert("ワ", "wa");
    map.insert("ヲ", "wo");
    map.insert("ン", "n");

    // Dakuon (katakana with dakuten)
    map.insert("ガ", "ga");
    map.insert("ギ", "gi");
    map.insert("グ", "gu");
    map.insert("ゲ", "ge");
    map.insert("ゴ", "go");
    map.insert("ザ", "za");
    map.insert("ジ", "ji");
    map.insert("ズ", "zu");
    map.insert("ゼ", "ze");
    map.insert("ゾ", "zo");
    map.insert("ダ", "da");
    map.insert("ヂ", "ji");
    map.insert("ヅ", "zu");
    map.insert("デ", "de");
    map.insert("ド", "do");

    // Combo (katakana with combination)
    map.insert("キャ", "kya");
    map.insert("キュ", "kyu");
    map.insert("キョ", "kyo");
    map.insert("シャ", "sha");
    map.insert("シュ", "shu");
    map.insert("ショ", "sho");
    map.insert("チャ", "cha");
    map.insert("チュ", "chu");
    map.insert("チョ", "cho");
    map.insert("ニャ", "nya");
    map.insert("ニュ", "nyu");
    map.insert("ニョ", "nyo");
    map.insert("ヒャ", "hya");
    map.insert("ヒュ", "hyu");
    map.insert("ヒョ", "hyo");
    map.insert("ミャ", "mya");
    map.insert("ミュ", "myu");
    map.insert("ミョ", "myo");
    map.insert("リャ", "rya");
    map.insert("リュ", "ryu");
    map.insert("リョ", "ryo");

    // Small (katakana with small kana)
    map.insert("ァ", "a");
    map.insert("ィ", "i");
    map.insert("ゥ", "u");
    map.insert("ェ", "e");
    map.insert("ォ", "o");
    map.insert("ッ", "tsu");

    // Long (katakana with long vowel)
    map.insert("アー", "a");
    map.insert("イー", "i");
    map.insert("ウー", "u");
    map.insert("エー", "e");
    map.insert("オー", "o");
    map
}

fn main() {
    yew::start_app::<RootComponent>();
}

fn getword(types:[bool; 10]) -> String {

    let hiragana_normal: Vec<&str> = vec![
    "あ", "い", "う", "え", "お", "か", "き", "く", "け", "こ",
    "さ", "し", "す", "せ", "そ", "た", "ち", "つ", "て", "と",
    "な", "に", "ぬ", "ね", "の", "は", "ひ", "ふ", "へ", "ほ",
    "ま", "み", "む", "め", "も", "や", "ゆ", "よ", "ら", "り",
    "る", "れ", "ろ", "わ", "を", "ん",
    ];

    // Dakuon (hiragana with dakuten)
    let hiragana_dakuon: Vec<&str> = vec![
        "が", "ぎ", "ぐ", "げ", "ご", "ざ", "じ", "ず", "ぜ", "ぞ",
        "だ", "ぢ", "づ", "で", "ど",
    ];

    // Combo (hiragana with combination)
    let hiragana_combo: Vec<&str> = vec![
        "きゃ", "きゅ", "きょ", "しゃ", "しゅ", "しょ",
        "ちゃ", "ちゅ", "ちょ", "にゃ", "にゅ", "にょ",
        "ひゃ", "ひゅ", "ひょ", "みゃ", "みゅ", "みょ",
        "りゃ", "りゅ", "りょ",
    ];

    // Small (hiragana with small kana)
    let hiragana_small: Vec<&str> = vec![
        "ぁ", "ぃ", "ぅ", "ぇ", "ぉ", "っ",
    ];

    // Long (hiragana with long vowel)
    let hiragana_long: Vec<&str> = vec![
        "あー", "いー", "うー", "えー", "おー",
    ];

    // Normal katakana
    let katakana_normal: Vec<&str> = vec![
        "ア", "イ", "ウ", "エ", "オ", "カ", "キ", "ク", "ケ", "コ",
        "サ", "シ", "ス", "セ", "ソ", "タ", "チ", "ツ", "テ", "ト",
        "ナ", "ニ", "ヌ", "ネ", "ノ", "ハ", "ヒ", "フ", "ヘ", "ホ",
        "マ", "ミ", "ム", "メ", "モ", "ヤ", "ユ", "ヨ", "ラ", "リ",
        "ル", "レ", "ロ", "ワ", "ヲ", "ン",
    ];

    // Dakuon (katakana with dakuten)
    let katakana_dakuon: Vec<&str> = vec![
        "ガ", "ギ", "グ", "ゲ", "ゴ", "ザ", "ジ", "ズ", "ゼ", "ゾ",
        "ダ", "ヂ", "ヅ", "デ", "ド",
    ];

    // Combo (katakana with combination)
    let katakana_combo: Vec<&str> = vec![
        "キャ", "キュ", "キョ", "シャ", "シュ", "ショ",
        "チャ", "チュ", "チョ", "ニャ", "ニュ", "ニョ",
        "ヒャ", "ヒュ", "ヒョ", "ミャ", "ミュ", "ミョ",
        "リャ", "リュ", "リョ",
    ];

    // Small (katakana with small kana)
    let katakana_small: Vec<&str> = vec![
        "ァ", "ィ", "ゥ", "ェ", "ォ", "ッ",
    ];

    // Long (katakana with long vowel)
    let katakana_long: Vec<&str> = vec![
        "アー", "イー", "ウー", "エー", "オー",
    ];

    let mut kanalist:Vec<&str> = Vec::new();
    if types[0] {
        kanalist.extend(hiragana_normal);
    }
    if types[1] {
        kanalist.extend(hiragana_dakuon);
    }
    if types[2] {
        kanalist.extend(hiragana_combo);
    }
    if types[3] {
        kanalist.extend(hiragana_small);
    }
    if types[4] {
        kanalist.extend(hiragana_long);
    }
    if types[5] {
        kanalist.extend(katakana_normal);
    }
    if types[6] {
        kanalist.extend(katakana_dakuon);
    }
    if types[7] {
        kanalist.extend(katakana_combo);
    }
    if types[8] {
        kanalist.extend(katakana_small);
    }
    if types[9] {
        kanalist.extend(katakana_long);
    }

    let mut rng = rand::thread_rng();
    let randomnum = rng.gen_range(0..kanalist.len());
    return kanalist[randomnum].to_string();

}

enum Msg {//send msg
    Input(String),
    Guess,
    Toggle0,
    Toggle1,
    Toggle2,
    Toggle3,
    Toggle4,
    Toggle5,
    Toggle6,
    Toggle7,
    Toggle8,
    Toggle9,
    None
}

struct RootComponent {//things
    input:String,
    kana:String,
    response:String,
    types:[bool; 10]
}

impl Component for RootComponent {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self { //constructor
            input:String::new(),
            types:[true,false,false,false,false,false,false,false,false,false],
            kana:getword([true,false,false,false,false,false,false,false,false,false]),
            response:String::new()
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {//get message
        match msg {
            Msg::Input(inputstring) => {
                self.input = inputstring;
                true
            }
            Msg::Guess => {
                let kana_map = init_kana_map();
                if kana_map.get(self.kana.as_str()).unwrap().to_string() == self.input {
                    self.response = "YAY".to_string();
                }
                else {
                    self.response = "U SUCK".to_string();
                }
                self.input = String::new();
                self.kana = getword(self.types);
                true
            }
            Msg::Toggle0 => {
                self.types[0] = !self.types[0];
                true
            }
            Msg::Toggle1 => {
                self.types[1] = !self.types[1];
                true
            }
            Msg::Toggle2 => {
                self.types[2] = !self.types[2];
                true
            }
            Msg::Toggle3 => {
                self.types[3] = !self.types[3];
                true
            }
            Msg::Toggle4 => {
                self.types[4] = !self.types[4];
                true
            }
            Msg::Toggle5 => {
                self.types[5] = !self.types[5];
                true
            }
            Msg::Toggle6 => {
                self.types[6] = !self.types[6];
                true
            }
            Msg::Toggle7 => {
                self.types[7] = !self.types[7];
                true
            }
            Msg::Toggle8 => {
                self.types[8] = !self.types[8];
                true
            }
            Msg::Toggle9 => {
                self.types[9] = !self.types[9];
                true
            }
            Msg::None => {
                false
            }
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let link = _ctx.link();
        html! {
            <div>
                <h1>{"Kana Practice App"}</h1>
                <div class = "kanadisplay">{self.kana.clone()}</div>
                <input class = "input" type = "text"
                oninput = {link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value())})}
                onkeypress={link.callback(|key:KeyboardEvent| {if key.char_code()==13 {Msg::Guess} else{Msg::None}})}
                value = {self.input.clone()}/>
                <div class = "responsedisplay">{self.response.clone()}</div>
                <div class = "buttons">
                    <div>
                        <button class = {self.types[0].to_string()} onclick = {link.callback(|_|Msg::Toggle0)}>{"Hiragana"}</button>
                        <button class = {self.types[1].to_string()} onclick = {link.callback(|_|Msg::Toggle1)}>{"Dakuon"}</button>
                        <button class = {self.types[2].to_string()} onclick = {link.callback(|_|Msg::Toggle2)}>{"Combo"}</button>
                        <button class = {self.types[3].to_string()} onclick = {link.callback(|_|Msg::Toggle3)}>{"Small"}</button>
                        <button class = {self.types[4].to_string()} onclick = {link.callback(|_|Msg::Toggle4)}>{"Long"}</button>
                    </div>
                    <div>
                        <button class = {self.types[5].to_string()} onclick = {link.callback(|_|Msg::Toggle5)}>{"Katakana"}</button>
                        <button class = {self.types[6].to_string()} onclick = {link.callback(|_|Msg::Toggle6)}>{"Dakuon"}</button>
                        <button class = {self.types[7].to_string()} onclick = {link.callback(|_|Msg::Toggle7)}>{"Combo"}</button>
                        <button class = {self.types[8].to_string()} onclick = {link.callback(|_|Msg::Toggle8)}>{"Small"}</button>
                        <button class = {self.types[9].to_string()} onclick = {link.callback(|_|Msg::Toggle9)}>{"Long"}</button>
                    </div>
                </div>
            </div>
        }
    }
}