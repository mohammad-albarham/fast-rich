//! Emoji name to Unicode character mapping.
//!
//! Maps shortcodes like `:smile:` to their Unicode equivalents.

use std::collections::HashMap;
use std::sync::LazyLock;

/// Static emoji map.
static EMOJI_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    // Smileys & Emotion
    m.insert("smile", "😄");
    m.insert("smiley", "😃");
    m.insert("grinning", "😀");
    m.insert("grin", "😁");
    m.insert("joy", "😂");
    m.insert("laughing", "😆");
    m.insert("sweat_smile", "😅");
    m.insert("rofl", "🤣");
    m.insert("wink", "😉");
    m.insert("blush", "😊");
    m.insert("innocent", "😇");
    m.insert("heart_eyes", "😍");
    m.insert("kissing_heart", "😘");
    m.insert("yum", "😋");
    m.insert("stuck_out_tongue", "😛");
    m.insert("thinking", "🤔");
    m.insert("shushing_face", "🤫");
    m.insert("raised_eyebrow", "🤨");
    m.insert("neutral_face", "😐");
    m.insert("expressionless", "😑");
    m.insert("no_mouth", "😶");
    m.insert("smirk", "😏");
    m.insert("unamused", "😒");
    m.insert("roll_eyes", "🙄");
    m.insert("grimacing", "😬");
    m.insert("relieved", "😌");
    m.insert("pensive", "😔");
    m.insert("sleepy", "😪");
    m.insert("sleeping", "😴");
    m.insert("drooling", "🤤");
    m.insert("mask", "😷");
    m.insert("nerd", "🤓");
    m.insert("sunglasses", "😎");
    m.insert("confused", "😕");
    m.insert("worried", "😟");
    m.insert("frowning", "☹️");
    m.insert("open_mouth", "😮");
    m.insert("hushed", "😯");
    m.insert("astonished", "😲");
    m.insert("flushed", "😳");
    m.insert("pleading", "🥺");
    m.insert("cry", "😢");
    m.insert("sob", "😭");
    m.insert("scream", "😱");
    m.insert("angry", "😠");
    m.insert("rage", "😡");
    m.insert("skull", "💀");
    m.insert("poop", "💩");
    m.insert("pile_of_poo", "💩");
    m.insert("clown", "🤡");
    m.insert("ghost", "👻");
    m.insert("alien", "👽");
    m.insert("robot", "🤖");

    // People & Body
    m.insert("wave", "👋");
    m.insert("raised_hand", "✋");
    m.insert("ok_hand", "👌");
    m.insert("thumbs_up", "👍");
    m.insert("thumbsup", "👍");
    m.insert("+1", "👍");
    m.insert("thumbs_down", "👎");
    m.insert("thumbsdown", "👎");
    m.insert("-1", "👎");
    m.insert("fist", "✊");
    m.insert("punch", "👊");
    m.insert("clap", "👏");
    m.insert("raised_hands", "🙌");
    m.insert("open_hands", "👐");
    m.insert("palms_up", "🤲");
    m.insert("handshake", "🤝");
    m.insert("pray", "🙏");
    m.insert("point_up", "☝️");
    m.insert("point_up_2", "👆");
    m.insert("point_down", "👇");
    m.insert("point_left", "👈");
    m.insert("point_right", "👉");
    m.insert("middle_finger", "🖕");
    m.insert("hand", "✋");
    m.insert("v", "✌️");
    m.insert("pinched_fingers", "🤌");
    m.insert("love_you", "🤟");
    m.insert("metal", "🤘");
    m.insert("call_me", "🤙");
    m.insert("muscle", "💪");
    m.insert("brain", "🧠");
    m.insert("eyes", "👀");
    m.insert("eye", "👁️");
    m.insert("tongue", "👅");
    m.insert("lips", "👄");
    m.insert("baby", "👶");
    m.insert("boy", "👦");
    m.insert("girl", "👧");
    m.insert("man", "👨");
    m.insert("woman", "👩");
    m.insert("older_man", "👴");
    m.insert("older_woman", "👵");

    // Animals & Nature
    m.insert("dog", "🐶");
    m.insert("cat", "🐱");
    m.insert("mouse", "🐭");
    m.insert("hamster", "🐹");
    m.insert("rabbit", "🐰");
    m.insert("fox", "🦊");
    m.insert("bear", "🐻");
    m.insert("panda", "🐼");
    m.insert("koala", "🐨");
    m.insert("tiger", "🐯");
    m.insert("lion", "🦁");
    m.insert("cow", "🐮");
    m.insert("pig", "🐷");
    m.insert("frog", "🐸");
    m.insert("monkey", "🐵");
    m.insert("see_no_evil", "🙈");
    m.insert("hear_no_evil", "🙉");
    m.insert("speak_no_evil", "🙊");
    m.insert("chicken", "🐔");
    m.insert("penguin", "🐧");
    m.insert("bird", "🐦");
    m.insert("eagle", "🦅");
    m.insert("duck", "🦆");
    m.insert("owl", "🦉");
    m.insert("bat", "🦇");
    m.insert("wolf", "🐺");
    m.insert("horse", "🐴");
    m.insert("unicorn", "🦄");
    m.insert("bee", "🐝");
    m.insert("bug", "🐛");
    m.insert("butterfly", "🦋");
    m.insert("snail", "🐌");
    m.insert("ladybug", "🐞");
    m.insert("ant", "🐜");
    m.insert("spider", "🕷️");
    m.insert("scorpion", "🦂");
    m.insert("crab", "🦀");
    m.insert("snake", "🐍");
    m.insert("turtle", "🐢");
    m.insert("fish", "🐟");
    m.insert("octopus", "🐙");
    m.insert("whale", "🐳");
    m.insert("dolphin", "🐬");
    m.insert("shark", "🦈");
    m.insert("crocodile", "🐊");
    m.insert("dragon", "🐉");
    m.insert("dinosaur", "🦕");
    m.insert("t_rex", "🦖");
    m.insert("raccoon", "🦝");
    m.insert("vampire", "🧛");

    // Food & Drink
    m.insert("apple", "🍎");
    m.insert("green_apple", "🍏");
    m.insert("pear", "🍐");
    m.insert("orange", "🍊");
    m.insert("lemon", "🍋");
    m.insert("banana", "🍌");
    m.insert("watermelon", "🍉");
    m.insert("grapes", "🍇");
    m.insert("strawberry", "🍓");
    m.insert("cherry", "🍒");
    m.insert("peach", "🍑");
    m.insert("mango", "🥭");
    m.insert("pineapple", "🍍");
    m.insert("coconut", "🥥");
    m.insert("avocado", "🥑");
    m.insert("eggplant", "🍆");
    m.insert("potato", "🥔");
    m.insert("carrot", "🥕");
    m.insert("corn", "🌽");
    m.insert("hot_pepper", "🌶️");
    m.insert("bread", "🍞");
    m.insert("croissant", "🥐");
    m.insert("pizza", "🍕");
    m.insert("hamburger", "🍔");
    m.insert("fries", "🍟");
    m.insert("hotdog", "🌭");
    m.insert("taco", "🌮");
    m.insert("burrito", "🌯");
    m.insert("egg", "🥚");
    m.insert("cooking", "🍳");
    m.insert("pancakes", "🥞");
    m.insert("bacon", "🥓");
    m.insert("steak", "🥩");
    m.insert("poultry_leg", "🍗");
    m.insert("sushi", "🍣");
    m.insert("ramen", "🍜");
    m.insert("cake", "🎂");
    m.insert("cookie", "🍪");
    m.insert("chocolate", "🍫");
    m.insert("candy", "🍬");
    m.insert("lollipop", "🍭");
    m.insert("icecream", "🍦");
    m.insert("donut", "🍩");
    m.insert("coffee", "☕");
    m.insert("tea", "🍵");
    m.insert("beer", "🍺");
    m.insert("beers", "🍻");
    m.insert("wine", "🍷");
    m.insert("cocktail", "🍸");
    m.insert("champagne", "🍾");

    // Objects & Symbols
    m.insert("heart", "❤️");
    m.insert("red_heart", "❤️");
    m.insert("orange_heart", "🧡");
    m.insert("yellow_heart", "💛");
    m.insert("green_heart", "💚");
    m.insert("blue_heart", "💙");
    m.insert("purple_heart", "💜");
    m.insert("black_heart", "🖤");
    m.insert("white_heart", "🤍");
    m.insert("broken_heart", "💔");
    m.insert("fire", "🔥");
    m.insert("sparkles", "✨");
    m.insert("star", "⭐");
    m.insert("glowing_star", "🌟");
    m.insert("sparkle", "❇️");
    m.insert("zap", "⚡");
    m.insert("boom", "💥");
    m.insert("sun", "☀️");
    m.insert("moon", "🌙");
    m.insert("cloud", "☁️");
    m.insert("rainbow", "🌈");
    m.insert("umbrella", "☂️");
    m.insert("snowflake", "❄️");
    m.insert("snowman", "⛄");
    m.insert("gift", "🎁");
    m.insert("balloon", "🎈");
    m.insert("tada", "🎉");
    m.insert("party_popper", "🎉");
    m.insert("confetti", "🎊");
    m.insert("trophy", "🏆");
    m.insert("medal", "🏅");
    m.insert("first_place", "🥇");
    m.insert("second_place", "🥈");
    m.insert("third_place", "🥉");
    m.insert("soccer", "⚽");
    m.insert("basketball", "🏀");
    m.insert("football", "🏈");
    m.insert("baseball", "⚾");
    m.insert("tennis", "🎾");
    m.insert("guitar", "🎸");
    m.insert("microphone", "🎤");
    m.insert("headphones", "🎧");
    m.insert("video_game", "🎮");
    m.insert("dice", "🎲");
    m.insert("dart", "🎯");
    m.insert("phone", "📱");
    m.insert("computer", "💻");
    m.insert("keyboard", "⌨️");
    m.insert("printer", "🖨️");
    m.insert("mouse_pc", "🖱️");
    m.insert("light_bulb", "💡");
    m.insert("bulb", "💡");
    m.insert("battery", "🔋");
    m.insert("electric_plug", "🔌");
    m.insert("money", "💰");
    m.insert("dollar", "💵");
    m.insert("credit_card", "💳");
    m.insert("gem", "💎");
    m.insert("wrench", "🔧");
    m.insert("hammer", "🔨");
    m.insert("gear", "⚙️");
    m.insert("link", "🔗");
    m.insert("lock", "🔒");
    m.insert("unlock", "🔓");
    m.insert("key", "🔑");
    m.insert("bell", "🔔");
    m.insert("bookmark", "🔖");
    m.insert("flag", "🚩");
    m.insert("triangular_flag", "🚩");
    m.insert("checkered_flag", "🏁");
    m.insert("clock", "🕐");
    m.insert("hourglass", "⏳");
    m.insert("watch", "⌚");
    m.insert("alarm_clock", "⏰");
    m.insert("stopwatch", "⏱️");
    m.insert("calendar", "📅");
    m.insert("memo", "📝");
    m.insert("pencil", "✏️");
    m.insert("pen", "🖊️");
    m.insert("book", "📖");
    m.insert("books", "📚");
    m.insert("newspaper", "📰");
    m.insert("folder", "📁");
    m.insert("inbox_tray", "📥");
    m.insert("outbox_tray", "📤");
    m.insert("envelope", "✉️");
    m.insert("email", "📧");
    m.insert("package", "📦");
    m.insert("clipboard", "📋");
    m.insert("pushpin", "📌");
    m.insert("paperclip", "📎");
    m.insert("scissors", "✂️");
    m.insert("wastebasket", "🗑️");

    // Status & Indicators
    m.insert("check", "✓");
    m.insert("checkmark", "✓");
    m.insert("check_mark", "✔️");
    m.insert("white_check_mark", "✅");
    m.insert("x", "❌");
    m.insert("cross", "❌");
    m.insert("cross_mark", "❌");
    m.insert("negative_squared_cross_mark", "❎");
    m.insert("question", "❓");
    m.insert("grey_question", "❔");
    m.insert("exclamation", "❗");
    m.insert("grey_exclamation", "❕");
    m.insert("warning", "⚠️");
    m.insert("no_entry", "⛔");
    m.insert("prohibited", "🚫");
    m.insert("sos", "🆘");
    m.insert("info", "ℹ️");
    m.insert("ok", "🆗");
    m.insert("new", "🆕");
    m.insert("free", "🆓");
    m.insert("up", "🆙");
    m.insert("cool", "🆒");
    m.insert("vs", "🆚");
    m.insert("100", "💯");
    m.insert("arrow_up", "⬆️");
    m.insert("arrow_down", "⬇️");
    m.insert("arrow_left", "⬅️");
    m.insert("arrow_right", "➡️");
    m.insert("arrow_upper_right", "↗️");
    m.insert("arrow_lower_right", "↘️");
    m.insert("arrow_lower_left", "↙️");
    m.insert("arrow_upper_left", "↖️");
    m.insert("arrows_counterclockwise", "🔄");
    m.insert("back", "🔙");
    m.insert("end", "🔚");
    m.insert("on", "🔛");
    m.insert("soon", "🔜");
    m.insert("top", "🔝");
    m.insert("arrow_forward", "▶️");
    m.insert("arrow_backward", "◀️");
    m.insert("play_pause", "⏯️");
    m.insert("stop_button", "⏹️");
    m.insert("record_button", "⏺️");
    m.insert("fast_forward", "⏩");
    m.insert("rewind", "⏪");
    m.insert("repeat", "🔁");
    m.insert("shuffle", "🔀");
    m.insert("radio_button", "🔘");
    m.insert("white_circle", "⚪");
    m.insert("black_circle", "⚫");
    m.insert("red_circle", "🔴");
    m.insert("blue_circle", "🔵");
    m.insert("green_circle", "🟢");
    m.insert("yellow_circle", "🟡");
    m.insert("orange_circle", "🟠");
    m.insert("purple_circle", "🟣");
    m.insert("brown_circle", "🟤");
    m.insert("white_square", "⬜");
    m.insert("black_square", "⬛");
    m.insert("red_square", "🟥");
    m.insert("blue_square", "🟦");
    m.insert("green_square", "🟩");
    m.insert("yellow_square", "🟨");
    m.insert("orange_square", "🟧");
    m.insert("purple_square", "🟪");
    m.insert("brown_square", "🟫");

    // Development
    m.insert("rocket", "🚀");
    m.insert("construction", "🚧");
    m.insert("mag", "🔍");
    m.insert("search", "🔍");
    m.insert("mag_right", "🔎");
    m.insert("speech_balloon", "💬");
    m.insert("thought_balloon", "💭");
    m.insert("nail_care", "💅");
    m.insert("zany_face", "🤪");
    m.insert("monocle", "🧐");
    m.insert("nerd_face", "🤓");
    m.insert("partying_face", "🥳");
    m.insert("mechanical_arm", "🦾");
    m.insert("mechanical_leg", "🦿");

    m
});

