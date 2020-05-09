/// Parses &[&str] into Vec<f32>.
pub fn f(args: &[&str]) -> Result<Vec<f32>, std::num::ParseFloatError> {
    args.iter().map(|arg| arg.parse()).collect()
}
