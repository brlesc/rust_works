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

    // string to int convert (with error)
    let missing_str_num: String = String::from("1258ea");
    let mut conv_int:u32 = 0;

    // Error Handling -  (switch case like)
    match missing_str_num.parse::<u32>() {
        Ok(n)=> conv_int = n,
        Err(err) => println!("There is convert problem ! ({})",err.to_string().to_uppercase())
    }
    println!("conv_int: {}", conv_int);

    let my_encoded_string:String = String::from("enis#aslan-beyza#aslan");
    let user_list:Vec<String> = my_encoded_string.split("-").map(str::to_string).collect(); // .map(|s| s.to_string())

    // print vector object count
    println!("user_list member count: {}", user_list.len());

    // loop in the list 
    let mut i: u8= 0;
    for userx in user_list.iter() {

        // get user informations 
        let user_info:Vec<String> = userx.split("#").map(str::to_string).collect();
        let user_fname:String = user_info[0].to_string();
        let user_lname:String = user_info[1].to_string();

        // print the user informations 
        println!("Member {}  First Name: {}", i, user_fname);
        println!("Member {}  Last Name: {}", i, user_lname);
        i = i+1;
    }
}

