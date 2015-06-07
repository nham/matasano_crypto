use rustc_serialize::base64::{self, ToBase64};
use rustc_serialize::hex::FromHex;
use std::collections::HashMap;


// Challenge 1
fn hex_to_base64(hex: &str) -> String {
    hex.from_hex().unwrap().as_slice().to_base64(base64::STANDARD)
}

// Challenge 2
fn fixed_xor(buf1: &[u8], buf2: &[u8]) -> Vec<u8> {
    assert_eq!(buf1.len(), buf2.len());
    let mut v = Vec::new();
    for (a, b) in buf1.iter().zip(buf2.iter()) {
        v.push(a ^ b);
    }
    v
}

fn partial_ascii_display(v: &Vec<u8>) {
    for &e in v.iter() {
        if e >= 0x20 && e < 0x7f {
            print!("{}", e as char);
        } else {
            print!("_");
        }
    }
    println!("");
}

// Challenge 3
pub fn single_byte_xor_cipher() -> String {
    let x_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let x = hex_to_bytes(x_str);

    let mut best_score = ::std::f64::MIN;
    let mut best = String::new();

    for key in 0x20..0x7f {
        // println!("\n\nKEY: {}", key as char);
        let mut v = Vec::new();
        for _ in 0..x.len() { v.push(key); }

        let xor = fixed_xor(x.as_slice(), v.as_slice());
        // partial_ascii_display(&xor);

        match String::from_utf8(xor) {
            Err(_) => continue,
            Ok(candidate) => {
                let candidate_score = score(&candidate[..]);
                // println!("score:  {}", candidate_score);

                if candidate_score > best_score {
                    best_score = candidate_score;
                    best = candidate;
                }
            },
        }
        // println!("");

    }

    //println!("best_score = {}", best_score);
    best
}

fn score(x: &str) -> f64 {
    // from http://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_letters_in_the_English_language
    let chars = ['e', 't', 'a', 'o', 'i', 'n',
                 's', 'h', 'r', 'd', 'l', 'c',
                 'u', 'm', 'w', 'f', 'g', 'y',
                 'p', 'b', 'v', 'k', 'j', 'x',
                 'q', 'z'];
    // round towards even
    let freqs_array = [0.1270, 0.0906, 0.0817, 0.0751, 0.0697, 0.0675,
                       0.0633, 0.0609, 0.0599, 0.0425, 0.0402, 0.0278,
                       0.0276, 0.0241, 0.0236, 0.0223, 0.0202, 0.0197,
                       0.0193, 0.0149, 0.0098, 0.0077, 0.0015, 0.0015,
                       0.0100, 0.007];
    let mut freqs = HashMap::new();
    for i in 0..chars.len() {
        freqs.insert(chars[i], freqs_array[i]);
    }

    let mut num_occurrences = HashMap::new();
    let mut num_alphas = 0;
    let mut num_whitespace = 0;
    let mut num_other = 0;

    // Phase 1: occurrence counting
    for mut c in x.chars() {
        c = c.to_lowercase().next().unwrap();
        if c.is_alphabetic() {
            num_alphas += 1;
            let num = num_occurrences.entry(c).or_insert(0);
            *num += 1;
        } else if c.is_whitespace() {
            num_whitespace += 1;
        } else {
            num_other += 1;
        }
    }

    // Phase 2: error calculation
    let mut total_error = 0.;

    // error from alphabetic
    for ch in freqs.keys() {
        let expected_occurrences = *freqs.get(ch).unwrap() * (num_alphas as f64);
        let actual_occurrences = match num_occurrences.get(ch) {
            Some(&count) => count as f64,
            None => 0.,
        };
        let error: f64 = (expected_occurrences - actual_occurrences).powi(2);
        //println!("{} error = {}", ch, error);
        total_error += 0.5 * error;
    }

    //println!("alpha error: {}", total_error);

    // error from whitespace characters
    let avg_word_length = 5.;
    // find the biggest k such that 5k + k - 1 <= x.len()
    // (wolfram alpha says the average length of an english word
    // is 5.1 letters. I'm rounding down to 5 here)
    let k = (x.len() + 1)/6; // rounds down automatically

    // check if k + 1 is closer
    // the real condition we want to check is if
    // |5(k+1) + k - x.len()| < |x.len() - (5k + k - 1)|
    //
    // but by assumption k is such that 5k + k - 1 < x.len()
    // so we can remove absolute value on the right
    // also we must have 5(k+1) + k > x.len(), because otherwise
    // k would not be the greatest integer such that 5k + k - 1 <= x.len()
    // so the absolute value on the left can be removed as well to obtain
    //
    // 5k + k + 5 - x.len() < x.len() - 5k - k + 1
    //
    // which simplifies to:
    let num_words = if 5*k + k + 2 < x.len() {
        k + 1
    } else {
        k
    };

    let expected_occurrences = (num_words - 1) as f64;
    let actual_occurrences = num_whitespace as f64;
    let error: f64 = (expected_occurrences - actual_occurrences).powi(2);
    //println!("ws error = {}", error);
    total_error += 2. * error;


    // error from non-alphabetic, non-whitespace characters
    // arbitrary guess on how many others there should be: 1 for every 20 characters.
    let expected_occurrences = (x.len() as f64) / 20.;
    //println!("expected/actual others: {}/{}", expected_occurrences, num_other as f64);
    let error = (expected_occurrences - (num_other as f64)).powi(2);
    //println!("other error = {}", error);
    total_error += 2. * error;

    -total_error
}


// util
fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex.from_hex().unwrap()
}


#[cfg(test)]
mod tests {
    use super::{hex_to_base64, fixed_xor, hex_to_bytes, single_byte_xor_cipher};

    #[test]
    fn test_hex_to_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(base64, hex_to_base64(hex));
    }

    #[test]
    fn test_fixed_xor() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        let expected_result = "746865206b696420646f6e277420706c6179";
        let xor = fixed_xor(hex_to_bytes(a).as_slice(),
                            hex_to_bytes(b).as_slice());

        assert_eq!(hex_to_bytes(expected_result), xor.as_slice());
    }

    #[test]
    fn test_single_byte_xor_cipher() {
        assert_eq!(single_byte_xor_cipher(),
                   String::from("Cooking MC's like a pound of bacon"));

    }
}
