#[allow(unused_macros)]
macro_rules! p{($x:expr,$t:ident)=>($x.trim().parse::<$t>().unwrap())}
#[allow(unused_macros)]
macro_rules! r{($l:expr)=>(io::stdin().read_line(&mut $l).unwrap())}
#[allow(unused_macros)]
macro_rules! t{($l:expr)=>($l=$l.trim_matches('\n').to_string())}

#[allow(dead_code)]
mod nlp100;

fn main() {
    println!("{:?}", nlp100::nlp005());
}
