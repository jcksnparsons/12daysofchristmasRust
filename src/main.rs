fn main() {
    const CHRISTMAS_CAROL_LYRICS: [&str; 12] = ["a patridge in a pear tree", "two turtle doves", "three french hens", "four calling birds", "FIVE GOLDEN RINGS!", "six geese a-laying", "seven swans a-swimming", "eight mades a milking", "nine ladies dancing", "ten lords-a leaping", "eleven pipers piping", "twelve drummers drumming"];

    for i in 0..CHRISTMAS_CAROL_LYRICS.len() {
        println!("On the 12th day of christmas my true love gave to me:");
        println!("{}", CHRISTMAS_CAROL_LYRICS[i]);
    }
}
