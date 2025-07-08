Char in std::ascii - Rust
std
::
ascii
Enum
Char
Copy item path
Source
#[repr(u8)]
pub enum Char {
Show 128 variants
Null = 0,
    StartOfHeading = 1,
    StartOfText = 2,
    EndOfText = 3,
    EndOfTransmission = 4,
    Enquiry = 5,
    Acknowledge = 6,
    Bell = 7,
    Backspace = 8,
    CharacterTabulation = 9,
    LineFeed = 10,
    LineTabulation = 11,
    FormFeed = 12,
    CarriageReturn = 13,
    ShiftOut = 14,
    ShiftIn = 15,
    DataLinkEscape = 16,
    DeviceControlOne = 17,
    DeviceControlTwo = 18,
    DeviceControlThree = 19,
    DeviceControlFour = 20,
    NegativeAcknowledge = 21,
    SynchronousIdle = 22,
    EndOfTransmissionBlock = 23,
    Cancel = 24,
    EndOfMedium = 25,
    Substitute = 26,
    Escape = 27,
    InformationSeparatorFour = 28,
    InformationSeparatorThree = 29,
    InformationSeparatorTwo = 30,
    InformationSeparatorOne = 31,
    Space = 32,
    ExclamationMark = 33,
    QuotationMark = 34,
    NumberSign = 35,
    DollarSign = 36,
    PercentSign = 37,
    Ampersand = 38,
    Apostrophe = 39,
    LeftParenthesis = 40,
    RightParenthesis = 41,
    Asterisk = 42,
    PlusSign = 43,
    Comma = 44,
    HyphenMinus = 45,
    FullStop = 46,
    Solidus = 47,
    Digit0 = 48,
    Digit1 = 49,
    Digit2 = 50,
    Digit3 = 51,
    Digit4 = 52,
    Digit5 = 53,
    Digit6 = 54,
    Digit7 = 55,
    Digit8 = 56,
    Digit9 = 57,
    Colon = 58,
    Semicolon = 59,
    LessThanSign = 60,
    EqualsSign = 61,
    GreaterThanSign = 62,
    QuestionMark = 63,
    CommercialAt = 64,
    CapitalA = 65,
    CapitalB = 66,
    CapitalC = 67,
    CapitalD = 68,
    CapitalE = 69,
    CapitalF = 70,
    CapitalG = 71,
    CapitalH = 72,
    CapitalI = 73,
    CapitalJ = 74,
    CapitalK = 75,
    CapitalL = 76,
    CapitalM = 77,
    CapitalN = 78,
    CapitalO = 79,
    CapitalP = 80,
    CapitalQ = 81,
    CapitalR = 82,
    CapitalS = 83,
    CapitalT = 84,
    CapitalU = 85,
    CapitalV = 86,
    CapitalW = 87,
    CapitalX = 88,
    CapitalY = 89,
    CapitalZ = 90,
    LeftSquareBracket = 91,
    ReverseSolidus = 92,
    RightSquareBracket = 93,
    CircumflexAccent = 94,
    LowLine = 95,
    GraveAccent = 96,
    SmallA = 97,
    SmallB = 98,
    SmallC = 99,
    SmallD = 100,
    SmallE = 101,
    SmallF = 102,
    SmallG = 103,
    SmallH = 104,
    SmallI = 105,
    SmallJ = 106,
    SmallK = 107,
    SmallL = 108,
    SmallM = 109,
    SmallN = 110,
    SmallO = 111,
    SmallP = 112,
    SmallQ = 113,
    SmallR = 114,
    SmallS = 115,
    SmallT = 116,
    SmallU = 117,
    SmallV = 118,
    SmallW = 119,
    SmallX = 120,
    SmallY = 121,
    SmallZ = 122,
    LeftCurlyBracket = 123,
    VerticalLine = 124,
    RightCurlyBracket = 125,
    Tilde = 126,
    Delete = 127,
}
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
Expand description
One of the 128 Unicode characters from U+0000 through U+007F,
often known as the
ASCII
subset.
Officially, this is the first
block
in Unicode,
Basic Latin
.
For details, see the
C0 Controls and Basic Latin
code chart.
This block was based on older 7-bit character code standards such as
ANSI X3.4-1977, ISO 646-1973, and
NIST FIPS 1-2
.
§
When to use this
The main advantage of this subset is that it’s always valid UTF-8.  As such,
the
&[ascii::Char]
->
&str
conversion function (as well as other related
ones) are O(1):
no
runtime checks are needed.
If you’re consuming strings, you should usually handle Unicode and thus
accept
str
s, not limit yourself to
ascii::Char
s.
However, certain formats are intentionally designed to produce ASCII-only
output in order to be 8-bit-clean.  In those cases, it can be simpler and
faster to generate
ascii::Char
s instead of dealing with the variable width
properties of general UTF-8 encoded strings, while still allowing the result
to be used freely with other Rust things that deal in general
str
s.
For example, a UUID library might offer a way to produce the string
representation of a UUID as an
[ascii::Char; 36]
to avoid memory
allocation yet still allow it to be used as UTF-8 via
as_str
without
paying for validation (or needing
unsafe
code) the way it would if it
were provided as a
[u8; 36]
.
§
Layout
This type is guaranteed to have a size and alignment of 1 byte.
§
Names
The variants on this type are
Unicode names
of the characters
in upper camel case, with a few tweaks:
For
<control>
characters, the primary alias name is used.
LATIN
is dropped, as this block has no non-latin letters.
LETTER
is dropped, as
CAPITAL
/
SMALL
suffices in this block.
DIGIT
s use a single digit rather than writing out
ZERO
,
ONE
, etc.
Variants
§
§
Null = 0
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0000 (The default variant)
§
StartOfHeading = 1
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0001
§
StartOfText = 2
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0002
§
EndOfText = 3
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0003
§
EndOfTransmission = 4
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0004
§
Enquiry = 5
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0005
§
Acknowledge = 6
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0006
§
Bell = 7
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0007
§
Backspace = 8
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0008
§
CharacterTabulation = 9
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0009
§
LineFeed = 10
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+000A
§
LineTabulation = 11
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+000B
§
FormFeed = 12
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+000C
§
CarriageReturn = 13
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+000D
§
ShiftOut = 14
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+000E
§
ShiftIn = 15
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+000F
§
DataLinkEscape = 16
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0010
§
DeviceControlOne = 17
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0011
§
DeviceControlTwo = 18
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0012
§
DeviceControlThree = 19
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0013
§
DeviceControlFour = 20
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0014
§
NegativeAcknowledge = 21
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0015
§
SynchronousIdle = 22
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0016
§
EndOfTransmissionBlock = 23
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0017
§
Cancel = 24
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0018
§
EndOfMedium = 25
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0019
§
Substitute = 26
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+001A
§
Escape = 27
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+001B
§
InformationSeparatorFour = 28
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+001C
§
InformationSeparatorThree = 29
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+001D
§
InformationSeparatorTwo = 30
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+001E
§
InformationSeparatorOne = 31
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+001F
§
Space = 32
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0020
§
ExclamationMark = 33
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0021
§
QuotationMark = 34
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0022
§
NumberSign = 35
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0023
§
DollarSign = 36
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0024
§
PercentSign = 37
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0025
§
Ampersand = 38
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0026
§
Apostrophe = 39
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0027
§
LeftParenthesis = 40
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0028
§
RightParenthesis = 41
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0029
§
Asterisk = 42
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+002A
§
PlusSign = 43
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+002B
§
Comma = 44
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+002C
§
HyphenMinus = 45
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+002D
§
FullStop = 46
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+002E
§
Solidus = 47
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+002F
§
Digit0 = 48
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0030
§
Digit1 = 49
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0031
§
Digit2 = 50
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0032
§
Digit3 = 51
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0033
§
Digit4 = 52
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0034
§
Digit5 = 53
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0035
§
Digit6 = 54
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0036
§
Digit7 = 55
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0037
§
Digit8 = 56
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0038
§
Digit9 = 57
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0039
§
Colon = 58
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+003A
§
Semicolon = 59
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+003B
§
LessThanSign = 60
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+003C
§
EqualsSign = 61
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+003D
§
GreaterThanSign = 62
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+003E
§
QuestionMark = 63
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+003F
§
CommercialAt = 64
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0040
§
CapitalA = 65
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0041
§
CapitalB = 66
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0042
§
CapitalC = 67
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0043
§
CapitalD = 68
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0044
§
CapitalE = 69
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0045
§
CapitalF = 70
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0046
§
CapitalG = 71
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0047
§
CapitalH = 72
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0048
§
CapitalI = 73
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0049
§
CapitalJ = 74
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+004A
§
CapitalK = 75
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+004B
§
CapitalL = 76
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+004C
§
CapitalM = 77
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+004D
§
CapitalN = 78
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+004E
§
CapitalO = 79
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+004F
§
CapitalP = 80
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0050
§
CapitalQ = 81
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0051
§
CapitalR = 82
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0052
§
CapitalS = 83
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0053
§
CapitalT = 84
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0054
§
CapitalU = 85
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0055
§
CapitalV = 86
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0056
§
CapitalW = 87
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0057
§
CapitalX = 88
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0058
§
CapitalY = 89
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0059
§
CapitalZ = 90
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+005A
§
LeftSquareBracket = 91
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+005B
§
ReverseSolidus = 92
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+005C
§
RightSquareBracket = 93
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+005D
§
CircumflexAccent = 94
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+005E
§
LowLine = 95
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+005F
§
GraveAccent = 96
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0060
§
SmallA = 97
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0061
§
SmallB = 98
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0062
§
SmallC = 99
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0063
§
SmallD = 100
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0064
§
SmallE = 101
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0065
§
SmallF = 102
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0066
§
SmallG = 103
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0067
§
SmallH = 104
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0068
§
SmallI = 105
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0069
§
SmallJ = 106
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+006A
§
SmallK = 107
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+006B
§
SmallL = 108
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+006C
§
SmallM = 109
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+006D
§
SmallN = 110
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+006E
§
SmallO = 111
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+006F
§
SmallP = 112
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0070
§
SmallQ = 113
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0071
§
SmallR = 114
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0072
§
SmallS = 115
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0073
§
SmallT = 116
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0074
§
SmallU = 117
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0075
§
SmallV = 118
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0076
§
SmallW = 119
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0077
§
SmallX = 120
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0078
§
SmallY = 121
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+0079
§
SmallZ = 122
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+007A
§
LeftCurlyBracket = 123
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+007B
§
VerticalLine = 124
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+007C
§
RightCurlyBracket = 125
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+007D
§
Tilde = 126
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+007E
§
Delete = 127
🔬
This is a nightly-only experimental API. (
ascii_char_variants
#110998
)
U+007F
Implementations
§
Source
§
impl
AsciiChar
Source
pub const fn
from_u8
(b:
u8
) ->
Option
<
AsciiChar
>
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
Creates an ascii character from the byte
b
,
or returns
None
if it’s too large.
Source
pub const unsafe fn
from_u8_unchecked
(b:
u8
) ->
AsciiChar
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
Creates an ASCII character from the byte
b
,
without checking whether it’s valid.
§
Safety
b
must be in
0..=127
, or else this is UB.
Source
pub const fn
digit
(d:
u8
) ->
Option
<
AsciiChar
>
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
When passed the
number
0
,
1
, …,
9
, returns the
character
'0'
,
'1'
, …,
'9'
respectively.
If
d >= 10
, returns
None
.
Source
pub const unsafe fn
digit_unchecked
(d:
u8
) ->
AsciiChar
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
When passed the
number
0
,
1
, …,
9
, returns the
character
'0'
,
'1'
, …,
'9'
respectively, without checking that it’s in-range.
§
Safety
This is immediate UB if called with
d > 64
.
If
d >= 10
and
d <= 64
, this is allowed to return any value or panic.
Notably, it should not be expected to return hex digits, or any other
reasonable extension of the decimal digits.
(This loose safety condition is intended to simplify soundness proofs
when writing code using this method, since the implementation doesn’t
need something really specific, not to make those other arguments do
something useful. It might be tightened before stabilization.)
Source
pub const fn
to_u8
(self) ->
u8
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
Gets this ASCII character as a byte.
Source
pub const fn
to_char
(self) ->
char
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
Gets this ASCII character as a
char
Unicode Scalar Value.
Source
pub const fn
as_str
(&self) -> &
str
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
Views this ASCII character as a one-code-unit UTF-8
str
.
Trait Implementations
§
Source
§
impl
Clone
for
AsciiChar
Source
§
fn
clone
(&self) ->
AsciiChar
Returns a copy of the value.
Read more
1.0.0
·
Source
§
fn
clone_from
(&mut self, source: &Self)
Performs copy-assignment from
source
.
Read more
Source
§
impl
Debug
for
AsciiChar
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
1.0.0
·
Source
§
impl
Default
for
AsciiChar
Source
§
fn
default
() ->
AsciiChar
Returns the default value of
Null
Source
§
impl
Display
for
AsciiChar
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
Source
§
impl<'a>
Extend
<&'a
AsciiChar
> for
String
Source
§
fn
extend
<I>(&mut self, iter: I)
where
    I:
