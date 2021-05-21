pub trait DefenseCalcs {
    fn calc_armor(&self) -> i8;
    fn calc_fortitude(&self) -> i8;
    fn calc_mental(&self) -> i8;
    fn calc_reflex(&self) -> i8;
}
