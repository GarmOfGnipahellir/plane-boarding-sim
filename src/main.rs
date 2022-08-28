use nannou::prelude::*;
use petgraph::{graph::UnGraph, Graph};

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

enum AreaType {
    Aisle,
    Seat,
}

struct PlaneNode {
    position: Vec2,
    area_type: AreaType,
}

type PlaneGraph = UnGraph<PlaneNode, ()>;

struct Model {
    graph: PlaneGraph,
}

fn model(_app: &App) -> Model {
    let mut graph = PlaneGraph::default();
    let entrance = graph.add_node(PlaneNode {
        position: Vec2::ZERO,
        area_type: AreaType::Aisle,
    });
    let left_seat = graph.add_node(PlaneNode {
        position: vec2(-50.0, -50.0),
        area_type: AreaType::Seat,
    });
    Model { graph }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.to_frame(app, &frame);
}
