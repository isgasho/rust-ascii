use std::mem::transmute;
use std::fmt;
use std::error::Error;
use std::ascii::AsciiExt;

use AsciiCast;

#[allow(non_camel_case_types)]
/// An ASCII character. It wraps a `u8`, with the highest bit always zero.
#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Copy)]
#[repr(u8)]
pub enum Ascii {
    /// `'\0'`
    Null            =   0,
    /// [Start Of Heading](http://en.wikipedia.org/wiki/Start_of_Heading)
    SOH             =   1,
    /// [Start Of teXt](http://en.wikipedia.org/wiki/Start_of_Text)
    SOX             =   2,
    /// [End of TeXt](http://en.wikipedia.org/wiki/End-of-Text_character)
    ETX             =   3,
    /// [End Of Transmission](http://en.wikipedia.org/wiki/End-of-Transmission_character)
    EOT             =   4,
    /// [Enquiry](http://en.wikipedia.org/wiki/Enquiry_character)
    ENQ             =   5,
    /// [Acknowledgement](http://en.wikipedia.org/wiki/Acknowledge_character)
    ACK             =   6,
    /// [bell / alarm / audible](http://en.wikipedia.org/wiki/Bell_character)
    ///
    /// `'\a'` is not recognized by Rust.
    Bell            =   7,
    /// [Backspace](http://en.wikipedia.org/wiki/Backspace)
    ///
    /// `'\b'` is not recognized by Rust.
    BackSpace       =   8,
    /// `'\t'`
    Tab             =   9,
    /// `'\n'`
    LineFeed        =  10,
    /// [Vertical tab](http://en.wikipedia.org/wiki/Vertical_Tab)
    ///
    /// `'\v'` is not recognized by Rust.
    VT              =  11,
    /// [Form Feed](http://en.wikipedia.org/wiki/Form_Feed)
    ///
    /// `'\f'` is not recognized by Rust.
    FF              =  12,
    /// `'\r'`
    CarriageReturn  =  13,
    /// [Shift In](http://en.wikipedia.org/wiki/Shift_Out_and_Shift_In_characters)
    SI              =  14,
    /// [Shift Out](http://en.wikipedia.org/wiki/Shift_Out_and_Shift_In_characters)
    SO              =  15,
    /// [Data Link Escape](http://en.wikipedia.org/wiki/Data_Link_Escape)
    DLE             =  16,
    /// [Device control 1, often XON](http://en.wikipedia.org/wiki/Device_Control_1)
    DC1             =  17,
    /// Device control 2
    DC2             =  18,
    /// Device control 3, Often XOFF
    DC3             =  19,
    /// Device control 4
    DC4             =  20,
    /// [Negative AcKnowledgement](http://en.wikipedia.org/wiki/Negative-acknowledge_character)
    NAK             =  21,
    /// [Synchronous idle](http://en.wikipedia.org/wiki/Synchronous_Idle)
    SYN             =  22,
    /// [End of Transmission Block](http://en.wikipedia.org/wiki/End-of-Transmission-Block_character)
    ETB             =  23,
    /// [Cancel](http://en.wikipedia.org/wiki/Cancel_character)
    CAN             =  24,
    /// [End of Medium](http://en.wikipedia.org/wiki/End_of_Medium)
    EM              =  25,
    /// [Substitute](http://en.wikipedia.org/wiki/Substitute_character)
    SUB             =  26,
    /// [Escape](http://en.wikipedia.org/wiki/Escape_character)
    ///
    /// `'\e'` is not recognized by Rust.
    ESC             =  27,
    /// [File Separator](http://en.wikipedia.org/wiki/File_separator)
    FS              =  28,
    /// [Group Separator](http://en.wikipedia.org/wiki/Group_separator)
    GS              =  29,
    /// [Record Separator](http://en.wikipedia.org/wiki/Record_separator)
    RS              =  30,
    /// [Unit Separator](http://en.wikipedia.org/wiki/Unit_separator)
    US              =  31,
    /// `' '`
    Space           =  32,
    /// `'!'`
    Exclamation     =  33,
    /// `'"'`
    Quotation       =  34,
    /// `'#'`
    Hash            =  35,
    /// `'$'`
    Dollar          =  36,
    /// `'%'`
    Percent         =  37,
    /// `'&'`
    Ampersand       =  38,
    /// `'\''`
    Apostrophe      =  39,
    /// `'('`
    ParenOpen       =  40,
    /// `')'`
    ParenClose      =  41,
    /// `'*'`
    Asterisk        =  42,
    /// `'+'`
    Plus            =  43,
    /// `','`
    Comma           =  44,
    /// `'-'`
    Minus           =  45,
    /// `'.'`
    Dot             =  46,
    /// `'/'`
    Slash           =  47,
    /// `'0'`
    _0              =  48,
    /// `'1'`
    _1              =  49,
    /// `'2'`
    _2              =  50,
    /// `'3'`
    _3              =  51,
    /// `'4'`
    _4              =  52,
    /// `'5'`
    _5              =  53,
    /// `'6'`
    _6              =  54,
    /// `'7'`
    _7              =  55,
    /// `'8'`
    _8              =  56,
    /// `'9'`
    _9              =  57,
    /// `':'`
    Colon           =  58,
    /// `';'`
    SemiColon       =  59,
    /// `'<'`
    LessThan        =  60,
    /// `'='`
    Equal           =  61,
    /// `'>'`
    GreaterThan     =  62,
    /// `'?'`
    Question        =  63,
    /// `'@'`
    At              =  64,
    /// `'A'`
    A               =  65,
    /// `'B'`
    B               =  66,
    /// `'C'`
    C               =  67,
    /// `'D'`
    D               =  68,
    /// `'E'`
    E               =  69,
    /// `'F'`
    F               =  70,
    /// `'G'`
    G               =  71,
    /// `'H'`
    H               =  72,
    /// `'I'`
    I               =  73,
    /// `'J'`
    J               =  74,
    /// `'K'`
    K               =  75,
    /// `'L'`
    L               =  76,
    /// `'M'`
    M               =  77,
    /// `'N'`
    N               =  78,
    /// `'O'`
    O               =  79,
    /// `'P'`
    P               =  80,
    /// `'Q'`
    Q               =  81,
    /// `'R'`
    R               =  82,
    /// `'S'`
    S               =  83,
    /// `'T'`
    T               =  84,
    /// `'U'`
    U               =  85,
    /// `'V'`
    V               =  86,
    /// `'W'`
    W               =  87,
    /// `'X'`
    X               =  88,
    /// `'Y'`
    Y               =  89,
    /// `'Z'`
    Z               =  90,
    /// `'['`
    BracketOpen     =  91,
    /// `'\'`
    BackSlash       =  92,
    /// `']'`
    BracketClose    =  93,
    /// `'_'`
    Caret           =  94,
    /// `'_'`
    UnderScore      =  95,
    /// `'`'`
    Grave           =  96,
    /// `'a'`
    a               =  97,
    /// `'b'`
    b               =  98,
    /// `'c'`
    c               =  99,
    /// `'d'`
    d               = 100,
    /// `'e'`
    e               = 101,
    /// `'f'`
    f               = 102,
    /// `'g'`
    g               = 103,
    /// `'h'`
    h               = 104,
    /// `'i'`
    i               = 105,
    /// `'j'`
    j               = 106,
    /// `'k'`
    k               = 107,
    /// `'l'`
    l               = 108,
    /// `'m'`
    m               = 109,
    /// `'n'`
    n               = 110,
    /// `'o'`
    o               = 111,
    /// `'p'`
    p               = 112,
    /// `'q'`
    q               = 113,
    /// `'r'`
    r               = 114,
    /// `'s'`
    s               = 115,
    /// `'t'`
    t               = 116,
    /// `'u'`
    u               = 117,
    /// `'v'`
    v               = 118,
    /// `'w'`
    w               = 119,
    /// `'x'`
    x               = 120,
    /// `'y'`
    y               = 121,
    /// `'z'`
    z               = 122,
    /// `'{'`
    CurlyBraceOpen  = 123,
    /// `'|'`
    VerticalBar     = 124,
    /// `'}'`
    CurlyBraceClose = 125,
    /// `'~'`
    Tilde           = 126,
    /// [Delete](http://en.wikipedia.org/wiki/Delete_character)
    DEL             = 127,
}

