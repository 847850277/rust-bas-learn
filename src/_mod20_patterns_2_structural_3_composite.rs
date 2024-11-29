//Component
trait Graphic {
    fn draw(&self);
}
//Leaf
struct Circle {}
impl Graphic for Circle {
    fn draw(&self) {
        println!("Draw circle")
    }
}
//Leaf
struct Square {}
impl Graphic for Square {
    fn draw(&self) {
        println!("Draw square")
    }
}
//Composite
struct Image {
    graphics: Vec<Box<dyn Graphic>>,
}
impl Graphic for Image {
    fn draw(&self) {
        println!("Draw image");
        for graphic in &self.graphics {
            graphic.draw();
        }
    }
}
impl Image {
    fn add(&mut self, graphic: Box<dyn Graphic>) {
        let _ = &self.graphics.push(graphic);
    }
    #[allow(dead_code)]
    fn remove(&mut self, index: usize) {
        self.graphics.remove(index);
    }
}

pub(crate) fn test() {
    //Client
    let mut image = Image { graphics: vec![] };
    image.add(Box::new(Circle {}));
    image.add(Box::new(Square {}));
    let mut picture = Image { graphics: vec![] };
    picture.add(Box::new(image));
    picture.draw();
}
