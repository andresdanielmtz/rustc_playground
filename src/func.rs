use rand::Rng;

pub(crate) fn get_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..10);
}