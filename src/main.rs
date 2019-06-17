use levelspecter::levelspecparser;


fn main() {
    println!("levelspecparser(\"DEV01.RD.9999.343\") {:?}",levelspecparser("DEV01.RD.9999.343"));
    println!("levelspecparser(\"DEV01.RD.9999\") {:?}",levelspecparser("DEV01.RD.9999"));
    println!("levelspecparser(\"DEV01.RD\") {:?}",levelspecparser("DEV01.RD"));
    println!("levelspecparser(\"DEV01\") {:?}",levelspecparser("DEV01"));

}
