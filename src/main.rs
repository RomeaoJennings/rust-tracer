use rust_tracer::{primitives::Canvas, scenes::three_balls_in_a_room, tracing::Tracer};

fn main() {
    let mut canvas = Canvas::new(400, 400, None);
    let (world, camera) = three_balls_in_a_room::build(&canvas);
    Tracer::trace_world(&world, &camera, &mut canvas);
    let result = canvas.to_ppm("C:/Users/romea/source/rust-tracer/output.ppm");
    if result.is_err() {
        println!("{:?}", result.err());
    }
}
