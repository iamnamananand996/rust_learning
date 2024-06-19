fn main() {
    let v1 = vec![1, 2, 2, 4];
    let v1_iter = v1.iter();

    // for i in v1_iter {
    //     println!("{}", i)
    // }

    let result: i32 = v1_iter.sum();

    println!("{}", result);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes_data = vec![
            Shoe {
                size: 3,
                style: String::from("boot"),
            },
            Shoe {
                size: 5,
                style: String::from("loafer"),
            },
            Shoe {
                size: 7,
                style: String::from("chappie"),
            },
        ];

        let size = shoes_in_my_size(shoes_data, 5);
        assert_eq!(
            size,
            vec![Shoe {
                size: 5,
                style: String::from("loafer")
            }]
        )
    }
    #[test]
    fn counter_test() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1)); // 2,6,12,20
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .sum();

        assert_eq!(40, sum)
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count = self.count + 1;
            Some(self.count)
        } else {
            None
        }
    }
}
