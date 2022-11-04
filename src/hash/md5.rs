#![allow(dead_code)]

pub struct MD5;

impl MD5 {
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
        5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
        0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
        0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
        0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
        0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
        0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
        0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
        0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];

    pub fn hash(bytes: &[u8]) -> [u32; 4] {
        let mut padded_msg: Vec<u8> = Vec::from(bytes);

        padded_msg.push(0x80);

        let pad_size = (64 - (padded_msg.len() + 8)) % 64;

        padded_msg.append(&mut vec![0; pad_size]);

        let len_bits = bytes.len() * 8;
        
        padded_msg.append(&mut len_bits.to_le_bytes().into());

        let mut a0: u32 = 0x67452301;
        let mut b0: u32 = 0xefcdab89;
        let mut c0: u32 = 0x98badcfe;
        let mut d0: u32 = 0x10325476;

        let chunks: Vec<&[u8]> = padded_msg.chunks(64).collect();

        for chunk in chunks {
            let mut a = a0;
            let mut b = b0;
            let mut c = c0;
            let mut d = d0;

            let mut words: [u32; 16] = [0; 16];

            for i in 0..16 {
                words[i] = u32::from_le_bytes([chunk[i * 4], chunk[i* 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]).to_le();
            }

            for i in 0..64 {
                let mut f = match i {
                    _ if i < 16 => (b & c) | ((!b) & d),
                    _ if i < 32 => (d & b) | ((!d) & c),
                    _ if i < 48 =>  b ^ c ^ d,
                    _ =>            c ^ (b | (!d)),
                };

                let g = match i {
                    _ if i < 16 => i,
                    _ if i < 32 => ((i * 5) + 1) % 16,
                    _ if i < 48 => ((i * 3) + 5) % 16,
                    _ =>           (i * 7) % 16,
                };

                f = f.wrapping_add(a)
                    .wrapping_add(MD5::K[i])
                    .wrapping_add(words[g]);
                a = d;
                d = c;
                c = b;
                b = b.wrapping_add(shlw(f, MD5::S[i] as usize));
            }

            a0 = a0.wrapping_add(a);
            b0 = b0.wrapping_add(b);
            c0 = c0.wrapping_add(c);
            d0 = d0.wrapping_add(d);
        }

        [a0.to_be(), b0.to_be(), c0.to_be(), d0.to_be()]
    }
}

#[inline]
fn shlw(num: u32, count: usize) -> u32 {
    (num << count) | (num >> (32 - count))
}