impl Ascii {
    /// Constructs an ASCII character from a `u8`, `char` or other character type.
    ///
    /// # Failure
    /// Returns `Err(())` if the character can't be ASCII encoded.
    ///
    /// # Example
    /// ```
    /// # use ascii::Ascii;
    /// let a = Ascii::from('g').unwrap();
    /// assert_eq!(a.as_char(), 'g');
    /// ```
    #[inline]
    pub fn from<C:IntoAscii>(ch: C) -> Result<Self, ()> {
        ch.into_ascii().map_err(|_| () )
    }

    /// Constructs an ASCII character from a `char` or `u8` without any checks.
    pub unsafe fn from_unchecked<C:IntoAscii>(ch: C) -> Self {
        ch.into_ascii_unchecked()
    }

    /// Constructs an ASCII character from a `u8`.
    ///
    /// # Failure
    /// Returns `Err(())` if the character can't be ASCII encoded.
    ///
    /// # Example
    /// ```
    /// # use ascii::Ascii;
    /// let a = Ascii::from_byte(65).unwrap();
    /// assert_eq!(a.as_char(), 'A');
    /// ```
    #[inline]
    pub fn from_byte(ch: u8) -> Result<Ascii, ()> {
        unsafe{if ch <= 0x7F {
            return Ok(ch.to_ascii_nocheck());
        }}
        Err(())
    }

