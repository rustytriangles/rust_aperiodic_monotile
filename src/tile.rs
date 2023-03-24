use crate::geom::{Point2, pt2, Vector2, vec2, Polygon};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum EdgeLength {
    SHORT,
    LONG,
    DOUBLE // 2 x SHORT
}

#[derive(Clone)]
pub struct Edge {
    pub center: (f64, f64),
    pub angle: i32,
    pub length: EdgeLength,
}

fn edge_index_to_vertex_tuple(e: i32) -> Result<(usize, usize), i32> {
    match e {
        1  => Ok(( 0 as usize, 1 as usize)),
        2  => Ok(( 1 as usize, 2 as usize)),
        3  => Ok(( 2 as usize, 3 as usize)),
        4  => Ok(( 3 as usize, 4 as usize)),
        5  => Ok(( 4 as usize, 5 as usize)),
        6  => Ok(( 5 as usize, 6 as usize)),
        7  => Ok(( 6 as usize, 7 as usize)),
        8  => Ok(( 7 as usize, 8 as usize)),
        9  => Ok(( 8 as usize, 9 as usize)),
        10 => Ok(( 9 as usize,10 as usize)),
        11 => Ok((10 as usize,11 as usize)),
        12 => Ok((11 as usize,12 as usize)),
        13 => Ok((12 as usize, 0 as usize)),
        _ => Err(e),
    }
}

fn tile_geom() -> [[f64; 2]; 13] {
    let s3 = 3_f64.sqrt();
    let r = s3 / 2.;

    [
        [0.5, 0.],
        [0.5, r],
        [-0.25, 3.*r/2.],
        [-0.5,r],
        [-1.,r],
        [-1.,0.],
        [-1.75,-r/2.],
        [-1.5,-r],
        [-0.5,-r],
        [-0.25,-r/2.],
        [0.5,-r],
        [1.25,-r/2.],
        [1.,0.],
    ]
}

pub enum Tile {
    UNREFLECTED,
    REFLECTED,
}

pub struct Unreflected {
    pub cx: f64,
    pub cy: f64,
    pub angle: i32,
}

impl Unreflected {
    pub fn new(x: f64, y: f64, a: i32) -> Self {
        Self {
            cx: x,
            cy: y,
            angle: (a+360)%360
        }
    }

    pub fn rotate(&self, angle: i32) -> Unreflected {
        Unreflected{ cx: self.cx, cy: self.cy, angle: (self.angle + angle)%360 }
    }

    pub fn translate(&self, ox: f64, oy: f64) -> Unreflected {
        Unreflected{ cx: (self.cx + ox), cy: (self.cy + oy), angle: self.angle }
    }

    pub fn polygon(&self, xoff: f32, yoff: f32, scale: f32) -> Vec<(f32,f32)> {
        let pts = self.geometry();

        let xoff64 = xoff as f64;
        let yoff64 = yoff as f64;
        let scale64 = scale as f64;

        vec![( (pts[2* 0+0]*scale64 + xoff64) as f32, (pts[2* 0+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 1+0]*scale64 + xoff64) as f32, (pts[2* 1+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 2+0]*scale64 + xoff64) as f32, (pts[2* 2+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 3+0]*scale64 + xoff64) as f32, (pts[2* 3+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 4+0]*scale64 + xoff64) as f32, (pts[2* 4+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 5+0]*scale64 + xoff64) as f32, (pts[2* 5+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 6+0]*scale64 + xoff64) as f32, (pts[2* 6+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 7+0]*scale64 + xoff64) as f32, (pts[2* 7+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 8+0]*scale64 + xoff64) as f32, (pts[2* 8+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 9+0]*scale64 + xoff64) as f32, (pts[2* 9+1]*scale64 + yoff64) as f32 ),
             ( (pts[2*10+0]*scale64 + xoff64) as f32, (pts[2*10+1]*scale64 + yoff64) as f32 ),
             ( (pts[2*11+0]*scale64 + xoff64) as f32, (pts[2*11+1]*scale64 + yoff64) as f32 ),
             ( (pts[2*12+0]*scale64 + xoff64) as f32, (pts[2*12+1]*scale64 + yoff64) as f32 ),


        ]
    }

    pub fn edge_angle(&self, e: i32) -> Result<i32, i32> {
        const EA: &'static [i32] = &[90, 150, 240, 180, 270, 210, 300, 0, 60, 330, 30, 120, 180];

        match e {
            1..=13 => Ok((EA[(e-1) as usize] + self.angle)%360),
            _ => Err(e),
        }
    }

