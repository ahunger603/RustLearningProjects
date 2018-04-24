#[derive(PartialEq)]
enum MessageType {
    Silence = 0,
    Symbols = 1,
    Normal = 2,
    Shouting = 3
}

#[derive(PartialEq)]
enum Punctuation {
    None = 0,
    Question = 1
}

pub fn reply(message: &str) -> &str {
    let mut message_interpretation : (MessageType, Punctuation) = (MessageType::Silence, Punctuation::None);
    for c in message.chars() {
        match c {
            'A' ... 'Z' => {
                if message_interpretation.0 == MessageType::Silence || message_interpretation.0 == MessageType::Symbols {
                    message_interpretation = (MessageType::Shouting, Punctuation::None)
                }
            },
            ' ' | '\t' | '\r' | '\n' => {},
            '?' => message_interpretation.1 = Punctuation::Question,
            'a' ... 'z' => message_interpretation = (MessageType::Normal, Punctuation::None),
            _ => {
                if message_interpretation.0 == MessageType::Silence {
                    message_interpretation = (MessageType::Symbols, Punctuation::None)
                }
            }
        }
    }
    match message_interpretation {
        (MessageType::Shouting, Punctuation::Question) => "Calm down, I know what I'm doing!",
        (_, Punctuation::Question) => "Sure.",
        (MessageType::Shouting, _) => "Whoa, chill out!",
        (MessageType::Silence, _) => "Fine. Be that way!",
        (_, _) => "Whatever.",
    }
}

