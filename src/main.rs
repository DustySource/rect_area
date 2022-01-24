


// Correct Yet Inefficent

/*fn main() {
    let height = 10;
    let width = 50;

    println!("The area of the rect is {}", 
    area(width,height));




}



fn area(x:i32, y:i32) -> i32{
    x*y
    


}*/

/*
More Efficent(uses tuples) Yet Still could be better :]

fn main(){
let rect1= (10,50);

println!("The area of the rectangle is {}",
area(rect1));




}

fn area(dimensions:(i32,i32))->i32{
    dimensions.0*dimensions.1


}
*/



//With Structs
/* 
struct RectCreator{
    height:i32,
    width:i32,

}
fn main(){


let mut rect1 = RectCreator{height:10,width:50};

rect1.height = 4;

println!("Rectangle area = {}", area(&rect1))


}

fn area(rect:&RectCreator)->i32{
rect.height*rect.width
    

}*/

//With Structs but implementing with the struct

struct ShapeCreator{
    height:i32,
    width:i32,

}
impl ShapeCreator{
    fn square (size: i32)-> ShapeCreator{
        ShapeCreator{
            height:size,
            width:size,
        }
       
    }
    fn area(&self)->i32{
    self.height*self.width
    }
    fn can_hold(&self, other:&ShapeCreator) -> bool{
        self.width > other.width && self.height > other.height


    }

}
fn main(){

let square1 = ShapeCreator::square(40);
let mut rect1 = ShapeCreator{height:10,width:50};
let rect2 = ShapeCreator{height:2, width:49};
rect1.height = 4;

println!("Rectangle area = {}", rect1.area());
println!("Rectangle 1 can hold rect 2 {}", rect1.can_hold(&rect2));
println!("Square created is equal to {}ft wide and {} tall", &square1.width,&square1.height)


}

