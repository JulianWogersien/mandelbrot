pub mod math {
    pub fn scale(v_max: f32, v_min: f32, o_max: f32, o_min: f32, v: f32) -> f32 {
        return ((v - v_min) / (v_max - v_min)) * (o_max - o_min) + o_min;
    }
}