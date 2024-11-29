//RealSubject
struct Image {
    file_name: String,
}
impl Image {
    fn draw(&self) {
        println!("draw {}", self.file_name);
    }
}
//Proxy
struct ImageProxy {
    file_name: String,
    image: Option<Image>,
}
impl ImageProxy {
    fn get_image(&mut self) -> Option<&Image> {
        if self.image.is_none() {
            self.image = Some(Image {
                file_name: self.file_name.clone(),
            });
        }
        return self.image.as_ref();
    }
    fn get_file_name(&self) -> String {
        return self.file_name.clone();
    }
    fn draw(&mut self) {
        let image: &Image = self.get_image().unwrap();
        image.draw();
    }
}

pub(crate) fn test() {
    //Client
    let mut proxy = ImageProxy {
        file_name: String::from("1.png"),
        image: None,
    };
    //operation without creating a RealSubject
    let file_name = proxy.get_file_name();
    println!("file_name is {file_name}");
    //forwarded to the RealSubject
    proxy.draw();
}