    /// Converts an ASCII character into a `u8`.
    #[inline]
    pub fn as_byte(&self) -> u8 {
        *self as u8
    }

    /// Converts an ASCII character into a `char`.
    #[inline]
    pub fn as_char(&self) -> char {
        self.as_byte() as char
    }

    // the following methods are like ctype, and the implementation is inspired by musl

    /// Check if the character is a letter (a-z, A-Z)
    #[inline]
    pub fn is_alphabetic(&self) -> bool {
        let c = self.as_byte() | 0b010_0000;// Turns uppercase into lowercase.
        c >= b'a' && c <= b'z'
    }

    /// Check if the character is a number (0-9)
    #[inline]
    pub fn is_digit(&self) -> bool {
        self >= &Ascii::_0 && self <= &Ascii::_9
    }

    /// Check if the character is a letter or number
    #[inline]
    pub fn is_alphanumeric(&self) -> bool {
        self.is_alphabetic() || self.is_digit()
    }

    /// Check if the character is a space or horizontal tab
    #[inline]
    pub fn is_blank(&self) -> bool {
        *self == Ascii::Space || *self == Ascii::Tab
    }

    /// Check if the character is a control character
    ///
    /// # Examples
    /// ```
    /// use ascii::AsciiCast;
    /// assert_eq!('\0'.to_ascii().unwrap().is_control(), true);
    /// assert_eq!('n'.to_ascii().unwrap().is_control(), false);
    /// assert_eq!(' '.to_ascii().unwrap().is_control(), false);
    /// assert_eq!('\n'.to_ascii().unwrap().is_control(), true);
    /// ```
    #[inline]
    pub fn is_control(&self) -> bool {
        self.as_byte() < 0x20 || *self == Ascii::DEL
    }

    /// Checks if the character is printable (except space)
    ///
    /// # Examples
    /// ```
    /// use ascii::AsciiCast;
    /// assert_eq!('n'.to_ascii().unwrap().is_graph(), true);
    /// assert_eq!(' '.to_ascii().unwrap().is_graph(), false);
    /// assert_eq!('\n'.to_ascii().unwrap().is_graph(), false);
    /// ```
    #[inline]
    pub fn is_graph(&self) -> bool {
        self.as_byte().wrapping_sub(0x21) < 0x5E
    }

