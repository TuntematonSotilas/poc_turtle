use turtle::{Drawing, Turtle};

fn main() {

    // Configure turtle
    let mut turtle = Turtle::new();
    turtle.set_pen_color("petrol");
    turtle.set_pen_size(1.0);
    
    // Configure drawing
    let drawing = turtle.drawing_mut();
    drawing.set_background_color("pale olive green");
    drawing.set_title("poc_turtle");
    drawing.set_size((800, 600));
    
    // Setup start position
    turtle.set_speed(12);
    turtle.hide();
    turtle.pen_up();
    turtle.go_to([-350., -100.]);
    turtle.set_speed(4);
    turtle.show();
    turtle.pen_down();

    // Letter H
    turtle.forward(200.);
    turtle.backward(100.);
    turtle.right(90.);
    turtle.forward(50.);
    turtle.left(90.);
    turtle.forward(100.);
    turtle.backward(200.);
    
    turtle.pen_up();
    turtle.right(90.);
    turtle.forward(50.);
}