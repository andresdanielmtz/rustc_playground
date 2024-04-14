mod func;

fn get_grade(grade: i32) -> &'static str {
    match grade {
        90..=100=>"A",
        80..=89=>"B",
        70..=79=>"C",
        60..=69=>"D",
        _=>"F"
    }
}

fn main() {
    println!("{}", func::get_random_number());
}

