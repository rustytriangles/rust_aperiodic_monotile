use nannou::prelude::*;

#[macro_use]

#[path = "tile.rs"]
mod tile;

use tile::*;

struct DrawProps {
    fill_color1: nannou::color::Srgb<u8>,
    fill_color2: nannou::color::Srgb<u8>,
    edge_color: nannou::color::Srgb<u8>,
    edge_weight: f32,
}

trait Drawable {
    fn draw(&self, draw: &nannou::draw::Draw, xoff: f32, yoff: f32, scale: f32, props: &DrawProps);
    fn append_to_vector(&self, dst: &mut Vec<Box<dyn Drawable>>, dx: f64, dy: f64);
    fn get_drawable_edges(&self) -> Vec<Edge>;
    fn get_ammann_bars(&self, xoff: f32, yoff: f32, scale: f32) -> Vec<(f32,f32)>;
}

fn interpolate(p1: (f32,f32), p2: (f32,f32), t: f64) -> (f32,f32) {
    return (
        (p1.0 as f64 + t * ( p2.0 as f64 - p1.0 as f64)) as f32,
        (p1.1 as f64 + t * ( p2.1 as f64 - p1.1 as f64)) as f32
    )
}

impl Drawable for Unreflected {
    fn draw(&self, draw: &nannou::draw::Draw, xoff: f32, yoff: f32, scale: f32, props: &DrawProps) {
        let pts = self.polygon(xoff, yoff, scale);
        let points = (0..13).map(|i| {
            pt2(pts[i].0, pts[i].1)
        });
        draw.polygon()
            .color(props.fill_color1)
            .stroke(props.edge_color)
            .stroke_weight(props.edge_weight)
            .join_miter()
            .points(points);

    }

    fn append_to_vector(&self, dst: &mut Vec<Box<dyn Drawable>>, dx: f64, dy: f64) {
        dst.push(Box::new(tile::Unreflected {
            cx: self.cx + dx,
            cy: self.cy + dy,
            angle: self.angle
        }));
    }

    fn get_drawable_edges(&self) -> Vec<Edge> {
        self.get_edges()
    }

    fn get_ammann_bars(&self, xoff: f32, yoff: f32, scale: f32) -> Vec<(f32,f32)> {
        let s5 = 5_f64.sqrt();
        let t1 = 1. / 4.;
        let t2 = 1. / (3. + s5);
        let t3 = (1. + s5) / 4.;

        let p = self.polygon(xoff, yoff, scale);
        let a0 = interpolate(p[1], p[0], t3);
        let a1 = interpolate(p[1], p[0], t1);
        let a2 = interpolate(p[1], p[2], t2);
        let a3 = interpolate(p[3], p[2], t2);
        let a4 = interpolate(p[3], p[0], t1);
        let a5 = interpolate(p[3], p[0], t3);

        let mut result = Vec::new();
        result.push(a1); result.push(a2);
        result.push(a2); result.push(a0);
        result.push(a5); result.push(a3);
        result.push(a3); result.push(a4);
        return result
    }
}

impl Drawable for Reflected {
    fn draw(&self, draw: &nannou::draw::Draw, xoff: f32, yoff: f32, scale: f32, props: &DrawProps) {
        let pts = self.polygon(xoff, yoff, scale);
        let points = (0..13).map(|i| {
            pt2(pts[i].0, pts[i].1)
        });
        draw.polygon()
            .color(props.fill_color2)
            .stroke(props.edge_color)
            .stroke_weight(props.edge_weight)
            .join_miter()
            .points(points);


    }

    fn append_to_vector(&self, dst: &mut Vec<Box<dyn Drawable>>, dx: f64, dy: f64) {
        dst.push(Box::new(tile::Reflected {
            cx: self.cx + dx,
            cy: self.cy + dy,
            angle: self.angle
        }));
    }

    fn get_drawable_edges(&self) -> Vec<Edge> {
        self.get_edges()
    }