    pub fn edge_length(&self, e: i32) -> Result<EdgeLength, i32> {
        const EL: &'static [EdgeLength] = &[EdgeLength::LONG,
                                            EdgeLength::LONG,
                                            EdgeLength::SHORT,
                                            EdgeLength::SHORT,
                                            EdgeLength::LONG,
                                            EdgeLength::LONG,
                                            EdgeLength::SHORT,
                                            EdgeLength::DOUBLE,
                                            EdgeLength::SHORT,
                                            EdgeLength::LONG,
                                            EdgeLength::LONG,
                                            EdgeLength::SHORT,
                                            EdgeLength::SHORT];
        match e {
            1..=13 => Ok(EL[(e-1) as usize]),
            _ => Err(e),
        }
    }

    pub fn edge_center(&self, e: i32) -> Result<(f64, f64), i32> {
        let (i1, i2) = edge_index_to_vertex_tuple(e)?;
        let pts = self.geometry();
        let x1 = pts[2*i1+0];
        let y1 = pts[2*i1+1];
        let x2 = pts[2*i2+0];
        let y2 = pts[2*i2+1];
        Ok(( (x1+x2)/2., (y1+y2)/2. ))
    }

    pub fn edge_points(&self, e: i32) -> Result<((f64, f64), (f64, f64)), i32> {
        let (i1, i2) = edge_index_to_vertex_tuple(e)?;
        let pts = self.geometry();
        let x1 = pts[2*i1+0];
        let y1 = pts[2*i1+1];
        let x2 = pts[2*i2+0];
        let y2 = pts[2*i2+1];
        Ok(( (x1,y1), (x2,y2) ))
    }

    fn geometry(&self) ->  Box<[f64]> {
        let angle_in_radians = self.angle as f64 * std::f64::consts::PI / 180.;
        let c = angle_in_radians.cos();
        let s = angle_in_radians.sin();
//        let s3 = 3_f64.sqrt();
//        let r = s3 / 2.;

        let mut boxed_arr = Box::new([0.; 26]);
        let tg = tile_geom();
        for i in 0..13 {
            boxed_arr[2* i+0] = self.cx + c*(tg[ i][0]) - s*(tg[ i][1]);
            boxed_arr[2* i+1] = self.cy + s*(tg[ i][0]) + c*(tg[ i][1]);
        }

        boxed_arr
    }

    pub fn get_edges(&self) -> Vec<Edge> {
        let mut result = Vec::new();
        for i in 1..14 {
            let mut e = Edge { center: (0., 0.), angle: 0, length: EdgeLength::SHORT };
            match self.edge_center(i) {
                Ok(c) => e.center = c,
                Err(_) => continue,
            }
            match self.edge_angle(i) {
                Ok(a) => e.angle = a,
                Err(_) => continue,
            }
            match self.edge_length(i) {
                Ok(l) => e.length = l,
                Err(_) => continue,
            }
            result.push(e);
        }
        result
    }

}

