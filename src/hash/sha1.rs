#![allow(dead_code)]

pub struct SHA1;

impl SHA1 {
    pub fn hash(message: &[u8]) -> [u32; 5] {
        let mut h0: u32 = 0x67452301;
        let mut h1: u32 = 0xEFCDAB89;
        let mut h2: u32 = 0x98BADCFE;
        let mut h3: u32 = 0x10325476;
        let mut h4: u32 = 0xC3D2E1F0;

        let mut padded_message: Vec<u8> = Vec::from(message);

        padded_message.push(0x80);

        let pad_size = (64 - (padded_message.len() + 8)) % 64;

        padded_message.append(&mut vec![0; pad_size]);
        
        let len_bits = message.len() * 8;
        
        padded_message.append(&mut len_bits.to_be_bytes().into());

        let chunks: Vec<&[u8]> = padded_message.chunks(64).collect();

        for chunk in chunks {
            let mut words = [0u32; 80];
            
            for i in 0..16 {
                words[i] = u32::from_le_bytes([chunk[i * 4], chunk[i* 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]).to_be();
            }
            
            for i in 16..80 {
                words[i] = shlw(words[i-3] ^ words[i-8] ^ words[i-14] ^ words[i-16], 1);
            }

            let mut a = h0;
            let mut b = h1;
            let mut c = h2;
            let mut d = h3;
            let mut e = h4;

            for i in 0..80 { 
                let f: u32 = match i {
                    _ if i < 20 => (b & c) | ((! b) & d),
                    _ if i < 40 => b ^ c ^ d,
                    _ if i < 60 => (b & c) | (b & d) | (c & d),
                    _ =>           b ^ c ^ d,
                };

                let k: u32 = match i {
                    _ if i < 20 => 0x5A827999,
                    _ if i < 40 => 0x6ED9EBA1,
                    _ if i < 60 => 0x8F1BBCDC,
                    _ =>           0xCA62C1D6,
                };

                let temp: u32 = shlw(a, 5)
                    + f
                    + e
                    + k
                    + words[i];

                e = d;
                d = c;
                c = shlw(b, 30);
                b = a;
                a = temp;
            }

            h0 = h0.wrapping_add(a);
            h1 = h1.wrapping_add(b);
            h2 = h2.wrapping_add(c);
            h3 = h3.wrapping_add(d);
            h4 = h4.wrapping_add(e);
        }
        
        [h0, h1, h2, h3, h4]
    }
}

#[inline]
fn shlw(num: u32, count: usize) -> u32 {
    (num << count) | (num >> (32 - count))
}