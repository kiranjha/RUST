/*Unlike functions, methods are defined within the context of a struct (or an enum or a trait object, which we cover in Chapters 6 and 17, respectively), and 
their first parameter is always self, which represents the instance of the struct the method is being called on. */
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle: {} X {}",self.width,self.height);
    }
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
fn main() {
    let my_rect = Rectangle{
        width: 10,
        height: 5,
    };
    my_rect.print_description();
    println!("Rectangle is a square : {}",my_rect.is_square());

}
