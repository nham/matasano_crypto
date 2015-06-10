use rustc_serialize::base64::{self, ToBase64};
use rustc_serialize::hex::FromHex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::{Index, IndexMut};


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
pub fn single_byte_xor_cipher(s: &str) -> (String, f64) {
    let x = hex_to_bytes(s);

    let mut best_score = ::std::f64::MIN;
    let mut best = String::new();

    for key in 0x20..0x7f {
        // println!("\n\nKEY: {}", key as char);
        let mut v = Vec::new();
        for _ in 0..x.len() { v.push(key); }

        let xor = fixed_xor(&x, &v);
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

    (best, best_score)
}

pub fn challenge3() -> String {
    let output = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    single_byte_xor_cipher(output).0
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


// Challenge 4
pub fn challenge4() -> ::std::io::Result<String> {
    let file_name = "data/challenge4.txt";
    let mut f = try!(File::open(file_name));
    let mut s = String::new();
    let mut best_score = ::std::f64::MIN;
    let mut best = String::new();
    let mut best_i = 0;
    try!(f.read_to_string(&mut s));
    for (i, line) in s.lines().enumerate() {
        let (orig, score) = single_byte_xor_cipher(line);
        if score > best_score {
            best_score = score;
            best = orig;
            best_i = i;
        }
    }
    println!("best one = {}", best_i);
    Ok(best)
    //single_byte_xor_cipher(output).0
}


// Challenge 5
pub fn repeating_xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(input.len() >= key.len());
    let repeats = input.len() / key.len();
    let mut v = Vec::new();
    for _ in 0..repeats {
        for &b in key.iter() {
            v.push(b);
        }
    }
    for i in 0..(input.len() % key.len()) {
        v.push(key[i]);
    }
    println!("right before fixed_xor call");
    fixed_xor(input, &v)
}

// Challenge 6
fn hamming_distance(buf1: &[u8], buf2: &[u8]) -> usize {
    assert_eq!(buf1.len(), buf2.len());
    let mut count = 0;
    for (a, b) in buf1.iter().zip(buf2.iter()) {
        count += count_ones(a ^ b);
    }
    count
}

fn count_ones(mut x: u8) -> usize {
    let mut count = 0;
    if x & 0x01 > 0 { count += 1; }
    for _ in 0..7 {
        x = x >> 1;
        if x & 0x01 > 0 { count += 1; }
    }
    count
}


// util
fn hex_to_bytes(hex: &str) -> Vec<u8> {
    match hex.from_hex() {
        Ok(v) => v,
        Err(e) => panic!("Error converting from hex string: {}", e),
    }
}


#[cfg(test)]
mod tests {
    use super::{hex_to_base64, fixed_xor, hex_to_bytes, challenge3};
    use super::{repeating_xor, count_ones, hamming_distance};

    // Test challenge 1
    #[test]
    fn test_hex_to_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(base64, hex_to_base64(hex));
    }

    // Test challenge 2
    #[test]
    fn test_fixed_xor() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        let expected_result = "746865206b696420646f6e277420706c6179";
        let xor = fixed_xor(&hex_to_bytes(a), &hex_to_bytes(b));

        assert_eq!(&hex_to_bytes(expected_result), &xor);
    }

    // Test challenge 3
    #[test]
    fn test_challenge3() {
        assert_eq!(challenge3(),
                   String::from("Cooking MC's like a pound of bacon"));
    }

    // Test challenge 5
    #[test]
    fn test_repeating_xor() {
        let s = b"Burning 'em, if you ain't quick and nimble\n\
                  I go crazy when I hear a cymbal";
        let key = b"ICE";
        let hex = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d\
                   63343c2a26226324272765272a282b2f20430a652e2c652a31\
                   24333a653e2b2027630c692b20283165286326302e27282f";
        let xor = repeating_xor(s, key);

        assert_eq!(&xor, &hex_to_bytes(hex));
    }

    #[test]
    fn test_count_ones() {
        // actual challenge wants this to be the hamming distance
        // but I'm gonna see if I can do it with levenshtein instead
        assert_eq!(1, count_ones(0x01));
        assert_eq!(1, count_ones(0x02));
        assert_eq!(1, count_ones(0x04));
        assert_eq!(1, count_ones(0x08));
        assert_eq!(1, count_ones(0x10));
        assert_eq!(1, count_ones(0x20));
        assert_eq!(1, count_ones(0x40));
        assert_eq!(1, count_ones(0x80));
        assert_eq!(2, count_ones(0x03));
        assert_eq!(2, count_ones(0x81));
        assert_eq!(2, count_ones(0x18));
        assert_eq!(3, count_ones(0x07));
        assert_eq!(3, count_ones(0x34));
        assert_eq!(8, count_ones(0xff));
    }

    #[test]
    fn test_hamming_distance() {
        let s1 = b"this is a test";
        let s2 = b"wokka wokka!!!";
        assert_eq!(hamming_distance(s1, s2), 37);
    }

}
