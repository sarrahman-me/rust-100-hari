use std::io::stdin;

fn conver_age(umur: u8) -> u32 {
    let jumlah_hari = umur as u32 * 365;

    jumlah_hari
}

fn main() {
    let mut input = String::new();

    println!("Berapa umur anda sekarang :");

    stdin()
        .read_line(&mut input)
        .expect("Gagal menmbaca input pengguna");

    match input.trim().parse::<u8>() {
        Ok(umur) => {
            let hari = conver_age(umur);

            println!("Umur anda sekarang berkisar {} hari", hari)
        }
        Err(_) => {
            println!("Input tidak valid, harap masukan umur yang benar")
        }
    }
}
