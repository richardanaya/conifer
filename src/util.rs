use rand::Rng;

pub fn random<T>() -> T where T:rand::Rand{
    rand::thread_rng().gen::<T>()
}