pub fn place_unreflected_edge(e: i32, pt: (f64,f64), edge_angle: i32) -> Unreflected {
    let s5 = 5_f64.sqrt();
    let phi = (1.+s5)/2.;
    let h = (5.+2.*s5).sqrt()/2.;

    match e {
        1 => {
            let new_angle = (edge_angle + 360 - 252) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = 0.25;
            let dy = h/2.;
            Unreflected::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        2 => {
            let new_angle = (edge_angle + 144) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = 0.25 - phi/2.;
            let dy = h/2.;
            Unreflected::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        3 => {
            let new_angle = (edge_angle + 36) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = 0.25 - phi/2.;
            let dy = -h/2.;
            Unreflected::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        4 => {
            let new_angle = (edge_angle + 252) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = 0.25;
            let dy = -h/2.;
            Unreflected::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        _ => {
            Unreflected::new(0.,0.,0)
        }
    }
}

////////////////////////////////////////////////////////////////////////


pub struct Reflected {
    pub cx: f64,
    pub cy: f64,
    pub angle: i32,
}

impl Reflected {
    pub fn new(x: f64, y: f64, a: i32) -> Self {
        Self {
            cx: x,
            cy: y,
            angle: (a+360)%360
        }
    }

    pub fn rotate(&self, angle: i32) -> Reflected {
        Reflected{ cx: self.cx, cy: self.cy, angle: (self.angle + angle)%360 }
    }

    pub fn translate(&self, ox: f64, oy: f64) -> Reflected {
        Reflected{ cx: (self.cx + ox), cy: (self.cy + oy), angle: self.angle }
    }

    pub fn polygon(&self, xoff: f32, yoff: f32, scale: f32) -> Vec<(f32,f32)> {
        let pts = self.geometry();

        let xoff64 = xoff as f64;
        let yoff64 = yoff as f64;
        let scale64 = scale as f64;

        vec![( (pts[2* 0+0]*scale64 + xoff64) as f32, (pts[2* 0+1]*scale64 + yoff64) as f32 ),
             ( (pts[2*12+0]*scale64 + xoff64) as f32, (pts[2*12+1]*scale64 + yoff64) as f32 ),
             ( (pts[2*11+0]*scale64 + xoff64) as f32, (pts[2*11+1]*scale64 + yoff64) as f32 ),
             ( (pts[2*10+0]*scale64 + xoff64) as f32, (pts[2*10+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 9+0]*scale64 + xoff64) as f32, (pts[2* 9+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 8+0]*scale64 + xoff64) as f32, (pts[2* 8+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 7+0]*scale64 + xoff64) as f32, (pts[2* 7+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 6+0]*scale64 + xoff64) as f32, (pts[2* 6+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 5+0]*scale64 + xoff64) as f32, (pts[2* 5+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 4+0]*scale64 + xoff64) as f32, (pts[2* 4+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 3+0]*scale64 + xoff64) as f32, (pts[2* 3+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 2+0]*scale64 + xoff64) as f32, (pts[2* 2+1]*scale64 + yoff64) as f32 ),
             ( (pts[2* 1+0]*scale64 + xoff64) as f32, (pts[2* 1+1]*scale64 + yoff64) as f32 ),


        ]
    }

    pub fn edge_angle(&self, e: i32) -> Result<i32, i32> {
        const EA: &'static [i32] = &[180, 240, 330, 30, 300, 360, 60, 150, 90, 180, 120, 210, 270];

        match e {
            1..=13 => Ok((EA[(e-1) as usize] + self.angle)%360),
            _ => Err(e),
        }
    }

    pub fn edge_length(&self, e: i32) -> Result<EdgeLength, i32> {
        const EL: &'static [EdgeLength] = &[EdgeLength::SHORT,
                                            EdgeLength::SHORT,
                                            EdgeLength::LONG,
                                            EdgeLength::LONG,
                                            EdgeLength::SHORT,
                                            EdgeLength::DOUBLE,
                                            EdgeLength::SHORT,
                                            EdgeLength::LONG,
                                            EdgeLength::LONG,
                                            EdgeLength::SHORT,
                                            EdgeLength::SHORT,
                                            EdgeLength::LONG,
                                            EdgeLength::LONG];
        match e {
            1..=13 => Ok(EL[(e-1) as usize]),
            _ => Err(e),
        }
    }

    pub fn edge_center(&self, e: i32) -> Result<(f64, f64), i32> {
        let (i1, i2) = edge_index_to_vertex_tuple(14 - e)?;
        let pts = self.geometry();
        let x1 = pts[2*i1+0];
        let y1 = pts[2*i1+1];
        let x2 = pts[2*i2+0];
        let y2 = pts[2*i2+1];
        Ok(( (x1+x2)/2., (y1+y2)/2. ))
    }

    pub fn edge_points(&self, e: i32) -> Result<((f64, f64), (f64, f64)), i32> {
        let (i1, i2) = edge_index_to_vertex_tuple(14 - e)?;
        let pts = self.geometry();
        let x1 = pts[2*i1+0];
        let y1 = pts[2*i1+1];
        let x2 = pts[2*i2+0];
        let y2 = pts[2*i2+1];
        Ok(( (x1,y1), (x2,y2) ))
    }

    fn geometry(&self) ->  Box<[f64]> {
        let angle_in_radians = self.angle as f64 * std::f64::consts::PI / 180.;
        let c = angle_in_radians.cos();
        let s = angle_in_radians.sin();
//        let s3 = 3_f64.sqrt();
//        let r = s3 / 2.;

        let mut boxed_arr = Box::new([0.; 26]);
        let tg = tile_geom();
        for i in 0..13 {
            boxed_arr[2* i+0] = self.cx + c*(-tg[ i][0]) - s*(tg[ i][1]);
            boxed_arr[2* i+1] = self.cy + s*(-tg[ i][0]) + c*(tg[ i][1]);
        }

        boxed_arr
    }

    pub fn get_edges(&self) -> Vec<Edge> {
        let mut result = Vec::new();
        for i in 1..14 {
            let mut e = Edge { center: (0., 0.), angle: 0, length: EdgeLength::SHORT };
            match self.edge_center(i) {
                Ok(c) => e.center = c,
                Err(_) => continue,
            }
            match self.edge_angle(i) {
                Ok(a) => e.angle = a,
                Err(_) => continue,
            }
            match self.edge_length(i) {
                Ok(l) => e.length = l,
                Err(_) => continue,
            }
            result.push(e);
        }
        result
    }

}

pub fn place_reflected_edge(e: i32, pt: (f64,f64), edge_angle: i32) -> Reflected {
    let s5 = 5_f64.sqrt();
    let phi = (1.+s5)/2.;
    let h = (5.+2.*s5).sqrt()/2.;

    match e {
        1 => {
            let new_angle = (edge_angle + 360 - 252) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = 0.25;
            let dy = h/2.;
            Reflected::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        2 => {
            let new_angle = (edge_angle + 144) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = 0.25 - phi/2.;
            let dy = h/2.;
            Reflected::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        3 => {
            let new_angle = (edge_angle + 36) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = 0.25 - phi/2.;
            let dy = -h/2.;
            Reflected::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        4 => {
            let new_angle = (edge_angle + 252) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = 0.25;
            let dy = -h/2.;
            Reflected::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        _ => {
            Reflected::new(0.,0.,0)
        }
    }
}

