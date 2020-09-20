pub mod hash;
pub mod luid;
pub mod prop;

#[macro_export]
macro_rules! suid {
    () => {{
        let f = module_path!();
        let l = line!();
        let c = column!();
        crate::util::hash::new(f.to_string() + &l.to_string() + &c.to_string())
    }};
}
