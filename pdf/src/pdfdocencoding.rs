/// PDFDocEncoding character mapping table for bytes 128-255
/// Characters 0-127 are identical to ASCII
/// This table maps byte values 128-255 to their corresponding Unicode code points
/// as defined in the PDF specification (Appendix D.2)
const PDFDOCENCODING_TABLE: [u16; 128] = [
    0x2022, // 128: BULLET
    0x2020, // 129: DAGGER
    0x2021, // 130: DOUBLE DAGGER
    0x2026, // 131: HORIZONTAL ELLIPSIS
    0x2014, // 132: EM DASH
    0x2013, // 133: EN DASH
    0x0192, // 134: LATIN SMALL LETTER F WITH HOOK
    0x2044, // 135: FRACTION SLASH
    0x2039, // 136: SINGLE LEFT-POINTING ANGLE QUOTATION MARK
    0x203A, // 137: SINGLE RIGHT-POINTING ANGLE QUOTATION MARK
    0x2212, // 138: MINUS SIGN
    0x2030, // 139: PER MILLE SIGN
    0x201E, // 140: DOUBLE LOW-9 QUOTATION MARK
    0x201C, // 141: LEFT DOUBLE QUOTATION MARK
    0x201D, // 142: RIGHT DOUBLE QUOTATION MARK
    0x2018, // 143: LEFT SINGLE QUOTATION MARK
    0x2019, // 144: RIGHT SINGLE QUOTATION MARK
    0x201A, // 145: SINGLE LOW-9 QUOTATION MARK
    0x2122, // 146: TRADE MARK SIGN
    0xFB01, // 147: LATIN SMALL LIGATURE FI
    0xFB02, // 148: LATIN SMALL LIGATURE FL
    0x0141, // 149: LATIN CAPITAL LETTER L WITH STROKE
    0x0152, // 150: LATIN CAPITAL LIGATURE OE
    0x0160, // 151: LATIN CAPITAL LETTER S WITH CARON
    0x0178, // 152: LATIN CAPITAL LETTER Y WITH DIAERESIS
    0x017D, // 153: LATIN CAPITAL LETTER Z WITH CARON
    0x0131, // 154: LATIN SMALL LETTER DOTLESS I
    0x0142, // 155: LATIN SMALL LETTER L WITH STROKE
    0x0153, // 156: LATIN SMALL LIGATURE OE
    0x0161, // 157: LATIN SMALL LETTER S WITH CARON
    0x017E, // 158: LATIN SMALL LETTER Z WITH CARON
    0xFFFF, // 159: undefined
    0x00A0, // 160: NO-BREAK SPACE
    0x00A1, // 161: INVERTED EXCLAMATION MARK
    0x00A2, // 162: CENT SIGN
    0x00A3, // 163: POUND SIGN
    0x00A4, // 164: CURRENCY SIGN
    0x00A5, // 165: YEN SIGN
    0x00A6, // 166: BROKEN BAR
    0x00A7, // 167: SECTION SIGN
    0x00A8, // 168: DIAERESIS
    0x00A9, // 169: COPYRIGHT SIGN
    0x00AA, // 170: FEMININE ORDINAL INDICATOR
    0x00AB, // 171: LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
    0x00AC, // 172: NOT SIGN
    0x00AD, // 173: SOFT HYPHEN
    0x00AE, // 174: REGISTERED SIGN
    0x00AF, // 175: MACRON
    0x00B0, // 176: DEGREE SIGN
    0x00B1, // 177: PLUS-MINUS SIGN
    0x00B2, // 178: SUPERSCRIPT TWO
    0x00B3, // 179: SUPERSCRIPT THREE
    0x00B4, // 180: ACUTE ACCENT
    0x00B5, // 181: MICRO SIGN
    0x00B6, // 182: PILCROW SIGN
    0x00B7, // 183: MIDDLE DOT
    0x00B8, // 184: CEDILLA
    0x00B9, // 185: SUPERSCRIPT ONE
    0x00BA, // 186: MASCULINE ORDINAL INDICATOR
    0x00BB, // 187: RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
    0x00BC, // 188: VULGAR FRACTION ONE QUARTER
    0x00BD, // 189: VULGAR FRACTION ONE HALF
    0x00BE, // 190: VULGAR FRACTION THREE QUARTERS
    0x00BF, // 191: INVERTED QUESTION MARK
    0x00C0, // 192: LATIN CAPITAL LETTER A WITH GRAVE
    0x00C1, // 193: LATIN CAPITAL LETTER A WITH ACUTE
    0x00C2, // 194: LATIN CAPITAL LETTER A WITH CIRCUMFLEX
    0x00C3, // 195: LATIN CAPITAL LETTER A WITH TILDE
    0x00C4, // 196: LATIN CAPITAL LETTER A WITH DIAERESIS
    0x00C5, // 197: LATIN CAPITAL LETTER A WITH RING ABOVE
    0x00C6, // 198: LATIN CAPITAL LETTER AE
    0x00C7, // 199: LATIN CAPITAL LETTER C WITH CEDILLA
    0x00C8, // 200: LATIN CAPITAL LETTER E WITH GRAVE
    0x00C9, // 201: LATIN CAPITAL LETTER E WITH ACUTE
    0x00CA, // 202: LATIN CAPITAL LETTER E WITH CIRCUMFLEX
    0x00CB, // 203: LATIN CAPITAL LETTER E WITH DIAERESIS
    0x00CC, // 204: LATIN CAPITAL LETTER I WITH GRAVE
    0x00CD, // 205: LATIN CAPITAL LETTER I WITH ACUTE
    0x00CE, // 206: LATIN CAPITAL LETTER I WITH CIRCUMFLEX
    0x00CF, // 207: LATIN CAPITAL LETTER I WITH DIAERESIS
    0x00D0, // 208: LATIN CAPITAL LETTER ETH
    0x00D1, // 209: LATIN CAPITAL LETTER N WITH TILDE
    0x00D2, // 210: LATIN CAPITAL LETTER O WITH GRAVE
    0x00D3, // 211: LATIN CAPITAL LETTER O WITH ACUTE
    0x00D4, // 212: LATIN CAPITAL LETTER O WITH CIRCUMFLEX
    0x00D5, // 213: LATIN CAPITAL LETTER O WITH TILDE
    0x00D6, // 214: LATIN CAPITAL LETTER O WITH DIAERESIS
    0x00D7, // 215: MULTIPLICATION SIGN
    0x00D8, // 216: LATIN CAPITAL LETTER O WITH STROKE
    0x00D9, // 217: LATIN CAPITAL LETTER U WITH GRAVE
    0x00DA, // 218: LATIN CAPITAL LETTER U WITH ACUTE
    0x00DB, // 219: LATIN CAPITAL LETTER U WITH CIRCUMFLEX
    0x00DC, // 220: LATIN CAPITAL LETTER U WITH DIAERESIS
    0x00DD, // 221: LATIN CAPITAL LETTER Y WITH ACUTE
    0x00DE, // 222: LATIN CAPITAL LETTER THORN
    0x00DF, // 223: LATIN SMALL LETTER SHARP S
    0x00E0, // 224: LATIN SMALL LETTER A WITH GRAVE
    0x00E1, // 225: LATIN SMALL LETTER A WITH ACUTE
    0x00E2, // 226: LATIN SMALL LETTER A WITH CIRCUMFLEX
    0x00E3, // 227: LATIN SMALL LETTER A WITH TILDE
    0x00E4, // 228: LATIN SMALL LETTER A WITH DIAERESIS
    0x00E5, // 229: LATIN SMALL LETTER A WITH RING ABOVE
    0x00E6, // 230: LATIN SMALL LETTER AE
    0x00E7, // 231: LATIN SMALL LETTER C WITH CEDILLA
    0x00E8, // 232: LATIN SMALL LETTER E WITH GRAVE
    0x00E9, // 233: LATIN SMALL LETTER E WITH ACUTE
    0x00EA, // 234: LATIN SMALL LETTER E WITH CIRCUMFLEX
    0x00EB, // 235: LATIN SMALL LETTER E WITH DIAERESIS
    0x00EC, // 236: LATIN SMALL LETTER I WITH GRAVE
    0x00ED, // 237: LATIN SMALL LETTER I WITH ACUTE
    0x00EE, // 238: LATIN SMALL LETTER I WITH CIRCUMFLEX
    0x00EF, // 239: LATIN SMALL LETTER I WITH DIAERESIS
    0x00F0, // 240: LATIN SMALL LETTER ETH
    0x00F1, // 241: LATIN SMALL LETTER N WITH TILDE
    0x00F2, // 242: LATIN SMALL LETTER O WITH GRAVE
    0x00F3, // 243: LATIN SMALL LETTER O WITH ACUTE
    0x00F4, // 244: LATIN SMALL LETTER O WITH CIRCUMFLEX
    0x00F5, // 245: LATIN SMALL LETTER O WITH TILDE
    0x00F6, // 246: LATIN SMALL LETTER O WITH DIAERESIS
    0x00F7, // 247: DIVISION SIGN
    0x00F8, // 248: LATIN SMALL LETTER O WITH STROKE
    0x00F9, // 249: LATIN SMALL LETTER U WITH GRAVE
    0x00FA, // 250: LATIN SMALL LETTER U WITH ACUTE
    0x00FB, // 251: LATIN SMALL LETTER U WITH CIRCUMFLEX
    0x00FC, // 252: LATIN SMALL LETTER U WITH DIAERESIS
    0x00FD, // 253: LATIN SMALL LETTER Y WITH ACUTE
    0x00FE, // 254: LATIN SMALL LETTER THORN
    0x00FF, // 255: LATIN SMALL LETTER Y WITH DIAERESIS
];

