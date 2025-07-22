// // Serde

// use serde::{ Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug)]
// struct User {
//     username: String,
//     password: String,
// }

// fn main() {
//     let s = String::from("{\"username\": \"sada\", \"password\": \"123123\"}");

//     let u: Result<User, serde_json::Error> = serde_json::from_str(&s);

//     println!("{:?}", u.unwrap());
// }




// // Borsh

// use borsh::{BorshSerialize, BorshDeserialize};

// #[derive(BorshSerialize, BorshDeserialize, Debug)]
// struct User {
//     username: String,
//     password: String,
// }

// fn main() {
//     let user = User {
//         username: String::from("sada"),
//         password: String::from("sada98767"),
//     };
//     let mut serializing: Vec<u8> = Vec::new();      // empty vector

//     let ans = user.serialize(&mut serializing);                    // serializing user

//     match ans {
//         Ok(_) => println!("{:?}", serializing),
//         Err(_) => println!("Error while serialization."),
//     }

//     let deserializing = User::try_from_slice(&serializing).unwrap();                   // deserializing user
//     println!("{:?}", deserializing);
// }





// Lifetimes

fn main() {
    let str1= String::from("sada");
    let ans;
    {
        let str2 = String::from("shiva");
        ans = longest_str(&str1, &str2);
        println!("{}", ans);
        
    }

    //println!("{}", ans);
}

fn longest_str<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        return  s1;
    } else {
        return s2;
    }
}