    fn get_ammann_bars(&self, xoff: f32, yoff: f32, scale: f32) -> Vec<(f32,f32)> {
        let s5 = 5_f64.sqrt();
        let t1 = 1. / 4.;
        let t2 = 1. / (3. + s5);

        let p = self.polygon(xoff, yoff, scale);
        let r0 = interpolate(p[0], p[1], t2);
        let r3 = interpolate(p[2], p[1], t1);
        let r4 = interpolate(p[2], p[3], t1);
        let r7 = interpolate(p[0], p[3], t2);

        let mut result = Vec::new();
        result.push(r3); result.push(r7);
        result.push(r7); result.push(r0);
        result.push(r0); result.push(r4);

        return result
    }
}

fn build_tile(tile: &tile::Tile, x: f64, y: f64, angle: i32) -> Result<Box<dyn Drawable>, i32> {
    match tile {
        tile::Tile::UNREFLECTED => Ok(Box::new(Unreflected::new(x, y, angle))),
        tile::Tile::REFLECTED => Ok(Box::new(Reflected::new(x, y, angle))),
    }
}

fn interp_angles(start_angle: i32, end_angle: i32) -> Vec<f64> {
    let mut result = Vec::new();

    let angles = if start_angle > end_angle {
        ( (start_angle as f64) * std::f64::consts::PI / 180.,
           ((end_angle + 360) as f64) * std::f64::consts::PI / 180. )
    } else {
        ( (start_angle as f64) * std::f64::consts::PI / 180.,
           (end_angle as f64) * std::f64::consts::PI / 180. )
    };

    for i in 0..25 {
        result.push(angles.0 + (i as f64) / 24. * (angles.1 - angles.0));
    }
    return result
}

// fn build_vertex1(x: f64, y: f64, angle: i32) -> Result<Vec<Box<dyn Drawable>>, i32> {
//     let phi = (1. + 5_f64.sqrt())/2.;
//     let d1 = Dart::new(x - phi, y, angle);
//     let d2 = place_dart_edge(3, d1.edge_center(2)?, d1.edge_angle(2)?);
//     let d3 = place_dart_edge(3, d2.edge_center(2)?, d2.edge_angle(2)?);
//     let d4 = place_dart_edge(3, d3.edge_center(2)?, d3.edge_angle(2)?);
//     let d5 = place_dart_edge(3, d4.edge_center(2)?, d4.edge_angle(2)?);
//     let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
//     tiles.push(Box::new(d1));
//     tiles.push(Box::new(d2));
//     tiles.push(Box::new(d3));
//     tiles.push(Box::new(d4));
//     tiles.push(Box::new(d5));
//     Ok(tiles)
// }

// fn build_vertex2(x: f64, y: f64, angle: i32) -> Result<Vec<Box<dyn Drawable>>, i32> {
//     let d1 = Dart::new(x, y, angle);
//     let k1 = place_kite_edge(2, d1.edge_center(4)?, d1.edge_angle(4)?);
//     let k2 = place_kite_edge(4, k1.edge_center(1)?, k1.edge_angle(1)?);

//     let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
//     tiles.push(Box::new(d1));
//     tiles.push(Box::new(k1));
//     tiles.push(Box::new(k2));
//     Ok(tiles)
// }

// fn build_vertex3(x: f64, y: f64, angle: i32) -> Result<Vec<Box<dyn Drawable>>, i32> {
//     let phi = (1. + 5_f64.sqrt())/2.;

//     let k1 = Kite::new(x + phi, y, angle);
//     let k2 = place_kite_edge(1, k1.edge_center(4)?, k1.edge_angle(4)?);
//     let k3 = place_kite_edge(1, k2.edge_center(4)?, k2.edge_angle(4)?);
//     let k4 = place_kite_edge(1, k3.edge_center(4)?, k3.edge_angle(4)?);
//     let k5 = place_kite_edge(1, k4.edge_center(4)?, k4.edge_angle(4)?);