/// Build a reverse mapping from Unicode code points to PDFDocEncoding bytes
/// This is used to encode Unicode text into PDFDocEncoding when possible
fn build_unicode_to_pdfdoc_map() -> Vec<(u16, u8)> {
    let mut map = Vec::new();
    
    // Characters 0-127 are identical to ASCII (same as UTF-8)
    for i in 0u8..128 {
        map.push((i as u16, i));
    }
    
    // Characters 128-255 use the special PDFDocEncoding mapping
    for (i, &unicode) in PDFDOCENCODING_TABLE.iter().enumerate() {
        if unicode != 0xFFFF {  // Skip undefined character at position 159
            map.push((unicode, (i + 128) as u8));
        }
    }
    
    map
}

/// Try to encode a Unicode character as PDFDocEncoding
/// Returns Some(byte) if the character can be represented in PDFDocEncoding,
/// None otherwise
fn unicode_to_pdfdoc(ch: char) -> Option<u8> {
    let code_point = ch as u32;
    
    // ASCII range (0-127) is directly representable
    if code_point < 128 {
        return Some(code_point as u8);
    }
    
    // Check if the character is in the PDFDocEncoding table (128-255)
    if code_point <= 0xFFFF {
        let code_point_u16 = code_point as u16;
        for (i, &table_code_point) in PDFDOCENCODING_TABLE.iter().enumerate() {
            if table_code_point == code_point_u16 {
                return Some((i + 128) as u8);
            }
        }
    }
    
    None
}

