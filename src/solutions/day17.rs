use itertools::Itertools;

struct Target {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

fn check_if_vel_hits(vel: &(isize, isize), target: &Target) -> Option<isize> {
    let (mut vel_x, mut vel_y) = vel.clone();
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut max_y_reached = 0;

    loop {
        if (target.x_min..=target.x_max).contains(&pos_x)
            && (target.y_max..=target.y_min).contains(&pos_y)
        {
            return Some(max_y_reached);
        }

        if pos_x > target.x_max || pos_y < target.y_max {
            return None;
        }

        // step
        pos_x += vel_x;
        pos_y += vel_y;
        if vel_x > 0 {
            vel_x -= 1
        };
        vel_y -= 1;

        if pos_y > max_y_reached {
            max_y_reached = pos_y;
        }
    }
}

fn part_one(target: &Target) {
    let highest_y = (0..1000)
        .cartesian_product(0..1000)
        .filter_map(|vel| check_if_vel_hits(&vel, target))
        .max()
        .unwrap();
    println!("part 1: {}", highest_y);
}

fn part_two(target: &Target) {
    let count = (0..1000)
        .cartesian_product(-1000..1000)
        .filter_map(|vel| check_if_vel_hits(&vel, target))
        .count();
    println!("{}", count);
}

pub fn run() {
    let target = Target {
        x_min: 287,
        x_max: 309,
        y_min: -48,
        y_max: -76,
    };
    part_one(&target);
    part_two(&target);
}