//     let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
//     tiles.push(Box::new(k1));
//     tiles.push(Box::new(k2));
//     tiles.push(Box::new(k3));
//     tiles.push(Box::new(k4));
//     tiles.push(Box::new(k5));
//     Ok(tiles)
// }

// fn build_vertex4(x: f64, y: f64, angle: i32) -> Result<Vec<Box<dyn Drawable>>, i32> {
//     let phi = (1. + 5_f64.sqrt())/2.;

//     let d1 = Dart::new(x - phi, y, angle);
//     let d2 = place_dart_edge(3, d1.edge_center(2)?, d1.edge_angle(2)?);
//     let k1 = place_kite_edge(1, d2.edge_center(2)?, d2.edge_angle(2)?);
//     let k2 = place_kite_edge(1, k1.edge_center(4)?, k1.edge_angle(4)?);
//     let d3 = place_dart_edge(3, k2.edge_center(4)?, k2.edge_angle(4)?);

//     let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
//     tiles.push(Box::new(d1));
//     tiles.push(Box::new(d2));
//     tiles.push(Box::new(k1));
//     tiles.push(Box::new(k2));
//     tiles.push(Box::new(d3));
//     Ok(tiles)
// }

// fn build_vertex5(x: f64, y: f64, angle: i32) -> Result<Vec<Box<dyn Drawable>>, i32> {

//     let k1 = Kite::new(x - 1., y, angle);
//     let d1 = place_dart_edge(4, k1.edge_center(2)?, k1.edge_angle(2)?);
//     let k2 = place_kite_edge(1, d1.edge_center(3)?, d1.edge_angle(3)?);
//     let k3 = place_kite_edge(1, k2.edge_center(4)?, k2.edge_angle(4)?);
//     let d2 = place_dart_edge(2, k3.edge_center(4)?, k3.edge_angle(4)?);

//     let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
//     tiles.push(Box::new(k1));
//     tiles.push(Box::new(d1));
//     tiles.push(Box::new(k2));
//     tiles.push(Box::new(k3));
//     tiles.push(Box::new(d2));
//     Ok(tiles)
// }

// fn build_vertex6(x: f64, y: f64, angle: i32) -> Result<Vec<Box<dyn Drawable>>, i32> {
//     let phi = (1. + 5_f64.sqrt())/2.;

//     let d1 = Dart::new(x - phi, y, angle);
//     let k1 = place_kite_edge(4, d1.edge_center(2)?, d1.edge_angle(2)?);
//     let k2 = place_kite_edge(2, k1.edge_center(3)?, k1.edge_angle(3)?);
//     let k3 = place_kite_edge(4, k2.edge_center(1)?, k2.edge_angle(1)?);
//     let k4 = place_kite_edge(2, k3.edge_center(3)?, k3.edge_angle(3)?);

//     let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
//     tiles.push(Box::new(d1));
//     tiles.push(Box::new(k1));
//     tiles.push(Box::new(k2));
//     tiles.push(Box::new(k3));
//     tiles.push(Box::new(k4));
//     Ok(tiles)
// }

// fn build_vertex7(x: f64, y: f64, angle: i32) -> Result<Vec<Box<dyn Drawable>>, i32> {
//     let s5 = 5_f64.sqrt();
//     let phi = (1.+s5)/2.;
//     let k = (2.+s5) / (1.+s5);
//     let p = (10. + 20_f64.sqrt()).sqrt()/4.;

//     let k1 = Kite::new(x + k - 1., y - p, angle);
//     let k2 = place_kite_edge(3, k1.edge_center(2)?, k1.edge_angle(2)?);
//     let d1 = place_dart_edge(4, k2.edge_center(2)?, k2.edge_angle(2)?);
//     let d2 = place_dart_edge(2, d1.edge_center(3)?, d1.edge_angle(3)?);

