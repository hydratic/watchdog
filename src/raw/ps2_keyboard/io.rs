#![no_std]

extern crate cpuio;

macro_rules! get_io {
	($in:expr) => {{
		let scancode = KEYBOARD.lock().read();
		let mut caps = in.split(".");
		
		if caps == 0 {
            match scancode {
                "1C" => letter = "a",
                "F01C" => letter = "BREAK",
                "32" => letter = "b",
                "F032" => letter = "BREAK",
                "21" => letter = "c",
                "F021" => letter = "BREAK",
                "23" => letter = "d",
                "F023" => letter = "BREAK",
                "24" => letter = "e",
                "F024" => letter = "BREAK",
                "2B" => letter = "f",
                "F02B" => letter = "BREAK",
                "34" => letter = "g",
                "F034" => letter = "BREAK",
                "33" => letter = "h",
                "F034" => letter = "BREAK",
                "43" => letter = "i",
                "F043" => letter = "BREAK",
                "3B" => letter = "j"
                "F03B" => letter = "BREAK"
                "42" => letter = "k";
                "F042" => letter = "BREAK";
                "4B" => letter = "l";
                "F04B" => letter = "BREAK";
                "3A" => letter = "m";
                "F03A" => letter = "BREAK";
                "31" => letter = "n";
                "F031" => letter = "BREAK";
                "44" => letter = "o";
                "F044" => letter = "BREAK";
                "4D" => letter = "p";
                "F04D" => letter = "BREAK";
                "15" => letter = "q";
                "F015" => letter = "BREAK";
                "2D" => letter = "r";
                "F02D" => letter = "BREAK";
                "1B" => letter = "s";
                "F01B" => letter = "BREAK";
                "2C" => letter = "t";
                "F02C" => letter = "BREAK";
                "3C" => letter = "u";
                "F03C" => letter = "BREAK";
                "2A" => letter = "v";
                "F02A" => letter = "BREAK";
                "1D" => letter = "w";
                "F01D" => letter = "BREAK";
                "22" => letter = "x";
                "F022" => letter = "BREAK";
                "35" => letter = "y";
                "F035" => letter = "BREAK";
                "1A" => letter = "z";
                "F01A" => letter = "BREAK";
            }
        }

        if caps == 1 {
            match scancode {
                "1C" => letter = "A"; // make code
                "F01C" => letter = "BREAK"; // break code
                "32" => letter = "B"; // repeat
                "F032" => letter = "BREAK";
                "21" => letter = "C";
                "F021" => letter = "BREAK";
                "23" => letter = "D";
                "F023" => letter = "BREAK";
                "24" => letter = "E";
                "F024" => letter = "BREAK";
                "2B" => letter = "F";
                "F02B" => letter = "BREAK";
                "34" => letter = "G";
                "F034" => letter = "BREAK";
                "33" => letter = "H";
                "F034" => letter = "BREAK";
                "43" => letter = "I";
                "F043" => letter = "BREAK";
                "3B" => letter = "J";
                "F03B" => letter = "BREAK";
                "42" => letter = "K";
                "F042" => letter = "BREAK";
                "4B" => letter = "L";
                "F04B" => letter = "BREAK";
                "3A" => letter = "M";
                "F03A" => letter = "BREAK";
                "31" => letter = "N";
                "F031" => letter = "BREAK";
                "44" => letter = "O";
                "F044" => letter = "BREAK";
                "4D" => letter = "P";
                "F04D" => letter = "BREAK";
                "15" => letter = "Q";
                "F015" => letter = "BREAK";
                "2D" => letter = "R";
                "F02D" => letter = "BREAK";
                "1B" => letter = "S";
                "F01B" => letter = "BREAK";
                "2C" => letter = "T";
                "F02C" => letter = "BREAK";
                "3C" => letter = "U";
                "F03C" => letter = "BREAK";
                "2A" => letter = "V";
                "F02A" => letter = "BREAK";
                "1D" => letter = "W";
                "F01D" => letter = "BREAK";
                "22" => letter = "X";
                "F022" => letter = "BREAK";
                "35" => letter = "Y";
                "F035" => letter = "BREAK";
                "1A" => letter = "Z";
                "F01A" => letter = "BREAK";
            }
	}};
}
