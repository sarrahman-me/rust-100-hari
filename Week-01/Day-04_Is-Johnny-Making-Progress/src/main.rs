fn main() {
    let progress: Vec<u32> = vec![10, 11, 12, 9, 10];
    let mut kemajuan = 0;
    let mut day_before: u32 = 0;

    for (index, progres) in progress.iter().enumerate() {
        if index == 0 {
            day_before = *progres;
            continue;
        }

        if day_before < *progres {
            kemajuan += 1;
        }

        day_before = *progres
    }

    println!("jumlah kemajuan {} kali", kemajuan)
}
