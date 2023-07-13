#[cfg(test)]
pub mod compiletest {
    #[rustversion::all(stable, since(1.71), before(1.72))]
    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.compile_fail("src/compiletest/*.rs");
    }
}
