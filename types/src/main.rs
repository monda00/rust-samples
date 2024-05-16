fn main() {
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));

    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta", "Coleoptera"];
    println!("{:?}", lazy_caterer);
    println!("{:?}", taxonomy);

    let mut primes = vec![2, 3, 5, 7];
    primes.push(11);
    primes.push(13);

    let default_win_install_path = r"C:\Program Files\WindowsApps";
    println!("{}", default_win_install_path);

    println!(r###"
        This raw string started with 'r###"'.
        Therefore it does not end until we reach a quote mark('"')
        followed immediately by three pound signs ('###'):
    "###);
}
