enum IpAddrKind {
    V4(u32, String),
    V6(String),
}

enum Status {
    Dispute,
    Completed,
    Pending,
}

fn status_to_tailwind(status: Status) -> &'static str {
    match status {
        Status::Dispute => "bg-red-500",
        Status::Completed => "bg-green-500",
        Status::Pending => "bg-yellow-500",
    }
}

// fn status_to_tailwind(status: Status) -> String {
//     match status {
//         Status::Dispute => String::from("bg-red-500"),
//         Status::Completed => String::from("bg-red-500"),
//         Status::Pending => String::from("bg-red-500"),
//     }
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("Biggest Coin!");
            25
        }
    }
}

// enum Option<T>

fn plus_one(number: Option<u32>) -> Option<u32> {
    match number {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn lad(dice_roll: u8) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

fn run_only_if_available(number: Option<u32>) {
    if let Some(number) = number {
        println!("It issss!!!")
    }
}

fn main() {
    println!("Hello, world!");

    run_only_if_available(None);
    run_only_if_available(Some(4444));

    let two = plus_one(Some(1));

    let my_coin = value_in_cents(Coin::Nickel);

    let pend = status_to_tailwind(Status::Pending);

    let _my_ip = IpAddrKind::V4(444, String::from("yolo"));
    let _hell = IpAddrKind::V6(String::from("Whoa"));

    #[derive(Debug)]
    struct Hello(String);

    let yo = Hello(String::from("Hello"));

    println!("Greetings {:?}", yo);

    // builtin enums

    let unknown_number: Option<u32> = None;
    let unknown_char: Option<char> = None;
}
