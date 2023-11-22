pub const PI: f64 = std::f64::consts::PI;

// for random scene only
#[macro_export]
macro_rules! world_add {
    ($world:expr, $object:expr) => {{
        $world.push(Box::new($object));
    }};
}
