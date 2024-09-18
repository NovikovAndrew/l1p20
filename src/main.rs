fn main() {
    // адатпер на примере животных
    let cat = Cat::new();
    let dog = Dog::new();

    let cat_adapter = CatAdapter::new(cat);
    let dog_adapter = DogAdapter::new(dog);

    animal_sound(&cat_adapter); // meow mew
    animal_sound(&dog_adapter); // gav gav
}

// общий интерфейс для всех животных
trait Animal {
    fn make_sound(&self);
}

// cтруктура для собаки
struct Dog;

impl Dog {
    fn new() -> Self {
        Dog
    }

    fn gav(&self) {
        println!("gav gav")
    }
}

// cтруктура для кошки
struct Cat;

impl Cat {
    fn new() -> Self {
        Cat
    }

    fn meow(&self) {
        println!("meow mew")
    }
}

// адаптер для собаки
struct DogAdapter {
    dog: Dog,
}

impl DogAdapter {
    fn new(dog: Dog) -> Self {
        DogAdapter { dog }
    }
}

impl Animal for DogAdapter {
    fn make_sound(&self) {
        self.dog.gav()
    }
}

// адаптер для кошки
struct CatAdapter {
    cat: Cat,
}

impl CatAdapter {
    fn new(cat: Cat) -> Self {
        CatAdapter { cat }
    }
}

impl Animal for CatAdapter {
    fn make_sound(&self) {
        self.cat.meow()
    }
}

// функция для взаимодействия с адаптером
// любого типа которому соответсвует Animal
fn animal_sound(animal: &dyn Animal) {
    animal.make_sound();
}