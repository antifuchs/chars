/// Generated with ["target/debug/generator", "../data/ascii/nametable"]

#[derive(Clone)]
pub struct Information {
    pub value: char,
    pub mnemonics: &'static [&'static str],
    pub synonyms: &'static [&'static str],
    pub note: Option<&'static str>,
}


pub static PRINTABLE_CHARS: &'static [Information; 127] = &[
    Information{value:'\u{0}', mnemonics:&["NUL"], synonyms:&["Null", "\\\\0"], note:None},
    Information{value:'\u{1}', mnemonics:&["SOH"], synonyms:&["Start Of Header"], note:None},
    Information{value:'\u{2}', mnemonics:&["STX"], synonyms:&["Start of Text"], note:None},
    Information{value:'\u{3}', mnemonics:&["ETX"], synonyms:&["End of Text"], note:None},
    Information{value:'\u{4}', mnemonics:&["EOT"], synonyms:&["End Of Transmission"], note:None},
    Information{value:'\u{5}', mnemonics:&["ENQ"], synonyms:&["Enquiry"], note:None},
    Information{value:'\u{6}', mnemonics:&["ACK"], synonyms:&["Acknowledge"], note:None},
    Information{value:'\u{7}', mnemonics:&["BEL"], synonyms:&["Bell", "\\\\a", "Alert"], note:None},
    Information{value:'\u{8}', mnemonics:&["BS"], synonyms:&["Backspace", "\\\\b"], note:None},
    Information{value:'\t', mnemonics:&["HT", "TAB"], synonyms:&["Character Tabulation", "Horizontal Tab", "\\\\t"], note:None},
    Information{value:'\n', mnemonics:&["LF", "NL"], synonyms:&["Line Feed", "Newline", "\\\\n"], note:None},
    Information{value:'\u{b}', mnemonics:&["VT"], synonyms:&["Line Tabulation", "Vertical Tab", "\\\\v"], note:None},
    Information{value:'\u{c}', mnemonics:&["FF"], synonyms:&["Form Feed", "\\\\f"], note:None},
    Information{value:'\r', mnemonics:&["CR"], synonyms:&["Carriage Return", "\\\\r"], note:None},
    Information{value:'\u{e}', mnemonics:&["SO", "LS1"], synonyms:&["Shift Out", "Locking Shift 1"], note:None},
    Information{value:'\u{f}', mnemonics:&["SI", "LS0"], synonyms:&["Shift In", "Locking Shift 0"], note:None},
    Information{value:'\u{10}', mnemonics:&["DLE"], synonyms:&["Data Link Escape"], note:None},
    Information{value:'\u{11}', mnemonics:&["DC1"], synonyms:&["Device Control 1"], note:None},
    Information{value:'\u{12}', mnemonics:&["DC2"], synonyms:&["Device Control 2"], note:None},
    Information{value:'\u{13}', mnemonics:&["DC3"], synonyms:&["Device Control 3"], note:None},
    Information{value:'\u{14}', mnemonics:&["DC4"], synonyms:&["Device Control 4"], note:None},
    Information{value:'\u{15}', mnemonics:&["NAK"], synonyms:&["Negative Acknowledge"], note:None},
    Information{value:'\u{16}', mnemonics:&["SYN"], synonyms:&["Synchronous Idle"], note:None},
    Information{value:'\u{17}', mnemonics:&["ETB"], synonyms:&["End of Transmission Block"], note:None},
    Information{value:'\u{18}', mnemonics:&["CAN"], synonyms:&["Cancel"], note:None},
    Information{value:'\u{19}', mnemonics:&["EM"], synonyms:&["End of Medium"], note:None},
    Information{value:'\u{1a}', mnemonics:&["SUB"], synonyms:&["Substitute"], note:None},
    Information{value:'\u{1b}', mnemonics:&["ESC"], synonyms:&["Escape"], note:None},
    Information{value:'\u{1c}', mnemonics:&["FS"], synonyms:&["File Separator"], note:None},
    Information{value:'\u{1d}', mnemonics:&["GS"], synonyms:&["Group Separator"], note:None},
    Information{value:'\u{1e}', mnemonics:&["RS"], synonyms:&["Record Separator"], note:None},
    Information{value:'\u{1f}', mnemonics:&["US"], synonyms:&["Unit Separator"], note:None},
    Information{value:' ', mnemonics:&[" ", "SP"], synonyms:&["Space", "Blank"], note:None},
    Information{value:'!', mnemonics:&["!"], synonyms:&["Exclamation Mark", "Bang", "Excl", "Wow", "Factorial", "Shriek", "Pling", "Smash", "Cuss"], note:None},
    Information{value:'\"', mnemonics:&["\\\""], synonyms:&["Quotation Mark", "Double Quote", "Quote", "String Quote", "Dirk", "Literal Mark", "Double Glitch", "&quot;"], note:Some("See \' and ` for matching names.")},
    Information{value:'#', mnemonics:&["#"], synonyms:&["Number Sign", "Pound", "Number", "Sharp", "Crunch", "Mesh", "Hex", "Hash", "Flash", "Grid", "Octothorpe"], note:None},
    Information{value:'$', mnemonics:&["$"], synonyms:&["Currency Sign", "Dollar", "Buck", "Cash", "Ding"], note:None},
    Information{value:'%', mnemonics:&["%"], synonyms:&["Percent Sign", "Mod", "Modulo"], note:None},
    Information{value:'&', mnemonics:&["&"], synonyms:&["Ampersand", "Amper", "And", "&amp;"], note:None},
    Information{value:'\'', mnemonics:&["\'"], synonyms:&["Apostrophe", "Single Quote", "Close Quote", "Prime", "Tick", "Pop", "Spark", "Glitch", "&apos;"], note:Some("See ` and \\\" for matching names.")},
    Information{value:'(', mnemonics:&["("], synonyms:&["Left Parenthesis", "Open", "Open Paren", "Left Paren", "Wax", "Sad"], note:Some("See ) for matching names.")},
    Information{value:')', mnemonics:&[")"], synonyms:&["Right Parenthesis", "Close", "Close Paren", "Right Paren", "Wane", "Happy"], note:Some("See ( for matching names.")},
    Information{value:'*', mnemonics:&["*"], synonyms:&["Asterisk", "Star", "Splat", "Aster", "Times", "Gear", "Dingle", "Bug", "Twinkle", "Glob\" ,"], note:None},
    Information{value:'+', mnemonics:&["+"], synonyms:&["Plus Sign", "Add", "Cross"], note:None},
    Information{value:',', mnemonics:&[","], synonyms:&["Comma", "Tail"], note:None},
    Information{value:'-', mnemonics:&["-"], synonyms:&["Hyphen", "Dash", "Minus", "Worm"], note:None},
    Information{value:'.', mnemonics:&["."], synonyms:&["Full Stop", "Dot", "Decimal Point", "Radix Point", "Point", "Period", "Spot"], note:None},
    Information{value:'/', mnemonics:&["/"], synonyms:&["Solidus", "Slash", "Stroke", "Slant", "Diagonal", "Virgule", "Over", "Slat"], note:Some("See \\\\ for matching names.")},
    Information{value:'0', mnemonics:&["0"], synonyms:&["Digit Zero"], note:None},
    Information{value:'1', mnemonics:&["1"], synonyms:&["Digit One"], note:None},
    Information{value:'2', mnemonics:&["2"], synonyms:&["Digit Two"], note:None},
    Information{value:'3', mnemonics:&["3"], synonyms:&["Digit Three"], note:None},
    Information{value:'4', mnemonics:&["4"], synonyms:&["Digit Four"], note:None},
    Information{value:'5', mnemonics:&["5"], synonyms:&["Digit Five"], note:None},
    Information{value:'6', mnemonics:&["6"], synonyms:&["Digit Six"], note:None},
    Information{value:'7', mnemonics:&["7"], synonyms:&["Digit Seven"], note:None},
    Information{value:'8', mnemonics:&["8"], synonyms:&["Digit Eight"], note:None},
    Information{value:'9', mnemonics:&["9"], synonyms:&["Digit Nine"], note:None},
    Information{value:':', mnemonics:&[":"], synonyms:&["Colon", "Double-Dot"], note:None},
    Information{value:';', mnemonics:&[";"], synonyms:&["Semicolon", "Semi", "Go-on"], note:None},
    Information{value:'<', mnemonics:&["<"], synonyms:&["Less-than Sign", "Left Angle Bracket", "Read From", "In", "From", "Comesfrom", "Left Funnel", "Left Broket", "Crunch", "Suck", "&lt;"], note:Some("See > for matching names.")},
    Information{value:'=', mnemonics:&["="], synonyms:&["Equals Sign", "Quadrathorp", "Gets", "Becomes", "Half-Mesh"], note:None},
    Information{value:'>', mnemonics:&[">"], synonyms:&["Greater-than sign", "Right Angle Bracket", "Write To", "Into", "Toward", "Out", "To", "Gozinta", "Right Funnel", "Right Broket", "Zap", "Blow", "&gt;"], note:Some("See < for matching names.")},
    Information{value:'?', mnemonics:&["?"], synonyms:&["Question Mark", "Whatmark", "What", "Ques"], note:None},
    Information{value:'@', mnemonics:&["@"], synonyms:&["Commercial At", "At", "Each", "Vortex", "Whorl", "Whirlpool", "Cyclone", "Snail", "Rose"], note:None},
    Information{value:'A', mnemonics:&["A"], synonyms:&["Majuscule A", "Capital A", "Uppercase A"], note:None},
    Information{value:'B', mnemonics:&["B"], synonyms:&["Majuscule B", "Capital B", "Uppercase B"], note:None},
    Information{value:'C', mnemonics:&["C"], synonyms:&["Majuscule C", "Capital C", "Uppercase C"], note:None},
    Information{value:'D', mnemonics:&["D"], synonyms:&["Majuscule D", "Capital D", "Uppercase D"], note:None},
    Information{value:'E', mnemonics:&["E"], synonyms:&["Majuscule E", "Capital E", "Uppercase E"], note:None},
    Information{value:'F', mnemonics:&["F"], synonyms:&["Majuscule F", "Capital F", "Uppercase F"], note:None},
    Information{value:'G', mnemonics:&["G"], synonyms:&["Majuscule G", "Capital G", "Uppercase G"], note:None},
    Information{value:'H', mnemonics:&["H"], synonyms:&["Majuscule H", "Capital H", "Uppercase H"], note:None},
    Information{value:'I', mnemonics:&["I"], synonyms:&["Majuscule I", "Capital I", "Uppercase I"], note:None},
    Information{value:'J', mnemonics:&["J"], synonyms:&["Majuscule J", "Capital J", "Uppercase J"], note:None},
    Information{value:'K', mnemonics:&["K"], synonyms:&["Majuscule K", "Capital K", "Uppercase K"], note:None},
    Information{value:'L', mnemonics:&["L"], synonyms:&["Majuscule L", "Capital L", "Uppercase L"], note:None},
    Information{value:'M', mnemonics:&["M"], synonyms:&["Majuscule M", "Capital M", "Uppercase M"], note:None},
    Information{value:'N', mnemonics:&["N"], synonyms:&["Majuscule N", "Capital N", "Uppercase N"], note:None},
    Information{value:'O', mnemonics:&["O"], synonyms:&["Majuscule O", "Capital O", "Uppercase O"], note:None},
    Information{value:'P', mnemonics:&["P"], synonyms:&["Majuscule P", "Capital P", "Uppercase P"], note:None},
    Information{value:'Q', mnemonics:&["Q"], synonyms:&["Majuscule Q", "Capital Q", "Uppercase Q"], note:None},
    Information{value:'R', mnemonics:&["R"], synonyms:&["Majuscule R", "Capital R", "Uppercase R"], note:None},
    Information{value:'S', mnemonics:&["S"], synonyms:&["Majuscule S", "Capital S", "Uppercase S"], note:None},
    Information{value:'T', mnemonics:&["T"], synonyms:&["Majuscule T", "Capital T", "Uppercase T"], note:None},
    Information{value:'U', mnemonics:&["U"], synonyms:&["Majuscule U", "Capital U", "Uppercase U"], note:None},
    Information{value:'V', mnemonics:&["V"], synonyms:&["Majuscule V", "Capital V", "Uppercase V"], note:None},
    Information{value:'W', mnemonics:&["W"], synonyms:&["Majuscule W", "Capital W", "Uppercase W"], note:None},
    Information{value:'X', mnemonics:&["X"], synonyms:&["Majuscule X", "Capital X", "Uppercase X"], note:None},
    Information{value:'Y', mnemonics:&["Y"], synonyms:&["Majuscule Y", "Capital Y", "Uppercase Y"], note:None},
    Information{value:'Z', mnemonics:&["Z"], synonyms:&["Majuscule Z", "Capital Z", "Uppercase Z"], note:None},
    Information{value:'[', mnemonics:&["["], synonyms:&["Left Square Bracket", "Bracket", "Bra", "Square"], note:Some("See ] for matching names.")},
    Information{value:'\\', mnemonics:&["\\\\"], synonyms:&["Reversed Solidus", "Backslash", "Bash", "Backslant", "Backwhack", "Backslat", "Literal", "Escape"], note:Some("See / for matching names.")},
    Information{value:']', mnemonics:&["]"], synonyms:&["Right Square Bracket", "Unbracket", "Ket", "Unsquare"], note:Some("See [ for matching names.")},
    Information{value:'^', mnemonics:&["^"], synonyms:&["Circumflex Accent", "Circumflex", "Caret", "Uparrow", "Hat", "Control", "Boink", "Chevron", "Hiccup", "Sharkfin", "Fang"], note:None},
    Information{value:'_', mnemonics:&["_"], synonyms:&["Low Line", "Underscore", "Underline", "Underbar", "Under", "Score", "Backarrow", "Flatworm"], note:Some("Backarrow refers to this character\'s graphic in 1963 ASCII.")},
    Information{value:'`', mnemonics:&["`"], synonyms:&["Grave Accent", "Grave", "Backquote", "Left Quote", "Open Quote", "Backprime", "Unapostrophe", "Backspark", "Birk", "Blugle", "Back Tick", "Push"], note:Some("See \' and \\\" for matching names.")},
    Information{value:'a', mnemonics:&["a"], synonyms:&["Miniscule a", "Small a", "Lowercase a"], note:None},
    Information{value:'b', mnemonics:&["b"], synonyms:&["Miniscule b", "Small b", "Lowercase b"], note:None},
    Information{value:'c', mnemonics:&["c"], synonyms:&["Miniscule c", "Small c", "Lowercase c"], note:None},
    Information{value:'d', mnemonics:&["d"], synonyms:&["Miniscule d", "Small d", "Lowercase d"], note:None},
    Information{value:'e', mnemonics:&["e"], synonyms:&["Miniscule e", "Small e", "Lowercase e"], note:None},
    Information{value:'f', mnemonics:&["f"], synonyms:&["Miniscule f", "Small f", "Lowercase f"], note:None},
    Information{value:'g', mnemonics:&["g"], synonyms:&["Miniscule g", "Small g", "Lowercase g"], note:None},
    Information{value:'h', mnemonics:&["h"], synonyms:&["Miniscule h", "Small h", "Lowercase h"], note:None},
    Information{value:'i', mnemonics:&["i"], synonyms:&["Miniscule i", "Small i", "Lowercase i"], note:None},
    Information{value:'j', mnemonics:&["j"], synonyms:&["Miniscule j", "Small j", "Lowercase j"], note:None},
    Information{value:'k', mnemonics:&["k"], synonyms:&["Miniscule k", "Small k", "Lowercase k"], note:None},
    Information{value:'l', mnemonics:&["l"], synonyms:&["Miniscule l", "Small l", "Lowercase l"], note:None},
    Information{value:'m', mnemonics:&["m"], synonyms:&["Miniscule m", "Small m", "Lowercase m"], note:None},
    Information{value:'n', mnemonics:&["n"], synonyms:&["Miniscule n", "Small n", "Lowercase n"], note:None},
    Information{value:'o', mnemonics:&["o"], synonyms:&["Miniscule o", "Small o", "Lowercase o"], note:None},
    Information{value:'p', mnemonics:&["p"], synonyms:&["Miniscule p", "Small p", "Lowercase p"], note:None},
    Information{value:'q', mnemonics:&["q"], synonyms:&["Miniscule q", "Small q", "Lowercase q"], note:None},
    Information{value:'r', mnemonics:&["r"], synonyms:&["Miniscule r", "Small r", "Lowercase r"], note:None},
    Information{value:'s', mnemonics:&["s"], synonyms:&["Miniscule s", "Small s", "Lowercase s"], note:None},
    Information{value:'t', mnemonics:&["t"], synonyms:&["Miniscule t", "Small t", "Lowercase t"], note:None},
    Information{value:'u', mnemonics:&["u"], synonyms:&["Miniscule u", "Small u", "Lowercase u"], note:None},
    Information{value:'v', mnemonics:&["v"], synonyms:&["Miniscule v", "Small v", "Lowercase v"], note:None},
    Information{value:'w', mnemonics:&["w"], synonyms:&["Miniscule w", "Small w", "Lowercase w"], note:None},
    Information{value:'x', mnemonics:&["x"], synonyms:&["Miniscule x", "Small x", "Lowercase x"], note:None},
    Information{value:'y', mnemonics:&["y"], synonyms:&["Miniscule y", "Small y", "Lowercase y"], note:None},
    Information{value:'z', mnemonics:&["z"], synonyms:&["Miniscule z", "Small z", "Lowercase z"], note:None},
    Information{value:'{', mnemonics:&["{"], synonyms:&["Left Curly Bracket", "Left Brace", "Brace", "Open Brace", "Curly", "Leftit", "Embrace"], note:Some("See } for matching names.")},
    Information{value:'|', mnemonics:&["|"], synonyms:&["Vertical Line", "Pipe", "Bar", "Or", "V-Bar", "Spike", "Gozinta", "Thru"], note:None},
    Information{value:'}', mnemonics:&["}"], synonyms:&["Right Curly Bracket", "Right Brace", "Unbrace", "Close Brace", "Uncurly", "Rytit", "Bracelet"], note:Some("See { for matching names.")},
    Information{value:'~', mnemonics:&["~"], synonyms:&["Overline", "Tilde", "Swung Dash", "Squiggle", "Approx", "Wiggle", "Twiddle", "Enyay"], note:None},
];