//     let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
//     tiles.push(Box::new(k1));
//     tiles.push(Box::new(k2));
//     tiles.push(Box::new(d1));
//     tiles.push(Box::new(d2));
//     Ok(tiles)
// }

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    tiles: Vec<Box<dyn Drawable>>,
    edges: Vec<tile::Edge>,
    current_point: Point2,
    show_edges: bool,
    scale: f64,
    debug: bool,
    next_tile: tile::Tile,
    angle: i32,
}

fn snap_tolerance(scale: f64) -> f64 {
    return 15. / scale
}

fn model(app: &App) -> Model {
    app.set_exit_on_escape(false);
    app.new_window()
        .size(720, 720)
        .event(window_event)
        .raw_event(raw_window_event)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .mouse_moved(mouse_moved)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_wheel(mouse_wheel)
        .mouse_entered(mouse_entered)
        .mouse_exited(mouse_exited)
        .touch(touch)
        .touchpad_pressure(touchpad_pressure)
        .moved(window_moved)
        .resized(window_resized)
        .hovered_file(hovered_file)
        .hovered_file_cancelled(hovered_file_cancelled)
        .dropped_file(dropped_file)
        .focused(window_focused)
        .unfocused(window_unfocused)
        .closed(window_closed)
        .build()
        .unwrap();
    Model { tiles: Vec::new(),
            edges: Vec::new(),
            current_point: pt2(0.,0.),
            show_edges: true,
            scale: 25.,
            debug: false,
            next_tile: tile::Tile::UNREFLECTED,
            angle: 0,
    }
}

fn snap_to_edges(tile: &Box<dyn Drawable>, edges: &Vec<Edge>, tol: f64) -> (f64, f64) {
    let mut result = (0., 0.);
    let mut curr_l2 = tol;

    let e1 = tile.get_drawable_edges();
    for e in edges {

        for i in 0..13 {
            let dx = e.center.0 - e1[i].center.0;
            let dy = e.center.1 - e1[i].center.1;
            let l2 = dx*dx + dy*dy;
            if (l2 < curr_l2) {
                if ((e1[i].angle + 180) % 360 != e.angle) {
                    continue;
                }
                if (e1[i].length != e.length) {
                    continue;
                }

                result = (dx, dy);
                curr_l2 = l2;

//                println!("snap_to_edges {}", l2);
            }

        }
    }
    return result
}

fn snaps(edges: &Vec<tile::Edge>, tile: &Box<dyn Drawable>, tol: f64) -> bool {

    let de = tile.get_drawable_edges();

    for edge in edges {
        for i in 0..13 {
            if (edge.angle + 180)%360 != de[i].angle {
                continue;
            }
            if (edge.length != de[i].length) {
                continue;
            }

            let dx = edge.center.0 - de[i].center.0;
            let dy = edge.center.1 - de[i].center.1;
            let l2 = dx*dx + dy*dy;
            if (l2 < tol) {
//                println!("snaps true {}", l2);
                return true
            }
        }
    }
    return false
}

fn match_edges(t1: &Box<dyn Drawable>, t2: &Box<dyn Drawable>, skip: [bool; 13]) -> [bool; 13] {
    let mut result = skip;
    let e1 = t1.get_drawable_edges();
    let e2 = t2.get_drawable_edges();

    for i in 0..13 {
        if result[i] { continue }

        for j in 0..13 {
            let dx = e2[j].center.0 - e1[i].center.0;
            let dy = e2[j].center.1 - e1[i].center.1;
            let l2 = dx*dx + dy*dy;
            if (l2 > 0.1) {
                continue;
            }

            let a1 = (e1[i].angle + 180) % 360;
            let a2 = e2[j].angle;
            if (a1 != a2) {
                continue;
            }
            result[i] = true;
        }
    }
    return result;
}

