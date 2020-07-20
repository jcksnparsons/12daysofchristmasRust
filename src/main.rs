fn main() {
    let christmas_carol_response: [&str; 12] = [
        "a patridge in a pear tree",
        "two turtle doves and",
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

    let christmas_carol_call: [&str; 12] = [
        "On the first day of Christmas, my true love gave to me:",
        "On the second day of Christmas, my true love gave to me:",
        "On the third day of Christmas, my true love gave to me:",
        "On the fourth day of Christmas, my true love gave to me:",
        "On the fifth day of Christmas, my true love gave to me:",
        "On the sixth day of Christmas, my true love gave to me:",
        "On the seventh day of Christmas, my true love gave to me:",
        "On the eighth day of Christmas, my true love gave to me:",
        "On the ninth day of Christmas, my true love gave to me:",
        "On the tenth day of Christmas, my true love gave to me:",
        "On the eleventh day of Christmas, my true love gave to me:",
        "On the twelfth day of Christmas, my true love gave to me:",
    ];

    for i in 0..christmas_carol_call.len() {
        println!("{}", christmas_carol_call[i]);

        let slice = &christmas_carol_response[0..i + 1];

        match i {
            1 => {
                for j in slice {
                    println!("{}", j)
                }
            }
            _ => {
                for j in slice {
                    println!("{}", j)
                }
            }
        }
    }
}
