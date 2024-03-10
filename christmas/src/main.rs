const ORDINAL_NUMBERS: [&str; 12] = [
    "first", "second", "third", "fourth", "fith", "sith", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: [&str; 11] = [
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

fn main() {
    for x in 0..12 {
        println!("On the {} day of Christmas", ORDINAL_NUMBERS[x]);
        println!("my true love gave to me");

        let gift_index = if x == 0 { 0 } else { x - 1 };
        for y in (0..gift_index).rev() {
            println!("{}", GIFTS[y]);
        }

        println!(
            "{} partridge in a pear tree",
            if x > 0 { "and a" } else { "A" }
        );
        println!("",);
    }
}
