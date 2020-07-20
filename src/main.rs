fn main() {
    let christmas_carol_lyrics: [&str; 12] = [
        "a patridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "FIVE GOLDEN RINGS!",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight mades a milking",
        "nine ladies dancing",
        "ten lords-a leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for line in christmas_carol_lyrics.iter() {
        println!("{}", line)
    }
}