/// Check if a string can be fully represented in PDFDocEncoding
fn can_use_pdfdocencoding(s: &str) -> bool {
    s.chars().all(|ch| unicode_to_pdfdoc(ch).is_some())
}

/// Encode a UTF-8 string to UTF-16BE with BOM
/// This is used when the string contains characters that cannot be represented in PDFDocEncoding
fn encode_utf16be_with_bom(s: &str) -> Vec<u8> {
    let mut result = vec![0xFE, 0xFF]; // UTF-16BE BOM
    
    for ch in s.chars() {
        let code_point = ch as u32;
        
        if code_point <= 0xFFFF {
            // Basic Multilingual Plane - single UTF-16 code unit
            result.push((code_point >> 8) as u8);
            result.push((code_point & 0xFF) as u8);
        } else {
            // Supplementary planes - surrogate pair
            let code_point = code_point - 0x10000;
            let high = 0xD800 + ((code_point >> 10) & 0x3FF);
            let low = 0xDC00 + (code_point & 0x3FF);
            
            result.push((high >> 8) as u8);
            result.push((high & 0xFF) as u8);
            result.push((low >> 8) as u8);
            result.push((low & 0xFF) as u8);
        }
    }
    
    result
}

/// Convert a UTF-8 string to bytes suitable for a PDF string
/// 
/// This function handles the conversion according to PDF specification:
/// - If all characters can be represented in PDFDocEncoding, use that encoding
/// - Otherwise, use UTF-16BE with BOM (byte order mark)
///
/// PDFDocEncoding is a superset of ASCII for the first 128 characters,
/// with special character mappings for bytes 128-255.
pub fn utf8_to_pdf_string_bytes(s: &str) -> Vec<u8> {
    if can_use_pdfdocencoding(s) {
        // All characters can be represented in PDFDocEncoding
        s.chars()
            .map(|ch| unicode_to_pdfdoc(ch).expect("Character should be in PDFDocEncoding"))
            .collect()
    } else {
        // Contains characters outside PDFDocEncoding, use UTF-16BE with BOM
        encode_utf16be_with_bom(s)
    }
}

