use std::fmt::Debug;

trait AnyYouThoughtJustDefine {
    fn any_you_thought_just_define(&self) -> String;
}

struct IEverNotKnowHereWhatIsThis(String);
struct ThisStepWillReturnABool(bool);
enum GeneralHappenedEventAndWeCanAddReceivedParams {
    ForExampleThis(IEverNotKnowHereWhatIsThis), // and I return back top to define The struct what wrap a String
    Step1(ThisStepWillReturnABool),
    SomeNextStep2IDireactToSayThisIsAPersonName(String),
}
struct SoIfNowIDefineAEventGenerator {
    event: GeneralHappenedEventAndWeCanAddReceivedParams,
}
fn catchEventHappened(SoIfNowIDefineAEventGenerator { event }: SoIfNowIDefineAEventGenerator) {
    type Here = GeneralHappenedEventAndWeCanAddReceivedParams;
    match event {
        Here::ForExampleThis(IEverNotKnowHereWhatIsThis(string)) => {
            println!("I catch a event happened, and the string is: {}", string);
        }
        Here::Step1(ThisStepWillReturnABool(bool)) => {
            println!("I catch a event happened, and the bool is: {}", bool);
        }
        Here::SomeNextStep2IDireactToSayThisIsAPersonName(string) => {
            println!("I catch a event happened, and the string is: {}", string);
        }
    }
}

struct Event(GeneralHappenedEventAndWeCanAddReceivedParams);
// then I test the From and To
impl From<GeneralHappenedEventAndWeCanAddReceivedParams> for Event {
    fn from(value: GeneralHappenedEventAndWeCanAddReceivedParams) -> Self {
        Event(value)
    }
}

trait Dusk {
    fn color() -> String;
}

trait Phone {
    fn shape(&self) -> String;
}

struct Iphone {
    round: bool,
}

impl Phone for Iphone {
    fn shape(&self) -> String {
        if self.round == true {
            "Round!!".to_owned()
        } else {
            "oh no".to_owned()
        }
    }
}

// 来搞一下函数参数中传参数格式化的函数是如何做的
// 可以从特性上收集信息，在函数参数的模式匹配
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
fn run() {
    let point = (3, 5);
    print_coordinates(&point);
}
struct Pos(i32, i32);
fn print_coordinates2(&Pos(x, y): &Pos) {
    println!("Current location: ({}, {})", x, y);
}
fn run2() {
    let point = Pos(3, 5);
    print_coordinates2(&point);
}
struct Pos2<T>(T);
#[derive(Debug, Clone, Copy)]
struct Coord(i32, i32, i32);
fn print_coordinates3<T>(&Pos2(t): &Pos2<T>)
where
    T: Debug + Copy,
{
    println!("Current location: ({:?})", t);
}
fn run3() {
    let point = Pos2(Coord(1, 2, 3));
    print_coordinates3(&point);
}
// 都是解构的内容
fn destructure() {
    let (x,y) = (1,2);
    // fn ( _(x,y): (i32,i32), )
}


// 自上而下
fn calc_distance<T> (p1: T, p2:T) -> f64 where T: Point,{ 
    p1.distance(&p2)
}
trait Point {
    fn distance(&self, other: &Self) -> f64;
}
struct Point2D {
    x: f64,
    y: f64,
}
impl Point for Point2D {
    fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
fn run4(){ 
    let p1 = Point2D { x: 0.0, y: 0.0 };
    let p2 = Point2D { x: 1.0, y: 1.0 };
    println!("{}", calc_distance(p1, p2));
}
/// 



fn main() {
    // 这里使用起来关于trait
}
