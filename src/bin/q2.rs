const INPUT: &str = "655-1102,2949-4331,885300-1098691,1867-2844,20-43,4382100-4484893,781681037-781860439,647601-734894,2-16,180-238,195135887-195258082,47-64,4392-6414,6470-10044,345-600,5353503564-5353567532,124142-198665,1151882036-1151931750,6666551471-6666743820,207368-302426,5457772-5654349,72969293-73018196,71-109,46428150-46507525,15955-26536,65620-107801,1255-1813,427058-455196,333968-391876,482446-514820,45504-61820,36235767-36468253,23249929-23312800,5210718-5346163,648632326-648673051,116-173,752508-837824";

// 学习 filter_map 和 split_once，如何 split 字符串
fn parse_input(input: impl AsRef<str>) -> Vec<(u64, u64)> {
    let s: &str = input.as_ref();
    let parts_iter = s.split(',').map(|x| x.trim());
    println!("{:?}", parts_iter.clone().collect::<Vec<_>>());
    let iter = parts_iter.filter_map(|p| {
        let (a, b) = p.split_once('-')?;
        let a = a.parse::<u64>().ok()?;
        let b = b.parse::<u64>().ok()?;
        Some((a, b))
    });
    iter.collect::<Vec<(u64, u64)>>()
}

// 学习如何将 F 传入，并返回一个 Iterator
fn get_invalid_ids<F>(start: u64, end: u64, is_invalid_fn: F) -> impl Iterator<Item = u64>
where
    F: Fn(u64) -> bool,
{
    (start..=end).filter(move |&x| is_invalid_fn(x))
}

fn is_invalid(num: u64) -> bool {
    let mut carry = 1;
    let mut num_mut = num;
    loop {
        if num_mut / 10 > 0 {
            num_mut /= 10;
            carry += 1;
        } else {
            break;
        }
    }
    if carry & 1 == 1 {
        return false;
    }
    let factor = 10u64.pow(carry >> 1);
    let left = num / factor;
    let right = num % factor;
    left == right
}

fn main() {
    let inputs = parse_input(INPUT);
    let mut invalid_ids: Vec<u64> = Vec::new();
    for (start, end) in inputs {
        // 学习两个 Vec 如何 extend
        invalid_ids.extend(get_invalid_ids(start, end, is_invalid));
    }
    println!("{:?}", invalid_ids.iter().sum::<u64>());
}