/// Get the emoji character for a given name.
///
/// Returns `None` if the emoji is not found.
pub fn get_emoji(name: &str) -> Option<&'static str> {
    EMOJI_MAP.get(name).copied()
}

/// Check if an emoji name is valid.
pub fn is_valid_emoji(name: &str) -> bool {
    EMOJI_MAP.contains_key(name)
}

/// Get all available emoji names.
pub fn all_emoji_names() -> impl Iterator<Item = &'static str> {
    EMOJI_MAP.keys().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_emoji() {
        assert_eq!(get_emoji("smile"), Some("😄"));
        assert_eq!(get_emoji("thumbs_up"), Some("👍"));
        assert_eq!(get_emoji("heart"), Some("❤️"));
        assert_eq!(get_emoji("rocket"), Some("🚀"));
    }

    #[test]
    fn test_unknown_emoji() {
        assert_eq!(get_emoji("nonexistent"), None);
    }

    #[test]
    fn test_is_valid_emoji() {
        assert!(is_valid_emoji("smile"));
        assert!(!is_valid_emoji("unknown"));
    }

    #[test]
    fn test_all_emoji_names() {
        let names: Vec<_> = all_emoji_names().collect();
        assert!(names.contains(&"smile"));
        assert!(names.contains(&"heart"));
        assert!(names.len() > 100); // We should have at least 100 emojis
    }
}
