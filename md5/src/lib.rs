//! The MD5 hash function
//!
//! ## Example
//! ```
//! let digest = md5::compute(b"abc");
//! assert_eq!(format!("{:>032x}", digest), "900150983cd24fb0d6963f7d28e17f72");
//! ```

const PADDING: [u8; 64] = [
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

static C1: [u32; 16] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821
];

static C2: [u32; 16] = [
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a
];

static C3: [u32; 16] = [
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665
];

static C4: [u32; 16] = [
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
];

fn copy_8to32_le(dst: &mut[u32], src: &[u8]) {
    assert!(dst.len() * 4 == src.len());
    let mut dst_index = 0;
    for i in 0..src.len() {
        if i % 4 == 0 {
            dst_index = i / 4;
            dst[dst_index] = 0;
        }
        dst[dst_index] += (src[i] as u32) << ((i % 4) * 8);
    }
}

fn append_padding_bits<T: AsRef<[u8]>>(data: T) -> (usize, Vec<u8>) {
    let data = data.as_ref();
    let bytes = data.len();
    let bytes_mod64 = bytes % 64;
    let mut vec: Vec<u8> = Vec::new();
    vec.extend_from_slice(data);
    vec.extend_from_slice(&PADDING[..(if bytes_mod64 < 56 { 56 - bytes_mod64 } else { 56 + 64 - bytes_mod64 })]);
    (bytes, vec)
}

fn append_length(bits: usize, vec: &mut Vec<u8>) {
    let mut tmp = bits;
    for _ in 0..8 {
        vec.push((tmp & 0xff) as u8);
        tmp = tmp >> 8;
    }
}

fn process_message(buf: &mut [u32; 4], input: &[u8]) {
    fn f(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (!x & z)
    }

    fn g(x: u32, y: u32, z: u32) -> u32 {
        (x & z) | (y & !z)
    }

    fn h(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }

    fn i(x: u32, y: u32, z: u32) -> u32 {
        y ^ (x | !z)
    }

    fn ff(a: u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) -> u32 {
        a.wrapping_add(f(b, c, d)).wrapping_add(x).wrapping_add(ac).rotate_left(s).wrapping_add(b)
    }

    fn gg(a: u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) -> u32 {
        a.wrapping_add(g(b, c, d)).wrapping_add(x).wrapping_add(ac).rotate_left(s).wrapping_add(b)
    }

    fn hh(a: u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) -> u32 {
        a.wrapping_add(h(b, c, d)).wrapping_add(x).wrapping_add(ac).rotate_left(s).wrapping_add(b)
    }

    fn ii(a: u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) -> u32 {
        a.wrapping_add(i(b, c, d)).wrapping_add(x).wrapping_add(ac).rotate_left(s).wrapping_add(b)
    }

    let mut a = buf[0];
    let mut b = buf[1];
    let mut c = buf[2];
    let mut d = buf[3];

    let mut data = [0u32; 16];

    copy_8to32_le(&mut data, input);

    // Round 1
    for i in 0..4 {
        let k = i * 4;
        a = ff(a, b, c, d, data[k    ],  7, C1[k    ]);
        d = ff(d, a, b, c, data[k + 1], 12, C1[k + 1]);
        c = ff(c, d, a, b, data[k + 2], 17, C1[k + 2]);
        b = ff(b, c, d, a, data[k + 3], 22, C1[k + 3]);
    }

    // Round 2
    let mut t = 1;
    for i in 0..4 {
        let k = i * 4;
        a = gg(a, b, c, d, data[ t       & 0x0f],  5, C2[k    ]);
        d = gg(d, a, b, c, data[(t +  5) & 0x0f],  9, C2[k + 1]);
        c = gg(c, d, a, b, data[(t + 10) & 0x0f], 14, C2[k + 2]);
        b = gg(b, c, d, a, data[(t + 15) & 0x0f], 20, C2[k + 3]);
        t += 20;
    }

    // Round 3
    t = 5;
    for i in 0..4 {
        let k = i * 4;
        a = hh(a, b, c, d, data[ t      & 0x0f],  4, C3[k    ]);
        d = hh(d, a, b, c, data[(t + 3) & 0x0f], 11, C3[k + 1]);
        c = hh(c, d, a, b, data[(t + 6) & 0x0f], 16, C3[k + 2]);
        b = hh(b, c, d, a, data[(t + 9) & 0x0f], 23, C3[k + 3]);
        t += 12;
    }

    // Round 4
    t = 0;
    for i in 0..4 {
        let k = i * 4;
        a = ii(a, b, c, d, data[ t       & 0x0f],  6, C4[k    ]);
        d = ii(d, a, b, c, data[(t + 7)  & 0x0f], 10, C4[k + 1]);
        c = ii(c, d, a, b, data[(t + 14) & 0x0f], 15, C4[k + 2]);
        b = ii(b, c, d, a, data[(t + 21) & 0x0f], 21, C4[k + 3]);
        t += 28;
    }

    buf[0] = buf[0].wrapping_add(a);
    buf[1] = buf[1].wrapping_add(b);
    buf[2] = buf[2].wrapping_add(c);
    buf[3] = buf[3].wrapping_add(d);
}

fn get_digest(buf: &[u32; 4]) -> u128 {
    let mut digest: u128 = 0;
    for index in 0..4 {
        let mut tmp = buf[index] as u128;
        for byte_index in 0..4 {
            digest += (tmp & 0xff) << (((3 - index) * 4 + (3 - byte_index)) * 8);
            tmp = tmp >> 8;
        }
    }

    digest
}

/// Compute MD5 message digest
pub fn compute<T: AsRef<[u8]>>(data: T) -> u128 {
    // Step 1. Append Padding Bits
    let (bytes, mut vec) = append_padding_bits(data);

    // Step 2. Append Length
    append_length(bytes * 8, &mut vec);

    // Step 3. Initialize MD Buffer
    let mut buf: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];

    // Step 4. Process Message in 16-Word Blocks
    let mut i: usize = 0;
    while i < vec.len() {
        process_message(&mut buf, &vec[i..(i + 64)]);
        i += 64;
    }

    get_digest(&buf)
}

#[cfg(test)]
mod tests {
    #[test]
    fn copy_8to32_le() {
        let mut buf32 : [u32; 2] = [0x0; 2];
        let buf8 : [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        ::copy_8to32_le(&mut buf32, &buf8);
        assert_eq!(buf32, [0x67452301, 0xefcdab89]);
    }

    #[test]
    fn append_length() {
        let mut vec = Vec::new();
        ::append_length(0x01020304, &mut vec);
        assert_eq!(vec, vec![4, 3, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn get_digest() {
        let buf: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];
        assert_eq!(::get_digest(&buf), 0x123456789abcdeffedcba9876543210);
    }
}
