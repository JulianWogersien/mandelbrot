pub mod math {
    

    #[allow(dead_code)]
    pub fn scale(v_max: f32, v_min: f32, o_max: f32, o_min: f32, v: f32) -> f32 {
        return ((v - v_min) / (v_max - v_min)) * (o_max - o_min) + o_min;
    }

    #[allow(dead_code)]
    pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (i32, i32, i32) {
        if h > 360.0 || h < 0.0 || s > 100.0 || s < 0.0 || v > 100.0 || v < 0.0 {
            println!("hue {}  saturation {}  value {} ", h, s, v);
            panic!("error converting hsv to rgb, values outside range");
        }
        let s: f32 = s / 100.0;
        let v: f32 = v / 100.0;
        let c: f32 = s * v;
        let x: f32 = c * (1.0 - f32::abs((h / 60.0 % 2.0) - 1.0));
        let m: f32 = v - c;
        let r: f32;
        let g: f32;
        let b: f32;
        if h >= 0. && h < 60. {
            r = c;
            g = x;
            b = 0.;
        }
        else if h >= 60. && h < 120. {
            r = x;
            g = c;
            b = 0.;
        }
        else if h >= 120. && h < 180. {
            r = 0.;
            g = c;
            b = x;
        }
        else if h >= 180. && h < 240. {
            r = 0.;
            g = x;
            b = c;
        }
        else if h >= 240. && h < 300. {
            r = x;
            g = 0.;
            b = c;
        }
        else {
            r = c;
            g = 0.;
            b = x;
        }

        let r: i32 = ((r + m) * 255.) as i32;
        let g: i32 = ((g + m) * 255.) as i32;
        let b: i32 = ((b + m) * 255.) as i32;
        return (r, g, b);
    }
}