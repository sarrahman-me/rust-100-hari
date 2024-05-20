fn barbecue_skewers(barbecues: [&str; 5]) -> [i32; 2] {
    let mut vegetarian: i32 = 0;
    let mut non_vegetarian: i32 = 0;

    for barbacue in barbecues {
        if barbacue.contains("-x") {
            non_vegetarian += 1
        } else if barbacue.contains("-o") {
            vegetarian += 1
        }
    }

    [vegetarian, non_vegetarian]
}

fn main() {
    let barbecues: [&str; 5] = [
        "--oooo-ooo--",
        "--xxxxxxxx--",
        "--o---",
        "-o-----o---x--",
        "--o---o-----",
    ];

    let count_barbaque: [i32; 2] = barbecue_skewers(barbecues);

    println!(
        "terdapat {} barbaque untuk vegan dan {} barbaque daging",
        count_barbaque[0], count_barbaque[1]
    )
}
