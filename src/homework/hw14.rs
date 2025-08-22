fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        let mut result = Vec::new();
        result.push(String::new());
        return result;
    }

    let mut result = Vec::new();
    let size = 1 << n;

    for i in 0..size {
        let mut s = String::new();

        for bit_pos in (0..n).rev() {
            if (i & (1 << bit_pos)) != 0 {
                s.push('1');
            } else {
                s.push('0');
            }
        }

        result.push(s);
    }

    result
}

#[test]
fn test() {
   let test_data =
       [
           (0, vec!("")),
           (1, vec!("0", "1")),
           (2, vec!("00", "01", "10", "11")),
           (3, vec!("000", "001", "010", "011", 
                    "100", "101", "110", "111")),
           (4, vec!("0000", "0001", "0010", "0011", 
                    "0100", "0101", "0110", "0111", 
                    "1000", "1001", "1010", "1011",
                    "1100", "1101", "1110", "1111")),
       ];


   test_data
       .iter()
       .for_each(|(n, out)| 
           assert_eq!(gray(*n), *out)
       );
}
