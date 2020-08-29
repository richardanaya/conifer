use rand::Rng;

pub fn random() -> f32 {
    rand::thread_rng().gen::<f32>()
}