/// Decode PDFDocEncoding bytes to a UTF-8 String
///
/// This function handles decoding according to PDF specification:
/// - If the data starts with UTF-16BE BOM (0xFE 0xFF), decode as UTF-16BE
/// - Otherwise, first try to decode as UTF-8 (for compatibility)
/// - If that fails, decode as PDFDocEncoding
///
/// PDFDocEncoding is identical to ASCII for bytes 0-127,
/// and has special character mappings for bytes 128-255.
pub fn pdf_string_bytes_to_utf8(bytes: &[u8]) -> String {
    // Check for UTF-16BE BOM
    if bytes.starts_with(&[0xFE, 0xFF]) {
        // Decode UTF-16BE
        return decode_utf16be(&bytes[2..]);
    }
    
    // Try UTF-8 first for backward compatibility
    // Many PDFs incorrectly use UTF-8 encoding instead of PDFDocEncoding
    if let Ok(s) = std::str::from_utf8(bytes) {
        return s.to_string();
    }
    
    // Fall back to PDFDocEncoding
    bytes.iter().map(|&b| {
        if b < 128 {
            // ASCII range - direct mapping
            b as char
        } else {
            // Look up in PDFDocEncoding table
            let table_index = (b as usize) - 128;
            let unicode_value = PDFDOCENCODING_TABLE[table_index];
            if unicode_value == 0xFFFF {
                // Undefined character at position 159
                std::char::REPLACEMENT_CHARACTER
            } else {
                std::char::from_u32(unicode_value as u32)
                    .unwrap_or(std::char::REPLACEMENT_CHARACTER)
            }
        }
    }).collect()
}

/// Decode UTF-16BE bytes to a UTF-8 String
/// Returns an error message if the decoding fails
fn decode_utf16be(bytes: &[u8]) -> String {
    let mut result = String::new();
    let mut i = 0;
    let mut had_error = false;
    
    while i + 1 < bytes.len() {
        let high = ((bytes[i] as u16) << 8) | (bytes[i + 1] as u16);
        i += 2;
        
        // Check if this is a high surrogate (0xD800-0xDBFF)
        if (0xD800..=0xDBFF).contains(&high) {
            // Need a low surrogate
            if i + 1 < bytes.len() {
                let low = ((bytes[i] as u16) << 8) | (bytes[i + 1] as u16);
                
                // Check if this is a valid low surrogate (0xDC00-0xDFFF)
                if (0xDC00..=0xDFFF).contains(&low) {
                    i += 2;
                    // Decode surrogate pair
                    let code_point = 0x10000 + (((high - 0xD800) as u32) << 10) + ((low - 0xDC00) as u32);
                    if let Some(ch) = std::char::from_u32(code_point) {
                        result.push(ch);
                    } else {
                        result.push(std::char::REPLACEMENT_CHARACTER);
                        had_error = true;
                    }
                } else {
                    // Invalid surrogate pair - high surrogate without valid low surrogate
                    // Don't consume the next two bytes as they might be a valid character
                    if !had_error {
                        result.push(std::char::REPLACEMENT_CHARACTER);
                        had_error = true;
                    }
                    // Continue to next iteration which will process those bytes
                }
            } else {
                // Incomplete surrogate pair - high surrogate at end, might have odd byte
                // Only add one replacement character for the whole error
                if !had_error {
                    result.push(std::char::REPLACEMENT_CHARACTER);
                    had_error = true;
                }
                break;  // Exit loop, will handle trailing byte below if needed
            }
        } else if (0xDC00..=0xDFFF).contains(&high) {
            // Low surrogate without high surrogate
            if !had_error {
                result.push(std::char::REPLACEMENT_CHARACTER);
                had_error = true;
            }
        } else {
            // Normal character
            if let Some(ch) = std::char::from_u32(high as u32) {
                result.push(ch);
                had_error = false;  // Reset error flag on successful decode
            } else {
                if !had_error {
                    result.push(std::char::REPLACEMENT_CHARACTER);
                    had_error = true;
                }
            }
        }
    }
    
    // If there's a trailing odd byte and we haven't already reported an error, add replacement
    // But if we just had an error (incomplete surrogate), don't add another replacement
    if i < bytes.len() && !had_error {
        result.push(std::char::REPLACEMENT_CHARACTER);
    }
    
    result
}

