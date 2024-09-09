struct Center;

trait SomeMethodNeedCenter {
    fn aa();
}
impl<F: Center> SomeMethodNeedCenter for F {

}
