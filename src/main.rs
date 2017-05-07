use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::prelude::*;
use std::env;


fn main() {
    // Encoding files is working
    let args: Vec<_> = env::args().collect();
    let path = Path::new(args[1].as_str());
    let mut file =
        match File::open(&path) {
            Err(why) => panic!("Couldn't open {:?}: {}", path, why),
            Ok(file) => file,
        };
    let mut example_text = String::from("");
    match file.read_to_string(&mut example_text) {
        Err(why) => panic!("Couldn't read: {}", why),
        Ok(_) => (),
    }
    let text = example_text.as_str();
    let mut table = Table::init(text);
    let encoded_text = table.encode_string(text);
    let mut file = 
        match File::create(format!("{}.bin", args[1])) {
            Err(why) => panic!("couldn't create file: {}", why),
            Ok(file) => file,
        };
    match file.write_all(table.as_string().as_bytes()) {
        Err(why) => panic!("couldn't write keys to file: {}", why),
        Ok(_) => println!("successfully wrote keys"),
    }
    match file.write_all(encoded_text.as_slice()) {
        Err(why) => panic!("couldn't write to file: {}", why),
        Ok(_) => println!("successfully encoded file"),
    }
}


type Frequency = Vec<(char, usize)>;


#[derive(Debug)]
struct Table(Vec<char>);

impl Table {
    fn init(text: &str) -> Self {
        Table(Self::char_counter(text))
    }

    fn char_counter(text: &str) -> Vec<char> {
        let mut keys: Frequency = vec!();
        for letter in text.chars() {
            match keys.iter().position(|&x| x.0 == letter) {
                Some(index) => {
                    keys[index] = (keys[index].0, keys[index].1 + 1);
                },
                None => {
                    keys.push((letter, 1));
                }
            }
        }
        keys.sort_by(|a, b| b.1.cmp(&a.1));
        keys.into_iter().map(|x| x.0).collect()
    }

    fn encode(&self, val: char) -> u8 {
        for (i, item) in self.0.iter().enumerate() {
            if &val == item {
                return i as u8;
            }
        }
        panic!("Character not in Keys");
    }

    fn decode(&self, val: u8) -> char {
        // Should decode real file
        for (i, item) in self.0.iter().enumerate() {
            if val == i as u8 {
                return *item;
            }
        }
        panic!("Character not found in Keys");
    }

    fn encode_string(&self, text: &str) -> Vec<u8> {
        let mut encode_text = vec!();
        for letter in text.chars() {
            encode_text.push(self.encode(letter));
        }
        encode_text
    }

    fn decode_vec_u8(&self, text: &Vec<u8>) -> Vec<char> {
        // FIXME
        let mut decode_text = vec!();
        for item in text {
            decode_text.push(self.decode(*item));
        }
        decode_text
    }

    fn decode_string(&self, text: &Vec<u8>) -> String {
        // FIXME
        let decode_vec = self.decode_vec_u8(text);
        let mut decode_str = String::from("");
        for c in decode_vec {
            decode_str.push(c);
        }
        decode_str
    }

    fn as_string(&mut self) -> String {
        let mut res = String::from("");
        for val in self.0.iter() {
            res.push(*val);
        }
        res.push_str("(END)");
        res
    }
}

