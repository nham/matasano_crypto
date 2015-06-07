use rustc_serialize::base64::{self, ToBase64};
use rustc_serialize::hex::FromHex;

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

// util
fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex.from_hex().unwrap()
}


#[cfg(test)]
mod tests {
    use super::{hex_to_base64, fixed_xor, hex_to_bytes};

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
}