fn add_tile(model: &mut Model, tile: Box<dyn Drawable>) {

    let offset = snap_to_edges(&tile, &model.edges.clone(), snap_tolerance(model.scale));
    tile.append_to_vector(&mut model.tiles, offset.0, offset.1);

    let mut new_edges = Vec::new();
    for t1 in &model.tiles {
        let mut matches = [false, false, false, false, false, false, false, false, false, false, false, false, false];
        for t2 in &model.tiles {
            // @todo don't need to check tile against itself
            // if (t1 == t2) {
            //     continue;
            // }
            matches = match_edges(&t1, &t2, matches);
        }

        let e = t1.get_drawable_edges();
        for i in 0..13 {
            if (!matches[i]) {
                new_edges.push(Edge {center: (e[i].center.0, e[i].center.1),
                                     angle: e[i].angle,
                                     length: e[i].length } );
            }
        }
    }
    model.edges = new_edges;
}

fn pop_last_tile(model: &mut Model) {

    model.tiles.pop();

    let mut new_edges = Vec::new();
    for t1 in &model.tiles {
        let mut matches = [false, false, false, false, false, false, false, false, false, false, false, false, false];
        for t2 in &model.tiles {
            // @todo don't need to check tile against itself
            // if (t1 == t2) {
            //     continue;
            // }
            matches = match_edges(&t1, &t2, matches);
        }

        let e = t1.get_drawable_edges();
        for i in 0..13 {
            if (!matches[i]) {
                new_edges.push(Edge {center: (e[i].center.0, e[i].center.1),
                                     angle: e[i].angle,
                                     length: e[i].length } );
            }
        }
    }
    model.edges = new_edges;
}

fn event(_app: &App, _model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            //raw: _,
            simple: _,
        } => {}
        Event::DeviceEvent(_device_id, _event) => {}
        Event::Update(_dt) => {}
        Event::Suspended => {}
        Event::Resumed => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {

    // Begin drawing
    let draw: nannou::draw::Draw = app.draw();

    // Clear the background to blue.
    draw.background().color(CORNFLOWERBLUE);

    let tile_props = DrawProps {
        fill_color1: LEMONCHIFFON,
        fill_color2: WHITESMOKE,
        edge_color: SIENNA,
        edge_weight: if model.show_edges { 2. } else { 0. },
    };

    let drag_props = DrawProps {
        fill_color1: GAINSBORO,
        fill_color2: GAINSBORO,
        edge_color: PINK,
        edge_weight: 0.,
    };

    let snap_props = DrawProps {
        fill_color1: LIGHTGREEN,
        fill_color2: LIGHTGREEN,
        edge_color: PINK,
        edge_weight: 0.,
    };

    // Draw the tiles
    for t in &model.tiles {
        t.draw(&draw, 0., 0., model.scale as f32, &tile_props);
    }

    // DEBUGGING: Draw the edges
    if (model.debug) {
        for e in &model.edges {
            let angle_in_radians = e.angle as f64 * std::f64::consts::PI / 180.0f64;
            let r = (model.scale as f64) * (if e.length == EdgeLength::SHORT { 1.0f64 } else { 1.6f64 });
            let v = Vector2::<f32>::new((r*angle_in_radians.cos()) as f32,
                                        (r*angle_in_radians.sin()) as f32);
            let cpt = pt2((e.center.0 * model.scale + 0.) as f32,
                          (e.center.1 * model.scale + 0.) as f32);
            let p1 = cpt - v;
            let p2 = cpt + v;
            draw.line().points(p1, p2)
                .color(BROWN)
                .weight(2.);
        }
    }

    // Draw currently dragged tile
    let x = model.current_point.x as f64 / model.scale;
    let y = model.current_point.y as f64 / model.scale;
    let tmp = build_tile(&model.next_tile, x, y, model.angle);
    match tmp {
        Ok(t) => {
            let props = if snaps(&model.edges, &t, snap_tolerance(model.scale)) { &snap_props } else { &drag_props };
            t.draw(&draw, 0., 0., model.scale as f32, props)
        },
        Err(_) => println!("Error drawing current tile"),
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

use nannou::event::*;

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            match key {
                // Key::Key1 => model.vertex_type = 1,
                // Key::Key2 => model.vertex_type = 2,
                // Key::Key3 => model.vertex_type = 3,
                // Key::Key4 => model.vertex_type = 4,
                // Key::Key5 => model.vertex_type = 5,
                // Key::Key6 => model.vertex_type = 6,
                // Key::Key7 => model.vertex_type = 7,
                Key::C => { model.tiles = Vec::new(); model.edges = Vec::new(); },
                Key::E => model.show_edges = !model.show_edges,
                Key::Up | Key::Down => model.next_tile = match model.next_tile {
                    tile::Tile::UNREFLECTED => tile::Tile::REFLECTED,
                    tile::Tile::REFLECTED => tile::Tile::UNREFLECTED,
                },
                Key::X => model.debug = !model.debug,
                Key::U => pop_last_tile(model),
                Key::Equals => { model.scale = 2.*model.scale.min(100.) },
                Key::Minus | Key::Underline => { model.scale = 0.5*model.scale.max(1.) },
                Key::Left => { model.angle = (model.angle + 30) % 360 },
                Key::Right => { model.angle = (model.angle + 360 - 30) % 360 },
                _ => println!("KeyPressed = {:?}", key),
            }
        }
        KeyReleased(_key) => {}
        MouseMoved(pos) => { model.current_point = pos }
        MousePressed(_button) => {
            let x = model.current_point.x as f64 / model.scale;
            let y = model.current_point.y as f64 / model.scale;
            // let res = match model.vertex_type {
            //     1 => build_vertex1(x, y, model.angle),
            //     2 => build_vertex2(x, y, model.angle),
            //     3 => build_vertex3(x, y, model.angle),
            //     4 => build_vertex4(x, y, model.angle),
            //     5 => build_vertex5(x, y, model.angle),
            //     6 => build_vertex6(x, y, model.angle),
            //     7 => build_vertex7(x, y, model.angle),
            //     _ => Err(666),
            // };
            // match res {
            //     Ok(t) => for o in t { model.tiles.push(o) },
            //     Err(_) => println!("Error building vertex 2"),
            // }
            let res = build_tile(&model.next_tile, x, y, model.angle);
            match res {
                Ok(t) => add_tile(model, t),
                Err(_) => println!("Error building vertex 2"),
            }
        }
        MouseReleased(_button) => {}
        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_amount, _phase) => {}
        Moved(_pos) => {}
        Resized(_size) => {}
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

