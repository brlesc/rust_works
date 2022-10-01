use std::string::String;

fn main() 
{
    let my_string = String::from("my string");
    println!("{}", my_string);

    // autiomatic type cast 
    let index= 2344;

    // format is like sprintf from c 
    let my_edited_str = format!("{} no:{}", my_string, index);
    println!("{}", my_edited_str);

    // forced type cast 
    let my_number: u32 = 8547;

    // convert to string 
    let str_num: String = my_number.to_string();

    // >>> print an println functions are different 
    print!("Converted String : {} \r\n", str_num);

    // convert string to int value 
    let x_value: u32 = 1453;
    let y_value: u32 = str_num.parse::<u32>().unwrap();
    println!("x_value + y_value : {}", x_value + y_value);

    // string to int convert error handling 
    let missing_str_num: String = String::from("1258ea");
    let mut conv_int:u32 = 0;

    // switch - case like 
    match missing_str_num.parse::<u32>() {
        Ok(n)=> conv_int = n,
        Err(err) => println!("There is convert problem ! ({})",err.to_string().to_uppercase())
    }

    println!("conv_int: {}", conv_int);
    

}
