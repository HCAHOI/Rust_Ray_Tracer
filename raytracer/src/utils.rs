pub const PI: f64 = std::f64::consts::PI;

#[macro_export]
macro_rules! world_add {
    ($world:expr, $object:expr) => {{
        $world.push(Box::new($object));
    }};
}
