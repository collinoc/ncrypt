mod hash;

#[cfg(test)]
mod tests {
    use super::hash::sha1::SHA1;
    use super::hash::md5::MD5;

    #[test]
    fn sha1_empty() {
        let hash: [u32; 5] = SHA1::hash(b"");
        assert_eq!(&hash_to_string32(&hash), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    }

    #[test]
    fn sha1_dog() {
        let hash: [u32; 5] = SHA1::hash(b"The quick brown fox jumps over the lazy dog");

        assert_eq!(&hash_to_string32(&hash), "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
    }

    #[test]
    fn sha1_cog() {
        let hash: [u32; 5] = SHA1::hash(b"The quick brown fox jumps over the lazy cog");
        
        assert_eq!(&hash_to_string32(&hash), "de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3");
    }
    
    #[test]
    fn sha1_longer() {
        let hash: [u32; 5] = SHA1::hash(b"Using my experience with UE3, I'll make some maps and try to make a game within the tools provided with S&Box and Source2. I have already dipped my feet into Source2 with HL:A and I would like to do more with the engine than just limited tools for a VR title. I'll also see if I can provide early feedback for other projects other developers are working on.");
        assert_eq!(&hash_to_string32(&hash), "5f4a099d8a89b2e2c8cfd74d9170fbeb798d3201");
    }
    
    #[test] 
    fn sha1_close_to_64() {
        let hash: [u32; 5] = SHA1::hash(b"Using my experience with UE3, I'll make some maps and try to");
        assert_eq!(&hash_to_string32(&hash), "d5bdc836b0a991f089dc085221020f2500fe560d");
    }
    
    #[test] 
    fn sha1_exactly_64() {
        let hash: [u32; 5] = SHA1::hash(b"Using my experience with UE3, I'll make some maps and try to mak");
        assert_eq!(&hash_to_string32(&hash), "ec8f563348018ee2b9b18df5d377ad8dc5d7ecc2");
    }

    #[test] 
    fn sha1_exactly_63() {
        let hash: [u32; 5] = SHA1::hash(b"Using my experience with UE3, I'll make some maps and try to ma");
        assert_eq!(&hash_to_string32(&hash), "5101119a893ed48d7da836e7733e6ff5178a8f91");
    }

    #[test] 
    fn sha1_exactly_65() {
        let hash: [u32; 5] = SHA1::hash(b"Using my experience with UE3, I'll make some maps and try to make");
        assert_eq!(&hash_to_string32(&hash), "a9fa1835bec1092a4022f7d35f22750c6a44f124");
    }


    #[test]
    fn md5_empty() {
        let hash: [u32; 4] = MD5::hash(b"");
        assert_eq!(&hash_to_string32(&hash), "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn md5_dog() {
        let hash: [u32; 4] = MD5::hash(b"The quick brown fox jumps over the lazy dog");
        assert_eq!(&hash_to_string32(&hash), "9e107d9d372bb6826bd81d3542a419d6");
    
    }
    
    #[test]
    fn md5_cog() {
        let hash: [u32; 4] = MD5::hash(b"The quick brown fox jumps over the lazy cog");
        assert_eq!(&hash_to_string32(&hash), "1055d3e698d289f2af8663725127bd4b");
    }

    #[test]
    fn md5_longer() {
        let hash: [u32; 4] = MD5::hash(b"Using my experience with UE3, I'll make some maps and try to make a game within the tools provided with S&Box and Source2. I have already dipped my feet into Source2 with HL:A and I would like to do more with the engine than just limited tools for a VR title. I'll also see if I can provide early feedback for other projects other developers are working on.");
        assert_eq!(&hash_to_string32(&hash), "3e353547aa3fe9a2cda98e5a91e20a87");
    }

    #[test] 
    fn md5_close_to_64() {
        let hash: [u32; 4] = MD5::hash(b"Using my experience with UE3, I'll make some maps and try to");
        assert_eq!(&hash_to_string32(&hash), "34d0c92658f3a49d40d6e2dd3d7d902a");
    }
    
    #[test] 
    fn md5_exactly_64() {
        let hash: [u32; 4] = MD5::hash(b"Using my experience with UE3, I'll make some maps and try to mak");
        assert_eq!(&hash_to_string32(&hash), "e2d783f6f918a5592221b26f41be4deb");
    }

    #[test] 
    fn md5_exactly_63() {
        let hash: [u32; 4] = MD5::hash(b"Using my experience with UE3, I'll make some maps and try to ma");
        assert_eq!(&hash_to_string32(&hash), "bad3603f517f21f885af8508b65260c3");
    }

    #[test] 
    fn md5_exactly_65() {
        let hash: [u32; 4] = MD5::hash(b"Using my experience with UE3, I'll make some maps and try to make");
        assert_eq!(&hash_to_string32(&hash), "55a46527af698b575e9e833ff342ebe8");
    }

    fn hash_to_string32(hash: &[u32]) -> String {
        hash.iter().map(|h| format!("{h:08x}")).into_iter().collect()
    }
}