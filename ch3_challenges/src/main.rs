fn main() {
    
    let fahr = 80.0;
    let converted = fahrenheit_to_celsius(fahr);

    println!("{}", converted);

    let song_lyrics:[&str; 12] = [
        "A partridge in a pear tree",
        "Two turtledoves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    let days = ["first", "second", 
    "third", "fourth", "fifth",
    "sixth", "seventh", "eigth",
    "ninth", "tenth", "eleventh", 
    "twelve" ];


    let sentence = "On the x day of Christmas, my true love sent me";

    let mut day_number: u32 = 1;

    for (number, day) in days.iter().enumerate() {

        println!("On the {} day of Christmas, my true love sent to me:", day);
    
        for i in (0..=number).rev() {
            
            if i == 0 && number > 0 {
                print!("and ");
            }
            println!("{}", song_lyrics[i]);
        }
        println!();
    }


}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 2.0 + 30.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (5.0/9.0) * (fahrenheit - 32.0) 
}


