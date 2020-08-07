use turtle::{Turtle};

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
    turtle.go_to([-300., -100.]);
    turtle.set_speed(4);
    turtle.show();
    turtle.pen_down();
    
    h(&mut turtle);
    blank(&mut turtle);
    e(&mut turtle);
    blank(&mut turtle);
    l(&mut turtle);
    blank(&mut turtle);
    l2(&mut turtle);

    turtle.drawing().save_svg("turtle.svg");
}


fn blank(turtle: &mut Turtle) {
    turtle.pen_up();
    turtle.forward(100.);
    turtle.pen_down();
}

fn h(turtle: &mut Turtle) {
    turtle.forward(200.);
    turtle.backward(100.);
    turtle.right(90.);
    turtle.forward(50.);
    turtle.left(90.);
    turtle.forward(100.);
    turtle.backward(200.);
    turtle.right(90.);
}

fn e(turtle: &mut Turtle) {
    turtle.right(180.);
    for _ in 0..270 {
        // Move forward three steps
        turtle.forward(0.7);
        // Rotate to the right (anti-clockwise) by 1 degree
        turtle.right(1.);
    }
    turtle.right(90.);
    turtle.forward(80.);
    turtle.right(180.); 
    turtle.forward(50.);  
}

fn l(turtle: &mut Turtle) {
    turtle.left(90.);
    turtle.forward(150.);
    turtle.backward(200.);
    turtle.right(90.);
}

fn l2(turtle: &mut Turtle) {
    turtle.left(90.);
    turtle.forward(200.);
    turtle.backward(200.);
    turtle.right(90.);
}