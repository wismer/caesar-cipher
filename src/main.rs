const ALPHA: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

enum Direction {
    Left,
    Right,
    Neither
}

struct Cipher {
    shift_num: usize,
    shift_dir: Direction
}

impl Cipher {
    fn process(&self, c: char, buffer: &mut String, shift: i32) {
        if c.is_alphabetic() {
            let corpus_loc = ALPHA.iter().position(|&a| a == c );
            if corpus_loc.is_some() {
                let actual = corpus_loc.unwrap() as i32;
                let mut crypted_pos: usize;
                if shift < 0 {
                    if actual + shift >= 0 {
                        crypted_pos = ((actual + shift) as usize);
                    } else {
                        crypted_pos = (actual + shift + 25) as usize;
                    }
                } else {
                    if actual + shift >= 25 {
                        crypted_pos = (actual + shift - 25) as usize;
                    } else {
                        crypted_pos = (actual + shift) as usize;
                    }
                }
                let crypted = ALPHA[crypted_pos];
                buffer.push(crypted);
            }
        } else {
            buffer.push(c);
        }
    }

    fn encode(&self, message: &str) -> String {
        let formatted_message = message.to_lowercase();
        let shift: i32 = match self.shift_dir {
            Direction::Right => self.shift_num as i32,
            Direction::Left => -(self.shift_num as i32),
            Direction::Neither => 0i32,
        };

        let mut crypted_message = String::new();

        for c in formatted_message.chars() {
            &self.process(c, &mut crypted_message, shift);
        }

        crypted_message
    }

    fn decode(&self, encrypted_msg: &str) -> String {
        let shift = match self.shift_dir {
            Direction::Right => -(self.shift_num as i32),
            Direction::Left => self.shift_num as i32,
            Direction::Neither => 0i32
        };

        let mut decoded_message = String::new();

        for c in encrypted_msg.chars() {
            &self.process(c, &mut decoded_message, shift);
        }

        decoded_message
    }
}

fn main() {
    let cipher = Cipher {
        shift_num: 2,
        shift_dir: Direction::Right
    };

    let encrypted = cipher.encode("that thing. did you get that thing i sent you.");

    let reverse_cipher = Cipher {
        shift_num: 4,
        shift_dir: Direction::Left
    };

    let encrypted_reverse = reverse_cipher.encode("System check identified 1 issue (0 silenced).
        September 16, 2015 - 21:27:47
        Django version 1.8.4, using settings 'doc.settings.local'
        Starting development server at http://127.0.0.1:8000/
        Quit the server with CONTROL-C.");

    {
        let decrypted_reverse = reverse_cipher.decode(&encrypted_reverse);
        println!("{}", decrypted_reverse);

        let decrypted = cipher.decode(&encrypted);
        println!("{}", decrypted);
    }
}
