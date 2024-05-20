use std::io::stdin;

fn finding_nemo(kalimat: String) {
    // cek apakah kalimat mengandung kata "Nemo";
    if !kalimat.contains("Nemo") {
        return println!("Saya tidak dapat menemukan Nemo :(");
    }

    let kata_kata: Vec<&str> = kalimat.split_whitespace().collect();

    for (index, &_kata) in kata_kata.iter().enumerate() {
        if kata_kata[index] == "Nemo" {
            println!("Saya menemukan Nemo di urutan {}", index + 1)
        }
    }
}

fn main() {
    let mut input = String::new();

    println!("Masukkan kalimat random :");

    stdin()
        .read_line(&mut input)
        .expect("Gagal mengambil input pengguna");

    finding_nemo(input)
}
