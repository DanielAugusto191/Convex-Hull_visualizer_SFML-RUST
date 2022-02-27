#![allow(non_snake_case)]
use sfml::{graphics::*, system::*, window::*};
// use std::collections::LinkedList;

const V_SIZE: f32 = 5.;

fn dir(p: CircleShape, q: CircleShape, r: CircleShape) -> i32{
    // let val = ((q.position().y + V_SIZE - p.position().y + V_SIZE) * (r.position().x + V_SIZE - q.position().x + V_SIZE) - 
    //         (q.position().x + V_SIZE - p.position().x + V_SIZE) * (r.position().y + V_SIZE - q.position().y + V_SIZE));
    let val = (q.position().y - p.position().y) * (r.position().x - q.position().x) - 
    (q.position().x - p.position().x) * (r.position().y - q.position().y);
    // println!("val {}", val);
    if val == 0. {return 0;}
    if val > 0. { return 1;} else {return 2;}
}

fn eqVertice(a: &CircleShape, b: &CircleShape) -> bool{
    return a.position().x == b.position().x && a.position().y == b.position().y;
}
fn main() {
    let windows_size = (1600,900);
    let video_mode = VideoMode::desktop_mode();
    let mut window = RenderWindow::new(
        VideoMode::new(windows_size.0, windows_size.1, video_mode.bits_per_pixel),
        "Convex Hull",
        Style::CLOSE,
        &Default::default(),
    );

    let mut vertices: Vec<CircleShape> = Vec::new();
    let mut verticesOnHull: Vec<CircleShape> = Vec::new();
    let mut lines: Vec<Vec<Vertex>> = Vec::new();
    let mut lines2: Vec<Vec<Vertex>> = Vec::new();
    while window.is_open() {
        while let Some(event) = window.poll_event(){
            match event{
                Event::Closed => window.close(),
                Event::MouseButtonPressed{button: mouse::Button::LEFT, x, y} => {
                    vertices.push(CircleShape::new(V_SIZE, 200));
                    let n = vertices.len();
                    vertices[n-1].set_position((x as f32 - V_SIZE, y as f32 - V_SIZE));
                }
                Event::KeyPressed{code: Key::R, ..} => {
                    if vertices.len() >= 3 {
                        lines.clear();
                        lines2.clear();
                        verticesOnHull.clear();
                        let mut l: usize = 0;
                        let n = vertices.len();
                        for i in 1..n {
                            if vertices[i].position().x < vertices[l].position().x{
                                    l = i;
                            }
                        }
                        let mut p = l;
                        loop {
                            verticesOnHull.push(vertices[p].clone());
                            let mut q = (p+1)%n;
                            for i in 0..n {
                                if i == q {continue;}
                                let d = dir(vertices[p].clone(), vertices[i].clone(), vertices[q].clone());
                                if d == 2 { q = i;}
                            }
                            p = q;
                            if p == l {break;}
                        }
                        for i in 1..verticesOnHull.len(){
                            lines.push(vec![
                                Vertex::new(Vector2f::new(verticesOnHull[i-1].position().x + V_SIZE, verticesOnHull[i-1].position().y + V_SIZE), Color::BLUE, Vector2f::new(0., 0.)),
                                Vertex::new(Vector2f::new(verticesOnHull[i].position().x + V_SIZE, verticesOnHull[i].position().y + V_SIZE), Color::BLUE, Vector2f::new(0., 0.)),
                            ]);
                        }
                        lines.push(vec![
                            Vertex::new(Vector2f::new(verticesOnHull[verticesOnHull.len()-1].position().x + V_SIZE, verticesOnHull[verticesOnHull.len()-1].position().y + V_SIZE), Color::BLUE, Vector2f::new(0., 0.)),
                            Vertex::new(Vector2f::new(verticesOnHull[0].position().x + V_SIZE, verticesOnHull[0].position().y + V_SIZE), Color::BLUE, Vector2f::new(0., 0.)),
                        ]);
                        for i in 0..vertices.len(){
                            let mut t = false;
                            for j in 0..verticesOnHull.len(){
                                if eqVertice(&vertices[i], &verticesOnHull[j]){
                                    t = true;
                                    break;
                                }
                            }
                            if t {continue;}
                            for j in i+1.. vertices.len(){
                                let mut t = false;
                                for k in 0..verticesOnHull.len(){
                                    if eqVertice(&vertices[j], &verticesOnHull[k]){
                                        t = true;
                                        break;
                                    }
                                }
                                if t {continue;}
                                lines2.push(vec![
                                    Vertex::new(Vector2f::new(vertices[i].position().x + V_SIZE, vertices[i].position().y + V_SIZE), Color::RED, Vector2f::new(0., 0.)),
                                    Vertex::new(Vector2f::new(vertices[j].position().x + V_SIZE, vertices[j].position().y + V_SIZE), Color::RED, Vector2f::new(0., 0.)),
                                ]);
                            } 
                        }
                    }else{
                        println!("At least 3 vertices are needed!");
                    }
                }
                _ => {}
            }
        }
        window.clear(Color::BLACK);
        for i in vertices.iter_mut() {
            i.set_fill_color(Color::YELLOW);
            window.draw(i);
        }
        for i in verticesOnHull.iter_mut(){
            i.set_fill_color(Color::GREEN);
            window.draw(i);
        }
        for i in &lines {
            window.draw_primitives(&i, PrimitiveType::LINES, &RenderStates::default());
        }
        for i in &lines2 {
            window.draw_primitives(&i, PrimitiveType::LINES, &RenderStates::default());
        }
        window.display();
    }
}
