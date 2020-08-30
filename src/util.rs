use rand::Rng;

pub fn random() -> f32 {
    rand::thread_rng().gen::<f32>()
}

pub fn random_n<T>(n: T) -> T
where
    T: Into<f32> + From<f32>,
{
    f32::floor(n.into() * random()).into()
}
