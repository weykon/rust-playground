trait Animal {}

struct Dog;
struct Cat;

struct AnimalShelter<T: Animal>(T);

impl Animal for Dog {}
impl Animal for Cat {}

//
// fn accept_any_animal_shelter(shelter: AnimalShelter<dyn Animal>) {
fn accept_any_animal_shelter(shelter: Box<AnimalShelter<dyn Animal>>) {
    println!("Accepting any animal shelter");
}
