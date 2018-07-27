#[derive(Debug)]
pub enum MaskPoint {
    Forehead,
    Eyes,
    Mouth,
    Chin,
}

#[derive(Debug)]
pub struct MaskPosition {
    point: MaskPoint,
    x_shift: f64,
    y_shift: f64,
    scale: f64,
}
