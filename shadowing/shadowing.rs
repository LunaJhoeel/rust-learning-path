fn main(){
    // Declare first var binding
    let shadow_num = 5;

    // Declare second var binding, shadows existing var
    let shadow_num = shadow_num + 4;

    // Declare third var biding, shadows second binding
    let shadow_num = shadow_num * 2;

    println!(
        "The number is {}.",
        shadow_num
    )
}