extern crate base64;


pub fn challenge_1(){
    println!("Please enter hex string :");
    let input_stream = std::io::stdin();            //getting stdin stream
    let mut line = String::new();
    input_stream.read_line(&mut line).unwrap();     //reading a line : .unwrap() THIS WILL ENFORCE 8 BITS UTF-8 (thats a good thing, but I'd like to know more about this function)
    line = line.trim().to_string();                 //trimming the line (contains a trailing \n)
    let b64 = hex_string_to_b64(&line);             //converting given string (expectedly hex string) into base64

    println!("{}", b64);
}



pub fn hex_string_to_b64(s: &String) -> String{
    let hex:Vec<u8> = hex_string_to_bytes(s).unwrap();       //converting given hex string into byte equivalents. e.g : "aa" would be x'aa', or b'10101010', or 170_u8
    let b64:String = b64_encode(hex);                        //encoding bytes into base64 string
    b64
}


//This will iterate through all couples of hexadecimal characters of the given string, turning each character into their numeric equivalent and combining each couple into a single byte. 
pub fn hex_string_to_bytes(hex: &String) -> Result<Vec<u8>, String>{
    
    if hex.len()%2 != 0 || hex.len() != hex.chars().count() {
        //String::len returns the length of the string in bytes, not the number of characters.
        //The character encoding is UTF-8, characters are not all the same size.
        //All hex characters (0123456789, abcdef, ABCDEF) should all be 1 byte long.
        //So, if all characters are indeed hex characters, the byte length should be the same as character length
        //And every 2 hex character encodes a byte
        //So the length of the byte vector should be half that of the hex string
        //Thus, so should its capacity if we want to optimise memory usage
        return Err("Invalid hex string.".to_string());
    }
    let mut r:Vec<u8> = Vec::with_capacity(hex.len()/2);
    for byte_hex in hex.chars().collect::<Vec<char>>().windows(2).step_by(2) {
        r.push(make_byte_from_hex(byte_hex[0], byte_hex[1]));          //second byte => 4 least signifiant bits. leave them in the least significant half. merge (bitwise OR) with other half.
    }
    Ok(r)
}

pub fn make_byte_from_hex(hex1: char, hex2: char) -> u8 {
    ((hex1.to_digit(16).unwrap() as u8) << 4) | (hex2.to_digit(16).unwrap() as u8)
}

//equivalent to char::to_digit(16)
//except to_digit will actually check if the character is hexadecimal
//this function will accept a character that's over 1 byte long if its least significant byte has the same value as '0123456789', 'abcdef' of 'ABCDEF'
//for instance, this function will think '〱' is a '1' because its lsb has value 49, which is '1' : https://www.w3schools.com/charsets/ref_utf_basic_latin.asp
fn match_hex_val_to_bits(hex_char: char) -> Result<u8, String> {
    println!("{}", hex_char as u8);
    let shift = match hex_char as u8 {
        48..=57     => Some(48),
        97..=102    => Some(87),
        65..=70     => Some(55),
        _           => None,
    };
    println!("shift = {}", shift.unwrap());
    match shift {
        Some(n) => Ok(hex_char as u8 - n),
        None    => Err(format!("Character {} is not hexadecimal", hex_char))
    }
}




/*
 *  Base 64 encoding ahead
 * 
 */



pub fn b64_encode(bytes: Vec<u8>) -> String {
    let b64_word_count = (((bytes.len() as f64)/3.) * 4.).ceil() as usize;

    let mut b64_words:Vec<u8> = Vec::with_capacity(b64_word_count);

    let mut padding:u8 = 0;

    for i in 0..b64_word_count {
        let start_bit = (i*6)%24;
        let end_bit = start_bit + 5;
        let b1:u8 = bytes[(i/4)*3 + start_bit/8];
        let mut b2:u8;
        if bytes.len() > ((i/4)*3 + end_bit/8) {
            b2 = bytes[(i/4)*3 + end_bit/8];
        }
        else {
            b2 = 0;
            if i%2 == 0{
                padding = 1;
            }
            else {
                padding = 2;
            }
        }
        let window_shift = (((i+1)*2) % 8) as u8;
        b64_words.push(make_b64_char(b1, b2, window_shift));
    }


    let mut out:String = String::new();

    for word in b64_words {
        out.push(b64_word_to_char(word).unwrap());
    }

    for i in 0..padding {
        out.push('=');
    }

    return out
}





pub fn b64_word_to_char(word: u8) -> Result<char, String> {
    match word {
        0..=25  => Ok((word + 65) as char),
        26..=51 => Ok((word + 71) as char),
        52..=61 => Ok((word - 4) as char),
        62      => Ok('+'),
        63      => Ok('/'),
        _       => Err("invalid base64 value (>63)".to_string()),
  }
}

pub fn make_b64_char(b1: u8, b2: u8, window_slide: u8) -> u8 {
    ((((b1 as u16) << 8 | (b2 as u16)) >> window_slide) as u8) & 0b00111111
}
