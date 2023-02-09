use std::{ops, f32::consts::PI};
// use rand::Rng;
// use rand::prelude::random;

use image::{ ImageBuffer, Pixel };

#[derive( Clone, Copy )]
struct Vec2 {
    x: f32,
    y: f32,
}
impl Vec2 {
    fn distance (self, p: Vec2) -> f32 {
        return (((self.x - p.x).powf(2.0) + (self.y - p.y).powf(2.0))).sqrt();
    }
    fn distance_from_line (self, p1: Vec2, p2: Vec2) -> f32 {
        return ((p2.x - p1.x)*(p1.y - self.y) - (p1.x - self.x)*(p2.y - p1.y)).abs() / p1.distance(p2);
    }
    fn rotate (self, theta: f32) -> Vec2 {
        return Vec2 { x: self.x * theta.cos() - self.y * theta.sin(), 
                      y: self.x * theta.sin() + self.y * theta.cos() }
    }
}
impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add (self, v: Vec2) -> Vec2 {
        return Vec2 {x: self.x + v.x, y: self.y + v.y};
    }
}
impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub (self, v: Vec2) -> Vec2 {
        return Vec2 {x: self.x - v.x, y: self.y - v.y};
    }
}
impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul (self, s: f32) -> Vec2 {
        return Vec2 {x: self.x * s, y: (self.y * s)};
    }
}

struct Leaf {
    center: Vec2,
    radius: f32,
}
impl Leaf {
    fn shade (&self, pos: Vec2) -> u8 {
        
        let d = self.center.distance(pos);

        if d > self.radius + 2.0 {
            return 0;
        } else if d < self.radius {
            return 255;
        }
        else {
            let x = (d - self.radius) / 2.0;
            return 255 - (255.0 * x * x * (3.0 - 2.0 * x)) as u8 
        }
    }
}

struct Tree {
    base: Vec2,
    top: Vec2,

    length: f32,
    base_width: f32,
    top_width: f32,

    branches: Vec<Tree>,

}
impl Tree {
    fn distance_from_branch (&self, pos: Vec2) -> f32 {
        return if pos.distance(self.base) < self.base.distance(self.top) && 
                  pos.distance(self.top) < self.top.distance(self.base) {
            pos.distance_from_line(self.base, self.top)
        } else {
            pos.distance(self.base).min(pos.distance(self.top))
        };
    }

    fn shade (&self, pos: Vec2) -> u8 {

        let mut d = self.distance_from_branch(pos);
        let br;
        (d, br) = self.get_closest_branch_distance(pos, d, self);

        let e1: f32 = 
            br.top_width + (pos.distance(br.top) / br.length) * (br.base_width - br.top_width);
        if d > e1 + 2.0 {
            return 0;
        } else if d < e1 {
            return 255;
        } else {
            let x = (d - e1) / 2.0;
            return 255 - (255.0 * x * x * (3.0 - 2.0 * x)) as u8 
        }
    }

    fn get_closest_branch_distance<'a> (&'a self, pos: Vec2, mut d: f32, br: &'a Tree) -> (f32, &Tree) {
        
        let mut br1 = br;

        for branch in &self.branches {
            let d1 = branch.distance_from_branch(pos);
            if d1 < d {
                d = d1;
                br1 = branch;
            }
            (d, br1) = branch.get_closest_branch_distance(pos, d, br1);
        }

        return (d, br1);
    }

    fn create_branches (&mut self, leaves: &mut Vec<Leaf>, depth: u8) {
        if depth <= 0 {
            return
        }

        let n = rand::random::<u32>() % 1 + 2;
        let branch_angle = PI * 0.15 * n as f32 * (rand::random::<f32>() * 0.5 + 0.75);
        for i in 0..n {
            let base = self.top;

            let theta: f32 = ((i as f32 / (n - 1) as f32) * branch_angle) - (branch_angle * 0.5);

            let top = base + ((self.top - self.base) * 0.7).rotate(
                theta * (rand::random::<f32>() * 0.5 + 0.75)
            );  //  * (rand::random::<f32>() * 0.5 + 0.75)
            let length = base.distance(top);

            if depth <= 2 {
                leaves.push(Leaf {
                    center: top,
                    radius: rand::random::<f32>() * 3.0 + 2.0
                })
            }

            let mut branch = Tree {
                base, top, length,
                base_width: self.top_width, top_width: self.top_width * 0.6,
                branches: vec![]
            };
            branch.create_branches(leaves, depth - 1);
            self.branches.push(branch);
        }
    }
}

const DIMS: Vec2 = Vec2 {x: 512.0, y: 512.0};
const CENTER: Vec2 = Vec2 {x: DIMS.x * 0.5, y: DIMS.y * 0.5};


fn get_color (pos: Vec2, tree: &Tree, leaves: &Vec<Leaf>) -> image::Rgba<u8> {

    let max_radius = DIMS.x as f32 * 0.6;
    let pos_radius = CENTER.distance(pos);

    let scale: u8 = 255 - (((pos_radius / max_radius) as f32 * 255.0) as u8).clamp(0, 255); 
    // let mut color = image::Rgba([48 - scale.clamp(0, 32), (scale / 2) + (0 - scale.clamp(0, 0)), scale, 255]);
    let mut color = image::Rgba([scale, scale, scale, 255]);

    color.blend(&image::Rgba([0, 0, 0, tree.shade(pos)]));
    for leaf in leaves {
        color.blend(&image::Rgba([255, 183, 197, leaf.shade(pos)]));
    }

    return color;

}

fn main() {

    const BASE: Vec2  = Vec2 {x: CENTER.x, y: 467.0};
    const TOP: Vec2   = Vec2 {x: CENTER.x, y: CENTER.y + 70.0};

    let mut tree = Tree {
        base: BASE, top: TOP,
        length: BASE.distance(TOP),
        base_width: 10.0, top_width: 6.0,
        branches: vec![],
    };
    let mut leaves: Vec<Leaf> = vec![];

    tree.create_branches(&mut leaves, 8);

    let img = ImageBuffer::from_fn(DIMS.x as u32, DIMS.y as u32,
        | x, y | {get_color(Vec2 {x: x as f32, y: y as f32}, &tree, &leaves)});
    img.save("bin/image.png").unwrap();

}