/// Decode PDFDocEncoding or UTF-16BE bytes to UTF-8, returning Result
/// This variant returns an error for invalid sequences (used by to_string())
pub fn pdf_string_bytes_to_utf8_strict(bytes: &[u8]) -> Result<String, &'static str> {
    // Check for UTF-16BE BOM
    if bytes.starts_with(&[0xFE, 0xFF]) {
        // Validate UTF-16BE
        let decoded = decode_utf16be(&bytes[2..]);
        // Check if there were any replacement characters (indicating errors)
        if decoded.contains(std::char::REPLACEMENT_CHARACTER) {
            return Err("Invalid UTF-16BE sequence");
        }
        return Ok(decoded);
    }
    
    // Try UTF-8 first
    if let Ok(s) = std::str::from_utf8(bytes) {
        return Ok(s.to_string());
    }
    
    // Try PDFDocEncoding - this should always succeed
    // but we still check for undefined characters
    let result: String = bytes.iter().map(|&b| {
        if b < 128 {
            b as char
        } else {
            let table_index = (b as usize) - 128;
            let unicode_value = PDFDOCENCODING_TABLE[table_index];
            if unicode_value == 0xFFFF {
                std::char::REPLACEMENT_CHARACTER
            } else {
                std::char::from_u32(unicode_value as u32)
                    .unwrap_or(std::char::REPLACEMENT_CHARACTER)
            }
        }
    }).collect();
    
    // If we had to use replacement characters, it's an error
    if result.contains(std::char::REPLACEMENT_CHARACTER) {
        return Err("Invalid character in PDFDocEncoding");
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_string() {
        let input = "Hello World";
        let output = utf8_to_pdf_string_bytes(input);
        assert_eq!(output, input.as_bytes());
    }

    #[test]
    fn test_smart_quotes() {
        // Left double quotation mark (U+201C) should map to PDFDocEncoding byte 141
        let input = "\u{201C}Hello\u{201D}";
        let output = utf8_to_pdf_string_bytes(input);
        // U+201C maps to byte 141, U+201D maps to byte 142
        assert_eq!(output, vec![141, b'H', b'e', b'l', b'l', b'o', 142]);
    }

    #[test]
    fn test_emoji_uses_utf16be() {
        // Emoji cannot be represented in PDFDocEncoding, should use UTF-16BE
        let input = "Hello \u{1F600}";
        let output = utf8_to_pdf_string_bytes(input);
        
        // Should start with UTF-16BE BOM
        assert_eq!(&output[0..2], &[0xFE, 0xFF]);
        
        // Should contain UTF-16BE encoded text
        assert!(output.len() > 2);
    }

    #[test]
    fn test_latin1_characters() {
        // Test characters from the Latin-1 Supplement range
        let input = "caf\u{00E9}";
        let output = utf8_to_pdf_string_bytes(input);
        // 'c', 'a', 'f', 'é' where 'é' (U+00E9) maps to byte 233 in PDFDocEncoding
        assert_eq!(output, vec![b'c', b'a', b'f', 233]);
    }

    #[test]
    fn test_em_dash() {
        // EM DASH (U+2014) should map to PDFDocEncoding byte 132
        let input = "test\u{2014}dash";
        let output = utf8_to_pdf_string_bytes(input);
        assert_eq!(output, vec![b't', b'e', b's', b't', 132, b'd', b'a', b's', b'h']);
    }

    #[test]
    fn test_en_dash() {
        // EN DASH (U+2013) should map to PDFDocEncoding byte 133
        let input = "test\u{2013}dash";
        let output = utf8_to_pdf_string_bytes(input);
        assert_eq!(output, vec![b't', b'e', b's', b't', 133, b'd', b'a', b's', b'h']);
    }

    #[test]
    fn test_single_quotes() {
        // Left single quotation mark (U+2018) -> byte 143
        // Right single quotation mark (U+2019) -> byte 144
        let input = "\u{2018}test\u{2019}";
        let output = utf8_to_pdf_string_bytes(input);
        assert_eq!(output, vec![143, b't', b'e', b's', b't', 144]);
    }

    #[test]
    fn test_trademark() {
        // TRADE MARK SIGN (U+2122) should map to PDFDocEncoding byte 146
        let input = "Test\u{2122}";
        let output = utf8_to_pdf_string_bytes(input);
        assert_eq!(output, vec![b'T', b'e', b's', b't', 146]);
    }

    #[test]
    fn test_copyright() {
        // COPYRIGHT SIGN (U+00A9) should map to PDFDocEncoding byte 169
        let input = "\u{00A9}2024";
        let output = utf8_to_pdf_string_bytes(input);
        assert_eq!(output, vec![169, b'2', b'0', b'2', b'4']);
    }

    #[test]
    fn test_mixed_content_uses_utf16be() {
        // Mix of PDFDocEncoding characters and emoji should use UTF-16BE
        let input = "Hello\u{2122} \u{1F31F}";
        let output = utf8_to_pdf_string_bytes(input);
        
        // Should start with UTF-16BE BOM
        assert_eq!(&output[0..2], &[0xFE, 0xFF]);
    }

    #[test]
    fn test_decode_ascii() {
        let bytes = b"Hello World";
        let result = pdf_string_bytes_to_utf8(bytes);
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_decode_smart_quotes() {
        // Encoded with PDFDocEncoding: byte 141 = U+201C, byte 142 = U+201D
        let bytes = vec![141, b'H', b'e', b'l', b'l', b'o', 142];
        let result = pdf_string_bytes_to_utf8(&bytes);
        assert_eq!(result, "\u{201C}Hello\u{201D}");
    }

    #[test]
    fn test_decode_accented_characters() {
        // 'é' (U+00E9) maps to byte 233 in PDFDocEncoding
        let bytes = vec![b'c', b'a', b'f', 233];
        let result = pdf_string_bytes_to_utf8(&bytes);
        assert_eq!(result, "caf\u{00E9}");
    }

    #[test]
    fn test_decode_utf16be() {
        // UTF-16BE with BOM for "Hello"
        let bytes = vec![
            0xFE, 0xFF,  // BOM
            0x00, b'H',
            0x00, b'e',
            0x00, b'l',
            0x00, b'l',
            0x00, b'o',
        ];
        let result = pdf_string_bytes_to_utf8(&bytes);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_roundtrip_pdfdocencoding() {
        let inputs = vec![
            "Hello World",
            "\u{201C}Hello\u{201D}",
            "caf\u{00E9}",
            "test\u{2014}dash",
            "\u{00A9}2024",
        ];
        
        for input in inputs {
            let encoded = utf8_to_pdf_string_bytes(input);
            let decoded = pdf_string_bytes_to_utf8(&encoded);
            assert_eq!(decoded, input, "Roundtrip failed for '{}'", input);
        }
    }

    #[test]
    fn test_roundtrip_utf16be() {
        // Test with emoji that requires UTF-16BE
        let input = "Hello \u{1F600}";
        let encoded = utf8_to_pdf_string_bytes(input);
        let decoded = pdf_string_bytes_to_utf8(&encoded);
        assert_eq!(decoded, input);
    }
}
