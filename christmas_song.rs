fn main(){
    let days = ["first", "second", "third", "fourth", "fifth", "sixth",
                "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let presents = [ "A partridge in a pear tree",
                    "Two turtle doves and",
                    "Three french hens",
                    "Four calling birds",
                    "Five golden rings",
                    "Six geese a-laying",
                    "Seven swans a-swimming",
                    "Eight maids a-milking",
                    "Nine ladies dancing",
                    "Ten lords a-leaping",
                    "Eleven pipers piping",
                    "Twelve drummers drumming"];
    let mut day = 0;
    
    while day < 12 {
        println!("On the {} day of Christmas, my true love sent to me", days[day]);
        for index in (0..day+1).rev() {
            println!("{}", presents[index]);
        }
        println!("");
        day += 1;
    }
}
