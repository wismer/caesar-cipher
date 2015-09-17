const ALPHA: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'y', 'x', 'z'];

enum Direction {
    Left,
    Right,
    Neither
}

struct Cipher {
    shift_num: usize,
    shift_dir: Direction,
    corpus: String
}

impl Cipher {
    fn show_positions(self) {
        let shift: i32 = match self.shift_dir {
            Direction::Right => self.shift_num as i32,
            Direction::Left => -(self.shift_num as i32),
            Direction::Neither => 0i32,
        };

        let mut crypted_message = String::new();
        for c in self.corpus.chars() {
            if c.is_alphabetic() {
                let corpus_loc = ALPHA.iter().position(|&a| a == c );
                if corpus_loc.is_some() {
                    let actual = corpus_loc.unwrap() as i32;
                    let mut crypted_pos: usize;
                    if shift < 0 {
                        if actual + shift > 0 {
                            crypted_pos = ((actual + shift) as usize);
                        } else {
                            crypted_pos = (actual + shift + 25) as usize;
                        }
                    } else {
                        if actual + shift > 25 {
                            crypted_pos = (actual + shift - 25) as usize;
                        } else {
                            crypted_pos = (actual + shift) as usize;
                        }
                    }
                    let crypted = ALPHA[crypted_pos];
                    crypted_message.push(crypted);
                }
            } else {
                crypted_message.push(c);
            }
        }
        println!("{}", crypted_message);
    }
}

fn main() {
    let cipher = Cipher {
        shift_num: 2,
        shift_dir: Direction::Right,
        corpus: "that thing. did you get that thing i sent you.".to_string()
    };

    cipher.show_positions();

    let reverse_cipher = Cipher {
        shift_num: 4,
        shift_dir: Direction::Left,
        corpus: "System check identified 1 issue (0 silenced).
            September 16, 2015 - 21:27:47
            Django version 1.8.4, using settings 'doc.settings.local'
            Starting development server at http://127.0.0.1:8000/
            Quit the server with CONTROL-C.".to_string()
    };

    reverse_cipher.show_positions()
}