IntoIterator
<Item = &'a
AsciiChar
>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, c: &'a
AsciiChar
)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Extends a collection with exactly one element.
Source
§
fn
extend_reserve
(&mut self, additional:
usize
)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Reserves capacity in a collection for the given number of additional elements.
Read more
Source
§
impl
Extend
<
AsciiChar
> for
String
Source
§
fn
extend
<I>(&mut self, iter: I)
where
    I:
IntoIterator
<Item =
AsciiChar
>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, c:
AsciiChar
)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Extends a collection with exactly one element.
Source
§
fn
extend_reserve
(&mut self, additional:
usize
)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Reserves capacity in a collection for the given number of additional elements.
Read more
Source
§
impl
From
<
AsciiChar
> for
char
Source
§
fn
from
(chr:
AsciiChar
) ->
char
Converts to this type from the input type.
Source
§
impl
From
<
AsciiChar
> for
u128
Source
§
fn
from
(chr:
AsciiChar
) ->
u128
Converts to this type from the input type.
Source
§
impl
From
<
AsciiChar
> for
u16
Source
§
fn
from
(chr:
AsciiChar
) ->
u16
Converts to this type from the input type.
Source
§
impl
From
<
AsciiChar
> for
u32
Source
§
fn
from
(chr:
AsciiChar
) ->
u32
Converts to this type from the input type.
Source
§
impl
From
<
AsciiChar
> for
u64
Source
§
fn
from
(chr:
AsciiChar
) ->
u64
Converts to this type from the input type.
Source
§
impl
From
<
AsciiChar
> for
u8
Source
§
fn
from
(chr:
AsciiChar
) ->
u8
Converts to this type from the input type.
Source
§
impl
Hash
for
AsciiChar
Source
§
fn
hash
<__H>(&self, state:
&mut __H
)
where
    __H:
