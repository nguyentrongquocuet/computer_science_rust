pub fn int2bin(num: i32) -> String {
  let mut n = num;
  let mut output= "".to_owned();

  while n > 0 {
    let r = n % 2;
    output.push_str(&r.to_string());
    n /= 2;
  }

  output.chars().rev().collect::<String>()
}
