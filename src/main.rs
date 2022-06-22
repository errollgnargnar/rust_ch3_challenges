fn main() {
    convert_temp(100.0, 'f');
    convert_temp(32.0, 'c');
    gen_nth_fib(20);
    twelve_days_xmas();
}

fn convert_temp(degrees: f32, scale: char) {
    if scale == 'c' {
        let fahr = (degrees * 1.8) + 32.0;
        println!("{} degrees celsius is {} degrees fahrenheit", degrees, fahr);
    } else {
        let cels = (degrees - 32.0) * 0.5556;
        println!("{} degrees fahrenheit is {} degrees celsius", degrees, cels);
    }
}

fn gen_nth_fib(n: u32) {
    let mut counter: u32 = 0;
    let mut preva: u32 = 0;
    let mut prevb: u32 = 0;
    let mut current: u32 = 0;
    while counter < n {
        if counter == 0 {
            println!("{}", current);
            preva = current;
            current += 1;
            prevb = current;
        } else {
            println!("{}", current);
            current = preva + prevb;
            preva = prevb;
            prevb = current;
        }
        counter += 1;
    }
}

fn twelve_days_xmas() {
    let ordinal_nums: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth"];

    let gifts: [&str; 12] = ["A partridge in a pear tree", "Two turtle doves", "Three French hens", "Four calling birds","Five gold rings (five golden rings)", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"]; 

    for n in 0..12 {
        let mut gift_num = n;

        println!("On the {} day of Christmas, my true love gave to me..", ordinal_nums[n]);
        gift_num += 1;
        while gift_num > 0 {
            if gift_num-1 == 0 && n != 0 {
                println!("and {}", gifts[gift_num-1]);
            } else {
                println!("{}", gifts[gift_num-1]);
            }
            gift_num -= 1;
        }
    }
}
