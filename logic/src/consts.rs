
use tok::Token;

pub const STR_NOT: &'static str = "!";
pub const STR_AND: &'static str = "^";
pub const STR_OR : &'static str = "v";
pub const STR_IF : &'static str = "->";
pub const STR_IFF: &'static str = "<->";

pub const STR_PRETTY_NOT: &'static str = "¬";
pub const STR_PRETTY_AND: &'static str = "∧";
pub const STR_PRETTY_OR : &'static str = "∨";
pub const STR_PRETTY_IF : &'static str = "→";
pub const STR_PRETTY_IFF: &'static str = "⇔";

pub const TOK_STR_NOT: &'static [Token] = &[Token::Char('!')];
pub const TOK_STR_AND: &'static [Token] = &[Token::Char('^')];
pub const TOK_STR_OR : &'static [Token] = &[Token::Char('v')];
pub const TOK_STR_IF : &'static [Token] = &[Token::Char('-'), Token::Char('>')];
pub const TOK_STR_IFF: &'static [Token] = &[Token::Char('<'), Token::Char('-'), Token::Char('>')];

pub const TOK_STR_PRETTY_NOT: &'static [Token] = &[Token::Char('¬')];
pub const TOK_STR_PRETTY_AND: &'static [Token] = &[Token::Char('∧')];
pub const TOK_STR_PRETTY_OR : &'static [Token] = &[Token::Char('∨')];
pub const TOK_STR_PRETTY_IF : &'static [Token] = &[Token::Char('→')];
pub const TOK_STR_PRETTY_IFF: &'static [Token] = &[Token::Char('⇔')];