    /// Checks if the character is printable (including space)
    ///
    /// # Examples
    /// ```
    /// use ascii::AsciiCast;
    /// assert_eq!('n'.to_ascii().unwrap().is_print(), true);
    /// assert_eq!(' '.to_ascii().unwrap().is_print(), true);
    /// assert_eq!('\n'.to_ascii().unwrap().is_print(), false);
    /// ```
    #[inline]
    pub fn is_print(&self) -> bool {
        self.as_byte().wrapping_sub(0x20) < 0x5F
    }

    /// Checks if the character is alphabetic and lowercase
    ///
    /// # Examples
    /// ```
    /// use ascii::AsciiCast;
    /// assert_eq!('a'.to_ascii().unwrap().is_lowercase(), true);
    /// assert_eq!('A'.to_ascii().unwrap().is_lowercase(), false);
    /// assert_eq!('@'.to_ascii().unwrap().is_lowercase(), false);
    /// ```
    #[inline]
    pub fn is_lowercase(&self) -> bool {
        self.as_byte().wrapping_sub(b'a') < 26
    }

    /// Checks if the character is alphabetic and uppercase
    ///
    /// # Examples
    /// ```
    /// use ascii::AsciiCast;
    /// assert_eq!('A'.to_ascii().unwrap().is_uppercase(), true);
    /// assert_eq!('a'.to_ascii().unwrap().is_uppercase(), false);
    /// assert_eq!('@'.to_ascii().unwrap().is_uppercase(), false);
    /// ```
    #[inline]
    pub fn is_uppercase(&self) -> bool {
        self.as_byte().wrapping_sub(b'A') < 26
    }

    /// Checks if the character is punctuation
    ///
    /// # Examples
    /// ```
    /// use ascii::AsciiCast;
    /// assert_eq!('n'.to_ascii().unwrap().is_punctuation(), false);
    /// assert_eq!(' '.to_ascii().unwrap().is_punctuation(), false);
    /// assert_eq!('_'.to_ascii().unwrap().is_punctuation(), true);
    /// assert_eq!('~'.to_ascii().unwrap().is_punctuation(), true);
    /// ```
    #[inline]
    pub fn is_punctuation(&self) -> bool {
        self.is_graph() && !self.is_alphanumeric()
    }

    /// Checks if the character is a valid hex digit
    ///
    /// # Examples
    /// ```
    /// use ascii::AsciiCast;
    /// assert_eq!('5'.to_ascii().unwrap().is_hex(), true);
    /// assert_eq!('a'.to_ascii().unwrap().is_hex(), true);
    /// assert_eq!('F'.to_ascii().unwrap().is_hex(), true);
    /// assert_eq!(32u8.to_ascii().unwrap().is_hex(), false);
    /// ```
    #[inline]
    pub fn is_hex(&self) -> bool {
        self.is_digit() || (self.as_byte() | 32u8).wrapping_sub(b'a') < 6
    }
}

impl fmt::Display for Ascii {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_char().fmt(f)
    }
}

impl fmt::Debug for Ascii {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_char().fmt(f)
     }
}

#[cfg(feature = "unstable")]
impl AsciiExt for Ascii {
    type Owned = Ascii;

    #[inline]
    fn is_ascii(&self) -> bool {
        true
    }

    fn to_ascii_uppercase(&self) -> Ascii {
        unsafe{ self.as_byte().to_ascii_uppercase().to_ascii_nocheck() }
    }

    fn to_ascii_lowercase(&self) -> Ascii {
        unsafe{ self.as_byte().to_ascii_uppercase().to_ascii_nocheck() }
    }

    fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
        self.as_byte().eq_ignore_ascii_case(&other.as_byte())
    }

    #[inline]
    fn make_ascii_uppercase(&mut self) {
        *self = self.to_ascii_uppercase();
    }

    #[inline]
    fn make_ascii_lowercase(&mut self) {
        *self = self.to_ascii_lowercase();
    }
}

impl<'a> AsciiCast<'a> for u8 {
    type Target = Ascii;

    #[inline]
    unsafe fn to_ascii_nocheck(&self) -> Ascii {
        transmute(*self)
    }
}

impl<'a> AsciiCast<'a> for char {
    type Target = Ascii;

    #[inline]
    unsafe fn to_ascii_nocheck(&self) -> Ascii {
        (*self as u8).to_ascii_nocheck()
    }
}


