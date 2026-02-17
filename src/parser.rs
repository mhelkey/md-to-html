#[derive(Debug, PartialEq)]
enum Symbol {
    Asterisk,
    Pound,
    LeftBracket,
    RightBracket,
    LeftParentheses,
    RightParenthese,
    Exclamation,
    Character(char),
}

impl Symbol {
    fn from_char(character: char) -> Symbol {
        match character {
            '*' => Symbol::Asterisk,
            '#' => Symbol::Pound,
            '[' => Symbol::LeftBracket,
            ']' => Symbol::RightBracket,
            '(' => Symbol::LeftParentheses,
            ')' => Symbol::RightParenthese,
            '!' => Symbol::Exclamation,
            character => Symbol::Character(character),
        }
    }
}

fn lex(text: impl ToString) -> Vec<Symbol> {
    let mut symbols = Vec::new();

    for c in text.to_string().chars() {
        symbols.push(Symbol::from_char(c));
    }

    symbols
}

/// Link -> LeftBracket, n * Character, RightBracket, LeftParenthese, n * Character, Right Parenthese
/// Image -> Exclamation, Link
/// Italics -> Asterisk, n * Characters, Asterisk
/// Bold -> Asterisk * 2, n * Characters, Asterisk * 2
/// Heading 1 -> Pound, n * Characters
/// Heading 2 -> Pound * 2, n * Characters
/// ...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let input = "a!b[c]d(e)f*g#";
        let output = vec![
            Symbol::Character('a'),
            Symbol::Exclamation,
            Symbol::Character('b'),
            Symbol::LeftBracket,
            Symbol::Character('c'),
            Symbol::RightBracket,
            Symbol::Character('d'),
            Symbol::LeftParentheses,
            Symbol::Character('e'),
            Symbol::RightParenthese,
            Symbol::Character('f'),
            Symbol::Asterisk,
            Symbol::Character('g'),
            Symbol::Pound,
        ];

        assert_eq!(lex(input), output);
    }
}
