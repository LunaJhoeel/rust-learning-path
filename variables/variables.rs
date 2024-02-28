fn main(){
    // Declare a var
    let mut a_number;

    // Declare a second var and bind a value
    let a_word = "Ten";

    // Bind a value to first var
    a_number = 10;

    println!(
        "The number is {}",
        a_number
    );
    println!(
        "The word is {}",
        a_word
    );

    // Change the value of an immutable variable
    a_number = 15;
    println!(
        "The number is {}",
        a_number
    );
}