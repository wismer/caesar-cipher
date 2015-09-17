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
        let shift = match self.shift_dir {
            Direction::Right => self.shift_num,
            Direction::Left => self.shift_num + 25,
            Direction::Neither => 0usize,
        };
        let mut crypted_message = String::new();
        for c in self.corpus.chars() {
            if c.is_alphabetic() {
                let corpus_loc = ALPHA.iter().position(|&a| a == c );
                if corpus_loc.is_some() {
                    let actual = corpus_loc.unwrap();
                    let mut crypted_pos = actual + shift;
                    if crypted_pos > 25 {
                        crypted_pos = crypted_pos - 25;
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
    let mut cipher = Cipher {
        shift_num: 2,
        shift_dir: Direction::Right,
        corpus: "that thing. did you get that thing i sent you.".to_string()
    };

    let mut reverse_cipher = Cipher {
        shift_num: 4,
        shift_dir: Direction::Left,
        corpus: "System check identified 1 issue (0 silenced).
            September 16, 2015 - 21:27:47
            Django version 1.8.4, using settings 'doc.settings.local'
            Starting development server at http://127.0.0.1:8000/
            Quit the server with CONTROL-C.".to_string()
    };

    cipher.show_positions();
    reverse_cipher.show_positions()
}
