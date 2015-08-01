import parser

def gen_header():
    print "// This file was generated automatically."
    print "// See the gen/ folder at the project root."
    print
    print "use std::fmt;"
    print "use std::str;"

def gen_enum(codes):
    print "/// Representation of IRC commands, replies and errors."
    print "#[derive(Clone, Debug, Eq, PartialEq)]"
    print "pub enum Code {"
    for code in codes:
        print "    /// " + code.code + ' = "' + code.value + '"'
        print "    " + code.format_code + ","
    print "    /// Codes that are unknown end up in here."
    print "    Unknown(String),"
    print "}"

def gen_methods(codes):
    print "impl Code {"
    print
    print "    /// Checks if the code is a reply."
    print "    pub fn is_reply(&self) -> bool {"
    print "        match *self {"
    for code in codes:
        if not code.reply: continue
        print "            Code::" + code.format_code + " => true,"
    print "            _  => false,"
    print "        }"
    print "    }"
    print
    print "    /// Check if the code is en error."
    print "    pub fn is_error(&self) -> bool {"
    print "        match *self {"
    for code in codes:
        if not code.error: continue
        print "            Code::" + code.format_code + " => true,"
    print "            _  => false,"
    print "        }"
    print "    }"
    print
    print "}"

def gen_display(codes):
    print "impl fmt::Display for Code {"
    print
    print "    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {"
    print "        let text = match *self {"
    for code in codes:
        print "            Code::" + code.format_code + " => " + code.format_value + ","
    print "            Code::Unknown(ref text) => &text[..],"
    print "        };"
    print "        f.write_str(text)"
    print "    }"
    print
    print "}"

def gen_fromstr(codes):
    print "impl str::FromStr for Code {"
    print "    type Err = ();"
    print
    print "    fn from_str(s: &str) -> Result<Code, ()> {"
    print "        let code = match s {"
    for code in codes:
        print "            " + code.format_value + " => Code::" + code.format_code + ","
    print "            _ => Code::Unknown(s.to_string()),"
    print "        };"
    print "        Ok(code)"
    print "    }"
    print "}"

if __name__ == '__main__':
    codes = parser.parse("codes.txt")
    gen_header()
    print
    gen_enum(codes)
    print
    gen_methods(codes)
    print
    gen_display(codes)
    print
    gen_fromstr(codes)
