fn main() {
    let target = "Hello";
    let base64 = Base64::new();
    let encoded = base64.encode(target);
    println!("Target : {:?}", target);
    println!("Encoded: {:?}", encoded);
    println!("Decoded: {:?}", base64.decode(&mut encoded.as_str()));
}

struct Base64 {
    tables: Vec<u8>,
}

impl Base64 {
    fn new() -> Self {
        let mut tables: Vec<u8> = Vec::new();
        (b'A'..=b'Z').for_each(|c| tables.push(c));
        (b'a'..=b'z').for_each(|c| tables.push(c));
        (b'0'..=b'9').for_each(|c| tables.push(c));
        tables.push('+' as u8);
        tables.push('/' as u8);
        Self { tables: tables }
    }

    fn encode(&self, str: &str) -> String {
        // string to hex to binary
        let bytes = str.as_bytes();
        let bin = bytes
            .iter()
            .map(|h| format!("{:X}", h))
            .collect::<Vec<String>>()
            .join("")
            .chars()
            .fold("".to_owned(), |s, c| {
                format!("{}{}", s, self.convert_hex_char_to_string(c))
            });

        // slice binary per 6 bits
        let mut tmp_str: String = String::new();
        let mut bin_6bit: Vec<String> = Vec::new();
        bin.chars().for_each(|c| {
            tmp_str = format!("{}{}", tmp_str, c);
            if tmp_str.len() == 6 {
                bin_6bit.push(tmp_str.to_owned());
                tmp_str = String::new();
            }
        });
        if tmp_str != String::new() {
            let pad = "0".repeat(6 - tmp_str.len());
            tmp_str = format!("{}{}", tmp_str, pad);
            bin_6bit.push(tmp_str.to_owned());
        }

        // encode bin
        let encoded = bin_6bit
            .iter()
            .map(|bin| self.tables[usize::from_str_radix(bin, 2).unwrap()] as char)
            .collect::<Vec<_>>();

        // format to base64
        let mut encoded_str = String::new();
        encoded.iter().for_each(|c| encoded_str.push(*c));
        let pad = "=".repeat(4 - encoded_str.len() % 4);
        encoded_str = format!("{}{}", encoded_str, pad);

        encoded_str
    }

    fn decode(&self, encoded_str: &mut &str) -> String {
        // SGVsbG8= => SGVsbG8
        let str = encoded_str.replace("=", "");

        // SGVsbG8 => 010010000110010101101100011011000110111100
        let bin = str
            .as_bytes()
            .iter()
            .map(|b| {
                let mut idx: usize = 0;
                for (i, v) in self.tables.iter().enumerate() {
                    if v == b {
                        idx = i;
                    }
                }
                idx
            })
            .map(|idx| format!("{:06b}", idx))
            .collect::<Vec<String>>()
            .join("");

        // 010010000110010101101100011011000110111100 =>
        // [0100, 1000, 0110, 0101, 0110, 1100, 0110, 1100, 0110, 1111] =>
        // ['4', '8', '6', '5', '6', 'C', '6', 'C', '6', 'F']
        let h = (0..(bin.len() - (bin.len() % 4)))
            .step_by(4)
            .map(|i| {
                let bytes = bin.as_bytes();
                let bin_4bit = format!(
                    "{}{}{}{}",
                    bytes[i] as char,
                    bytes[i + 1] as char,
                    bytes[i + 2] as char,
                    bytes[i + 3] as char
                );
                self.convert_string_to_hex_char(bin_4bit.as_str())
            })
            .collect::<Vec<char>>();

        // ['4', '8', '6', '5', '6', 'C', '6', 'C', '6', 'F'] =>
        // 48,65,6C,6C,6F => Hello
        (0..(h.len() - 1))
            .step_by(2)
            .map(|i| {
                let s = u8::from_str_radix(format!("{}{}", h[i], h[i + 1]).as_str(), 16).unwrap();
                (s as char).to_string()
            })
            .collect::<Vec<_>>()
            .join("")
    }

    fn convert_hex_char_to_string(&self, c: char) -> String {
        match c {
            '0' => "0000".to_string(),
            '1' => "0001".to_string(),
            '2' => "0010".to_string(),
            '3' => "0011".to_string(),
            '4' => "0100".to_string(),
            '5' => "0101".to_string(),
            '6' => "0110".to_string(),
            '7' => "0111".to_string(),
            '8' => "1000".to_string(),
            '9' => "1001".to_string(),
            'A' => "1010".to_string(),
            'B' => "1011".to_string(),
            'C' => "1100".to_string(),
            'D' => "1101".to_string(),
            'E' => "1110".to_string(),
            'F' => "1111".to_string(),
            _ => "".to_string(),
        }
    }

    fn convert_string_to_hex_char(&self, c: &str) -> char {
        match c {
            "0000" => '0',
            "0001" => '1',
            "0010" => '2',
            "0011" => '3',
            "0100" => '4',
            "0101" => '5',
            "0110" => '6',
            "0111" => '7',
            "1000" => '8',
            "1001" => '9',
            "1010" => 'A',
            "1011" => 'B',
            "1100" => 'C',
            "1101" => 'D',
            "1110" => 'E',
            "1111" => 'F',
            _ => '.',
        }
    }
}
