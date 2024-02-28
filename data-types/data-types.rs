fn main(){
    let number:u32 = 14;
    println!(
        "The number is {}",
        number
    );

    // i: signed number (+ or -)
    // u: unsigned number (0 or +)

    let _number_64 = 4.0; // compiler infers the value by default must be type f64
    let _number_32:f32 = 5.0;  // type f32 specified via annotation

    // Primitive number types
    // Addition, subtraction, multiplication
    println!(
        "1 + 2 = {}\nand\n8 - 5 = {}\nand\n15 * 3 = {}",
        1u32 + 2,
        8i32 - 5,
        15 * 3
    );
    // Integer and floating point division
    println!(
        "9 / 2 = {}\nbut\n9.0 / 2.0 = {}",
        9u32 / 2,
        9.0 / 2.0
    );

    // Booleans
    // Declare var to store result of "greater than" test, Is 1 > 4? -- false
    let is_bigger = 1 > 4;
    println!(
        "Is 1 > 4? {}",
        is_bigger
    );

    // Text: characters
    // let uppercase_s = 'S';
    // let lowercase_f = 'f';
    // let smiley_face = '😃';

    // Text: strings

    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';

    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face_2 = '😃';

    // Compiler interprets a series of items in quotations as a "str" data type
    // and creates a "&str" reference
    let string_1 = "miley";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: &str = "ace";

    println!(
        "{} is a {}{}{}{}.",
        smiley_face_2,
        character_1,
        string_1,
        character_2,
        string_2
    );
}
