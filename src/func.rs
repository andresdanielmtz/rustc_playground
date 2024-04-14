use rand::Rng;

pub(crate) fn get_random_number() -> i32 {
    let mut rng = rand::thread_rng();

    let n1: i32 = rng.gen_range(0..10);
    return n1;
}