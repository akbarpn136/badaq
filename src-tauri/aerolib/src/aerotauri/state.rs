#[derive(Debug, Default)]
pub struct AppState {
    pub nama: String,
    pub koreksi: Vec<f32>,
    pub seq: String,
    pub koleksi: Vec<String>,
    pub koef: Vec<String>,
}