fn raw_window_event(_app: &App, _model: &mut Model, _event: &nannou::winit::event::WindowEvent) {}

fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {}

fn key_released(_app: &App, _model: &mut Model, _key: Key) {}

fn mouse_moved(_app: &App, _model: &mut Model, _pos: Point2) {}

fn mouse_pressed(_app: &App, _model: &mut Model, _button: MouseButton) {}

fn mouse_released(_app: &App, _model: &mut Model, _button: MouseButton) {}

fn mouse_wheel(_app: &App, _model: &mut Model, _dt: MouseScrollDelta, _phase: TouchPhase) {}

fn mouse_entered(_app: &App, _model: &mut Model) {}

fn mouse_exited(_app: &App, _model: &mut Model) {}

fn touch(_app: &App, _model: &mut Model, _touch: TouchEvent) {}

fn touchpad_pressure(_app: &App, _model: &mut Model, _pressure: TouchpadPressure) {}

fn window_moved(_app: &App, _model: &mut Model, _pos: Point2) {}

fn window_resized(_app: &App, _model: &mut Model, _dim: Vector2) {}

fn window_focused(_app: &App, _model: &mut Model) {}

fn window_unfocused(_app: &App, _model: &mut Model) {}

fn window_closed(_app: &App, _model: &mut Model) {}

fn hovered_file(_app: &App, _model: &mut Model, _path: std::path::PathBuf) {}

fn hovered_file_cancelled(_app: &App, _model: &mut Model) {}

fn dropped_file(_app: &App, _model: &mut Model, _path: std::path::PathBuf) {}
