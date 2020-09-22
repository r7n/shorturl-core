use fasthash::murmur3;
fn get_hash(url: String) -> u32{
    murmur3::hash32(url.as_bytes())
}
fn get_hash_with_seed(url: String, seed: u32) -> u32 {
    murmur3::hash32_with_seed(url.as_bytes(), seed)
}
fn u32_to_62(hash: u32) -> String {
    let dict = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut n = hash;
    let mut chars:Vec<char> = vec![];
    while n > 0 {
        let i = (n % 62) as usize;
        let c = dict.chars().nth(i).unwrap();
        chars.push(c);
        n /= 62;
    }
    chars.reverse();
    chars.into_iter().collect::<String>()
}
pub fn short_url(url: String) -> String {
    let hash = get_hash(url);
    u32_to_62(hash)
}
pub fn short_url_with_seed(url: String, seed: u32) -> String {
    let hash = get_hash_with_seed(url, seed);
    u32_to_62(hash)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_hash(){
        assert_eq!(get_hash("r7n.cc".to_string()), 3567690683);
    }
    #[test]
    fn test_get_hash_with_seed(){
        assert_eq!(get_hash_with_seed("r7n.cc".to_string(), 0), get_hash("r7n.cc".to_string()));
        assert_eq!(get_hash_with_seed("r7n.cc".to_string(), 1), 3437183471);
    }
    #[test]
    fn test_u32_to_62() {
        assert_eq!(u32_to_62(3567690683), "3TrFk7");
        assert_eq!(u32_to_62(3437183471), "3KC4r5");
    }
    #[test]
    fn test_short_url() {
        assert_eq!(short_url("r7n.cc".to_string()), "3TrFk7");
    }
    #[test]
    fn test_short_url_with_seed() {
        assert_eq!(short_url_with_seed("r7n.cc".to_string(), 0), "3TrFk7");
        assert_eq!(short_url_with_seed("r7n.cc".to_string(), 1), "3KC4r5");
    }
}
