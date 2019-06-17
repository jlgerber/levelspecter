use levelspecter::levelspec_parser;


fn main() {
    println!("levelspec_parser(\"DEV01.RD.9999.343\") {:?}",levelspec_parser("DEV01.RD.9999.343"));
    println!("levelspec_parser(\"DEV01.rd.9999\") {:?}",levelspec_parser("DEV01.rd.9999"));

    println!("levelspec_parser(\"DEV01.RD.9999\") {:?}",levelspec_parser("DEV01.RD.9999"));
    println!("levelspec_parser(\"DEV01.RD\") {:?}",levelspec_parser("DEV01.RD"));
    println!("levelspec_parser(\"DEV01\") {:?}",levelspec_parser("DEV01"));

}
