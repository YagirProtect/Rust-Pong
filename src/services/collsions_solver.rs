use vek::Vec2;
use crate::classes::t_entity::Entity;

pub fn solve_collisions_clear(renderers: &mut Vec<Box<dyn Entity>>) {
    for (i, e) in renderers.iter_mut().enumerate() {
        if e.has_collision() {
            let mut col = match e.get_collision(){
                Some(col) => col,
                None => continue,
            };

            col.set_intersection(false, Vec2::zero(), Vec2::zero(), 0.0);
        }
    }
}

pub fn solve_collisions(renderers: &mut Vec<Box<dyn Entity>>){


    let mut collidable: Vec<usize> = Vec::new();

    for (i, e) in renderers.iter_mut().enumerate() {
        if e.has_collision() {
            let mut col = match e.get_collision(){
                Some(col) => col,
                None => continue,
            };
            col.set_intersection(false, Vec2::zero(), col.velocity(), 0.0);
            collidable.push(i);
        }
    }


    for i in 0..collidable.len() {
        for j in (i + 1)..collidable.len() {
            let a = collidable[i];
            let b = collidable[j];

            let (left, right) = renderers.split_at_mut(b);
            let ea = &mut left[a];
            let eb = &mut right[0];


            let colA = ea.get_collision().unwrap();
            let colB = eb.get_collision().unwrap();


            let boundsA = colA.get_bounds();
            let boundsB = colB.get_bounds();

            let (a_min, a_max) = boundsA;
            let (b_min, b_max) = boundsB;

            if (colA.check_intersection(boundsB)){
                // println!("A min={:?} max={:?} | B min={:?} max={:?}", a_min, a_max, b_min, b_max);

                if let Some((normal_a_to_b, _pen)) = aabb_normal_and_penetration(a_min, a_max, b_min, b_max) {

                    let normal_for_a = -normal_a_to_b;
                    let normal_for_b =  normal_a_to_b;

                    let velA = colA.velocity();
                    let velB = colB.velocity();

                    colA.set_intersection(true, normal_for_a, velB, _pen);
                    colB.set_intersection(true, normal_for_b, velA, _pen);
                }
            }
        }
    }
}

pub fn aabb_normal_and_penetration(a_min: Vec2<f32>, a_max: Vec2<f32>, b_min: Vec2<f32>, b_max: Vec2<f32>)-> Option<(Vec2<f32>, f32)> {
    // centers
    let a_c = (a_min + a_max) * 0.5;
    let b_c = (b_min + b_max) * 0.5;

    // halfs
    let a_e = (a_max - a_min) * 0.5;
    let b_e = (b_max - b_min) * 0.5;

    let d = b_c - a_c;               // dir from A to B
    let ox = a_e.x + b_e.x - d.x.abs(); // overlap X
    let oy = a_e.y + b_e.y - d.y.abs(); // overlap Y

    if ox <= 0.0 || oy <= 0.0 {
        return None; // does not have a penetration
    }

    // minimal axis
    if ox < oy {
        let nx = if d.x >= 0.0 { 1.0 } else { -1.0 };
        Some((Vec2::new(nx, 0.0), ox))
    } else {
        let ny = if d.y >= 0.0 { 1.0 } else { -1.0 };
        Some((Vec2::new(0.0, ny), oy))
    }
}