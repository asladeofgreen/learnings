use alloy_sol_types::sol;

sol! {
    struct ComputeHyptoneusePublicParams {
        uint8 x;
        uint32 y;
        uint32 z;
    }
}

pub fn hypotoneuse(x: u8, y: u32) -> u32 {
    (((x as u32).pow(2) * y.pow(2)) as f64).sqrt() as u32
}
