use std::net::ToSocketAddrs;

fn valid_word_abbreviation(word: String, abbr: String) -> bool {
    let word = word.chars().collect::<Vec<_>>();
    let abbr = abbr.chars().collect::<Vec<_>>();

    let mut nums = Vec::<usize>::new();
    for (i, c) in abbr.iter().enumerate() {
        if c.is_ascii_digit() {
            if (i == 0 && *c == '0')
                || (i > 0 && *c == '0' && !abbr[i-1].is_ascii_digit()) {
                return false;
            }

            let digit = c.to_digit(10).unwrap() as usize;
            if i > 0 && abbr[i-1].is_ascii_digit() {
                if let Some(num) = nums.get_mut(0) {
                    *num = *num * 10 + digit;

                    if *num > word.len() - (i - num.to_string().len()) {
                        return false;
                    }
                }
            } else {
                nums.insert(0, digit);
            }
        }
    }

    if nums.is_empty() {
        return word == abbr;
    }

    let mut res = Vec::new();
    for (i, c) in abbr.iter().enumerate() {
        if c.is_ascii_digit() {
            if i > 0 && abbr[i-1].is_ascii_digit() {
                continue;
            }

            if let Some(n) = nums.pop() {
                res.append(&mut vec!['_'; n]);
            }
        } else {
            res.push(abbr[i]);
        }
    }

    if word.len() != res.len() {
        return false;
    }

    for (i, c) in res.iter_mut().enumerate() {
        if *c == '_' {
            *c = word[i];
        }
    }

    word == res
}

pub fn main() {
    let word = "localization".to_string();
    let abbr = "l10n".to_string();
    println!("{}", valid_word_abbreviation(word, abbr));
}