Hasher
,
Feeds this value into the given
Hasher
.
Read more
1.3.0
·
Source
§
fn
hash_slice
<H>(data: &[Self], state:
&mut H
)
where
    H:
Hasher
,
    Self:
Sized
,
Feeds a slice of this type into the given
Hasher
.
Read more
Source
§
impl
Ord
for
AsciiChar
Source
§
fn
cmp
(&self, other: &
AsciiChar
) ->
Ordering
This method returns an
Ordering
between
self
and
other
.
Read more
1.21.0
·
Source
§
fn
max
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the maximum of two values.
Read more
1.21.0
·
Source
§
fn
min
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the minimum of two values.
Read more
1.50.0
·
Source
§
fn
clamp
(self, min: Self, max: Self) -> Self
where
    Self:
Sized
,
Restrict a value to a certain interval.
Read more
Source
§
impl
PartialEq
for
AsciiChar
Source
§
fn
eq
(&self, other: &
AsciiChar
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl
PartialOrd
for
AsciiChar
Source
§
fn
partial_cmp
(&self, other: &
AsciiChar
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl
Step
for
AsciiChar
Source
§
fn
steps_between
(_: &
AsciiChar
, _: &
AsciiChar
) -> (
usize
,
Option
<
usize
>)
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the bounds on the number of
successor
steps required to get from
start
to
end
like
Iterator::size_hint()
.
Read more
Source
§
fn
forward_checked
(start:
AsciiChar
, count:
usize
) ->
Option
<
AsciiChar
>
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
successor
of
self
count
times.
Read more
Source
§
fn
backward_checked
(start:
AsciiChar
, count:
usize
) ->
Option
<
AsciiChar
>
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
predecessor
of
self
count
times.
Read more
Source
§
unsafe fn
forward_unchecked
(start:
AsciiChar
, count:
usize
) ->
AsciiChar
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
successor
of
self
count
times.
Read more
Source
§
unsafe fn
backward_unchecked
(start:
AsciiChar
, count:
usize
) ->
AsciiChar
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
predecessor
of
self
count
times.
Read more
Source
§
fn
forward
(start: Self, count:
usize
) -> Self
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
successor
of
self
count
times.
Read more
Source
§
fn
backward
(start: Self, count:
usize
) -> Self
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
predecessor
of
self
count
times.
Read more
Source
§
impl
Copy
for
AsciiChar
Source
§
impl
Eq
for
AsciiChar
Source
§
impl
StructuralPartialEq
for
AsciiChar
Source
§
impl
TrustedStep
for
AsciiChar
Auto Trait Implementations
§
§
impl
Freeze
for
AsciiChar
§
impl
RefUnwindSafe
for
AsciiChar
§
impl
Send
for
AsciiChar
§
impl
Sync
for
AsciiChar
§
impl
Unpin
for
AsciiChar
§
impl
UnwindSafe
for
AsciiChar
Blanket Implementations
§
Source
§
impl<T>
Any
for T
where
    T: 'static + ?
Sized
,
Source
§
fn
type_id
(&self) ->
TypeId
Gets the
TypeId
of
self
.
Read more
Source
§
impl<T>
Borrow
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow
(&self) ->
&T
Immutably borrows from an owned value.
Read more
Source
§
impl<T>
BorrowMut
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow_mut
(&mut self) ->
&mut T
Mutably borrows from an owned value.
Read more
Source
§
impl<T>
CloneToUninit
for T
where
    T:
Clone
,
Source
§
unsafe fn
clone_to_uninit
(&self, dest:
*mut
u8
)
🔬
This is a nightly-only experimental API. (
clone_to_uninit
#126799
)
Performs copy-assignment from
self
to
dest
.
Read more
Source
§
impl<T>
From
<T> for T
Source
§
fn
from
(t: T) -> T
Returns the argument unchanged.
Source
§
impl<T, U>
Into
<U> for T
where
    U:
From
<T>,
Source
§
fn
into
(self) -> U
Calls
U::from(self)
.
That is, this conversion is whatever the implementation of
From
<T> for U
chooses to do.
Source
§
impl<T>
ToOwned
for T
where
    T:
Clone
,
Source
§
type
Owned
= T
The resulting type after obtaining ownership.
Source
§
fn
to_owned
(&self) -> T
Creates owned data from borrowed data, usually by cloning.
Read more
Source
§
fn
clone_into
(&self, target:
&mut T
)
Uses borrowed data to replace owned data, usually by cloning.
Read more
Source
§
impl<T>
ToString
for T
where
    T:
Display
+ ?
Sized
,
Source
§
fn
to_string
(&self) ->
String
Converts the given value to a
String
.
Read more
Source
§
impl<T, U>
TryFrom
<U> for T
where
    U:
Into
<T>,
Source
§
type
Error
=
Infallible
The type returned in the event of a conversion error.
Source
§
fn
try_from
(value: U) ->
Result
<T, <T as
TryFrom
<U>>::
Error
>
Performs the conversion.
Source
§
impl<T, U>
TryInto
<U> for T
where
    U:
TryFrom
<T>,
Source
§
type
Error
= <U as
TryFrom
<T>>::
Error
The type returned in the event of a conversion error.
Source
§
fn
try_into
(self) ->
Result
<U, <U as
TryFrom
<T>>::
Error
>
Performs the conversion.