/// Error returned by `IntoAscii`.
#[derive(PartialEq)]
pub struct IntoAsciiError(());

const ERRORMSG_CHAR: &'static str = "not an ASCII character";

impl fmt::Debug for IntoAsciiError {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", ERRORMSG_CHAR)
    }
}

impl fmt::Display for IntoAsciiError {
    fn fmt(&self,  fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", ERRORMSG_CHAR)
    }
}

impl Error for IntoAsciiError {
    fn description(&self) -> &'static str {
        ERRORMSG_CHAR
    }
}


/// Convert `char`, `u8` and other character types to `Ascii`.
pub trait IntoAscii : AsciiExt {
    /// Convert to `Ascii` without checking that it is an ASCII character.
    unsafe fn into_ascii_unchecked(self) -> Ascii;
    /// Convert to `Ascii`.
    fn into_ascii(self) -> Result<Ascii,IntoAsciiError>;
}

#[cfg(feature = "unstable")]
impl IntoAscii for Ascii {
    fn into_ascii(self) -> Result<Ascii,IntoAsciiError> {
        Ok(self)
    }
    unsafe fn into_ascii_unchecked(self) -> Ascii {
        self
    }
}

impl IntoAscii for u8 {
    fn into_ascii(self) -> Result<Ascii,IntoAsciiError> {
        unsafe{if self <= 0x7F {
            return Ok(self.into_ascii_unchecked());
        }}
        Err(IntoAsciiError(()))
    }
    unsafe fn into_ascii_unchecked(self) -> Ascii {
        transmute(self)
    }
}

impl IntoAscii for char {
    fn into_ascii(self) -> Result<Ascii,IntoAsciiError> {
        unsafe{if self as u32 <= 0x7F {
            return Ok(self.into_ascii_unchecked());
        }}
        Err(IntoAsciiError(()))
    }
    unsafe fn into_ascii_unchecked(self) -> Ascii {
        (self as u8).into_ascii_unchecked()
    }
}


#[cfg(test)]
mod tests {
    use AsciiCast;
    use super::{Ascii,IntoAscii,IntoAsciiError};

    #[test]
    fn to_ascii() {
        assert_eq!(65_u8.to_ascii(), Ok(Ascii::A));
        assert_eq!(255_u8.to_ascii(), Err(()));

        assert_eq!('A'.to_ascii(), Ok(Ascii::A));
        assert_eq!('λ'.to_ascii(), Err(()));
    }

    #[test]
    fn into_ascii() {
        fn generic<C:IntoAscii>(c: C) -> Result<Ascii,IntoAsciiError> {
            c.into_ascii()
        }
        assert_eq!(generic('A'), Ok(Ascii::A));
        assert_eq!(generic(b'A'), Ok(Ascii::A));
    }

    #[test]
    fn as_byte() {
        assert_eq!(65u8.to_ascii().unwrap().as_byte(), 65u8);
        assert_eq!('A'.to_ascii().unwrap().as_byte(), 65u8);
    }

    #[test]
    fn as_char() {
        assert_eq!(65u8.to_ascii().unwrap().as_char(), 'A');
        assert_eq!('A'.to_ascii().unwrap().as_char(), 'A');
    }

    #[test]
    fn is_digit() {
        assert!('0'.to_ascii().unwrap().is_digit());
        assert!('9'.to_ascii().unwrap().is_digit());
        assert!(!'/'.to_ascii().unwrap().is_digit());
        assert!(!':'.to_ascii().unwrap().is_digit());
    }

    #[test]
    fn is_control() {
        assert!(0x1f_u8.to_ascii().unwrap().is_control());
        assert!(0x7f_u8.to_ascii().unwrap().is_control());
        assert!(!' '.to_ascii().unwrap().is_control());
    }

    #[test]
    fn fmt_display_ascii() {
        assert_eq!(format!("{}", Ascii::t), "t".to_string());
    }

    #[test]
    fn fmt_debug_ascii() {
        assert_eq!(format!("{:?}", Ascii::t), "'t'".to_string());
    }
}