pub static NAMES: &'static [(&'static str, char)] = &[
    ("NUL", '\u{0}'),("null", '\u{0}'),("\\\\0", '\u{0}'),
    ("SOH", '\u{1}'),("start of header", '\u{1}'),
    ("STX", '\u{2}'),("start of text", '\u{2}'),
    ("ETX", '\u{3}'),("end of text", '\u{3}'),
    ("EOT", '\u{4}'),("end of transmission", '\u{4}'),
    ("ENQ", '\u{5}'),("enquiry", '\u{5}'),
    ("ACK", '\u{6}'),("acknowledge", '\u{6}'),
    ("BEL", '\u{7}'),("bell", '\u{7}'),("\\\\a", '\u{7}'),("alert", '\u{7}'),
    ("BS", '\u{8}'),("backspace", '\u{8}'),("\\\\b", '\u{8}'),
    ("HT", '\t'),("TAB", '\t'),("character tabulation", '\t'),("horizontal tab", '\t'),("\\\\t", '\t'),
    ("LF", '\n'),("NL", '\n'),("line feed", '\n'),("newline", '\n'),("\\\\n", '\n'),
    ("VT", '\u{b}'),("line tabulation", '\u{b}'),("vertical tab", '\u{b}'),("\\\\v", '\u{b}'),
    ("FF", '\u{c}'),("form feed", '\u{c}'),("\\\\f", '\u{c}'),
    ("CR", '\r'),("carriage return", '\r'),("\\\\r", '\r'),
    ("SO", '\u{e}'),("LS1", '\u{e}'),("shift out", '\u{e}'),("locking shift 1", '\u{e}'),
    ("SI", '\u{f}'),("LS0", '\u{f}'),("shift in", '\u{f}'),("locking shift 0", '\u{f}'),
    ("DLE", '\u{10}'),("data link escape", '\u{10}'),
    ("DC1", '\u{11}'),("device control 1", '\u{11}'),
    ("DC2", '\u{12}'),("device control 2", '\u{12}'),
    ("DC3", '\u{13}'),("device control 3", '\u{13}'),
    ("DC4", '\u{14}'),("device control 4", '\u{14}'),
    ("NAK", '\u{15}'),("negative acknowledge", '\u{15}'),
    ("SYN", '\u{16}'),("synchronous idle", '\u{16}'),
    ("ETB", '\u{17}'),("end of transmission block", '\u{17}'),
    ("CAN", '\u{18}'),("cancel", '\u{18}'),
    ("EM", '\u{19}'),("end of medium", '\u{19}'),
    ("SUB", '\u{1a}'),("substitute", '\u{1a}'),
    ("ESC", '\u{1b}'),("escape", '\u{1b}'),
    ("FS", '\u{1c}'),("file separator", '\u{1c}'),
    ("GS", '\u{1d}'),("group separator", '\u{1d}'),
    ("RS", '\u{1e}'),("record separator", '\u{1e}'),
    ("US", '\u{1f}'),("unit separator", '\u{1f}'),
    ("SP", ' '),("space", ' '),("blank", ' '),
    ("exclamation mark", '!'),("bang", '!'),("excl", '!'),("wow", '!'),("factorial", '!'),("shriek", '!'),("pling", '!'),("smash", '!'),("cuss", '!'),
    ("\\\"", '\"'),("quotation mark", '\"'),("double quote", '\"'),("quote", '\"'),("string quote", '\"'),("dirk", '\"'),("literal mark", '\"'),("double glitch", '\"'),("&quot;", '\"'),
    ("number sign", '#'),("number", '#'),("pound", '#'),("number", '#'),("sharp", '#'),("crunch", '#'),("mesh", '#'),("hex", '#'),("hash", '#'),("flash", '#'),("grid", '#'),("octothorpe", '#'),
    ("currency sign", '$'),("currency", '$'),("dollar", '$'),("buck", '$'),("cash", '$'),("ding", '$'),
    ("percent sign", '%'),("percent", '%'),("mod", '%'),("modulo", '%'),
    ("ampersand", '&'),("amper", '&'),("and", '&'),("&amp;", '&'),
    ("apostrophe", '\''),("single quote", '\''),("close quote", '\''),("prime", '\''),("tick", '\''),("pop", '\''),("spark", '\''),("glitch", '\''),("&apos;", '\''),
    ("left parenthesis", '('),("open", '('),("open paren", '('),("left paren", '('),("wax", '('),("sad", '('),
    ("right parenthesis", ')'),("close", ')'),("close paren", ')'),("right paren", ')'),("wane", ')'),("happy", ')'),
    ("asterisk", '*'),("star", '*'),("splat", '*'),("aster", '*'),("times", '*'),("gear", '*'),("dingle", '*'),("bug", '*'),("twinkle", '*'),("glob\" ,", '*'),
    ("plus sign", '+'),("plus", '+'),("add", '+'),("cross", '+'),
    ("comma", ','),("tail", ','),
    ("hyphen", '-'),("dash", '-'),("minus", '-'),("worm", '-'),
    ("full stop", '.'),("dot", '.'),("decimal point", '.'),("radix point", '.'),("point", '.'),("period", '.'),("spot", '.'),
    ("solidus", '/'),("slash", '/'),("stroke", '/'),("slant", '/'),("diagonal", '/'),("virgule", '/'),("over", '/'),("slat", '/'),
    ("digit zero", '0'),("zero", '0'),
    ("digit one", '1'),("one", '1'),
    ("digit two", '2'),("two", '2'),
    ("digit three", '3'),("three", '3'),
    ("digit four", '4'),("four", '4'),
    ("digit five", '5'),("five", '5'),
    ("digit six", '6'),("six", '6'),
    ("digit seven", '7'),("seven", '7'),
    ("digit eight", '8'),("eight", '8'),
    ("digit nine", '9'),("nine", '9'),
    ("colon", ':'),("double-dot", ':'),
    ("semicolon", ';'),("semi", ';'),("go-on", ';'),
    ("less-than sign", '<'),("less-than", '<'),("left angle bracket", '<'),("read from", '<'),("in", '<'),("from", '<'),("comesfrom", '<'),("left funnel", '<'),("left broket", '<'),("crunch", '<'),("suck", '<'),("&lt;", '<'),
    ("equals sign", '='),("equals", '='),("quadrathorp", '='),("gets", '='),("becomes", '='),("half-mesh", '='),
    ("greater-than sign", '>'),("greater-than", '>'),("right angle bracket", '>'),("write to", '>'),("into", '>'),("toward", '>'),("out", '>'),("to", '>'),("gozinta", '>'),("right funnel", '>'),("right broket", '>'),("zap", '>'),("blow", '>'),("&gt;", '>'),
    ("question mark", '?'),("whatmark", '?'),("what", '?'),("ques", '?'),
    ("commercial at", '@'),("at", '@'),("each", '@'),("vortex", '@'),("whorl", '@'),("whirlpool", '@'),("cyclone", '@'),("snail", '@'),("rose", '@'),
    ("majuscule a", 'A'),("capital a", 'A'),("uppercase a", 'A'),
    ("majuscule b", 'B'),("capital b", 'B'),("uppercase b", 'B'),
    ("majuscule c", 'C'),("capital c", 'C'),("uppercase c", 'C'),
    ("majuscule d", 'D'),("capital d", 'D'),("uppercase d", 'D'),
    ("majuscule e", 'E'),("capital e", 'E'),("uppercase e", 'E'),
    ("majuscule f", 'F'),("capital f", 'F'),("uppercase f", 'F'),
    ("majuscule g", 'G'),("capital g", 'G'),("uppercase g", 'G'),
    ("majuscule h", 'H'),("capital h", 'H'),("uppercase h", 'H'),
    ("majuscule i", 'I'),("capital i", 'I'),("uppercase i", 'I'),
    ("majuscule j", 'J'),("capital j", 'J'),("uppercase j", 'J'),
    ("majuscule k", 'K'),("capital k", 'K'),("uppercase k", 'K'),
    ("majuscule l", 'L'),("capital l", 'L'),("uppercase l", 'L'),
    ("majuscule m", 'M'),("capital m", 'M'),("uppercase m", 'M'),
    ("majuscule n", 'N'),("capital n", 'N'),("uppercase n", 'N'),
    ("majuscule o", 'O'),("capital o", 'O'),("uppercase o", 'O'),
    ("majuscule p", 'P'),("capital p", 'P'),("uppercase p", 'P'),
    ("majuscule q", 'Q'),("capital q", 'Q'),("uppercase q", 'Q'),
    ("majuscule r", 'R'),("capital r", 'R'),("uppercase r", 'R'),
    ("majuscule s", 'S'),("capital s", 'S'),("uppercase s", 'S'),
    ("majuscule t", 'T'),("capital t", 'T'),("uppercase t", 'T'),
    ("majuscule u", 'U'),("capital u", 'U'),("uppercase u", 'U'),
    ("majuscule v", 'V'),("capital v", 'V'),("uppercase v", 'V'),
    ("majuscule w", 'W'),("capital w", 'W'),("uppercase w", 'W'),
    ("majuscule x", 'X'),("capital x", 'X'),("uppercase x", 'X'),
    ("majuscule y", 'Y'),("capital y", 'Y'),("uppercase y", 'Y'),
    ("majuscule z", 'Z'),("capital z", 'Z'),("uppercase z", 'Z'),
    ("left square bracket", '['),("bracket", '['),("bra", '['),("square", '['),
    ("\\\\", '\\'),("reversed solidus", '\\'),("backslash", '\\'),("bash", '\\'),("backslant", '\\'),("backwhack", '\\'),("backslat", '\\'),("literal", '\\'),("escape", '\\'),
    ("right square bracket", ']'),("unbracket", ']'),("ket", ']'),("unsquare", ']'),
    ("circumflex accent", '^'),("circumflex", '^'),("caret", '^'),("uparrow", '^'),("hat", '^'),("control", '^'),("boink", '^'),("chevron", '^'),("hiccup", '^'),("sharkfin", '^'),("fang", '^'),
    ("low line", '_'),("underscore", '_'),("underline", '_'),("underbar", '_'),("under", '_'),("score", '_'),("backarrow", '_'),("flatworm", '_'),
    ("grave accent", '`'),("grave", '`'),("backquote", '`'),("left quote", '`'),("open quote", '`'),("backprime", '`'),("unapostrophe", '`'),("backspark", '`'),("birk", '`'),("blugle", '`'),("back tick", '`'),("push", '`'),
    ("miniscule a", 'a'),("small a", 'a'),("lowercase a", 'a'),
    ("miniscule b", 'b'),("small b", 'b'),("lowercase b", 'b'),
    ("miniscule c", 'c'),("small c", 'c'),("lowercase c", 'c'),
    ("miniscule d", 'd'),("small d", 'd'),("lowercase d", 'd'),
    ("miniscule e", 'e'),("small e", 'e'),("lowercase e", 'e'),
    ("miniscule f", 'f'),("small f", 'f'),("lowercase f", 'f'),
    ("miniscule g", 'g'),("small g", 'g'),("lowercase g", 'g'),
    ("miniscule h", 'h'),("small h", 'h'),("lowercase h", 'h'),
    ("miniscule i", 'i'),("small i", 'i'),("lowercase i", 'i'),
    ("miniscule j", 'j'),("small j", 'j'),("lowercase j", 'j'),
    ("miniscule k", 'k'),("small k", 'k'),("lowercase k", 'k'),
    ("miniscule l", 'l'),("small l", 'l'),("lowercase l", 'l'),
    ("miniscule m", 'm'),("small m", 'm'),("lowercase m", 'm'),
    ("miniscule n", 'n'),("small n", 'n'),("lowercase n", 'n'),
    ("miniscule o", 'o'),("small o", 'o'),("lowercase o", 'o'),
    ("miniscule p", 'p'),("small p", 'p'),("lowercase p", 'p'),
    ("miniscule q", 'q'),("small q", 'q'),("lowercase q", 'q'),
    ("miniscule r", 'r'),("small r", 'r'),("lowercase r", 'r'),
    ("miniscule s", 's'),("small s", 's'),("lowercase s", 's'),
    ("miniscule t", 't'),("small t", 't'),("lowercase t", 't'),
    ("miniscule u", 'u'),("small u", 'u'),("lowercase u", 'u'),
    ("miniscule v", 'v'),("small v", 'v'),("lowercase v", 'v'),
    ("miniscule w", 'w'),("small w", 'w'),("lowercase w", 'w'),
    ("miniscule x", 'x'),("small x", 'x'),("lowercase x", 'x'),
    ("miniscule y", 'y'),("small y", 'y'),("lowercase y", 'y'),
    ("miniscule z", 'z'),("small z", 'z'),("lowercase z", 'z'),
    ("left curly bracket", '{'),("left brace", '{'),("brace", '{'),("open brace", '{'),("curly", '{'),("leftit", '{'),("embrace", '{'),
    ("vertical line", '|'),("pipe", '|'),("bar", '|'),("or", '|'),("v-bar", '|'),("spike", '|'),("gozinta", '|'),("thru", '|'),
    ("right curly bracket", '}'),("right brace", '}'),("unbrace", '}'),("close brace", '}'),("uncurly", '}'),("rytit", '}'),("bracelet", '}'),
    ("overline", '~'),("tilde", '~'),("swung dash", '~'),("squiggle", '~'),("approx", '~'),("wiggle", '~'),("twiddle", '~'),("enyay", '~'),
];