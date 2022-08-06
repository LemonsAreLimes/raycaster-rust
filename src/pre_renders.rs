#[path="./triangulation.rs"]
mod triangulation;

pub fn reset_angle(mut rot:f64) -> f64{
    if rot >= 360.0 {rot = 0.0}
    else if rot <= 0.0 {rot = 360.0}

    return rot;

}

pub fn movement(key:&[i32;2], mut pos:[f64;2], rot:f64) -> [f64;2]{
    let speed = 0.01;

    let x_0 = ((rot+90.0).to_radians().cos() * speed).to_degrees();
    let y_0 = ((rot+90.0).to_radians().sin() * speed).to_degrees();

    let x_45 = ((rot+135.0).to_radians().cos() * speed).to_degrees();
    let y_45 = ((rot+135.0).to_radians().sin() * speed).to_degrees();

    if key == &[0,1]        {pos[0] += x_0; pos[1] += y_0;}   //forward
    else if key == &[0,0]   {pos[0] -= x_0; pos[1] -= y_0;}   //backward
    else if key == &[1,1]   {pos[0] += x_45; pos[1] += y_45;} //left
    else if key == &[1,0]   {pos[0] -= x_45; pos[1] -= y_45;} //right

    return pos;

}

pub fn prerender_objects(y:f64, pos:[f64; 2], rot:f64, map:&Vec<[[i32;2];2]> ) -> Vec<[[f64;2];3]> {

    println!("{}", rot);

    let baseline = y;
    let render_cutoff = 100.0;

    //parse out lines
    let mut polys:Vec<[[f64;2];3]> = Vec::new();
    for line in map {

        //parse line
        let parsed_line = triangulation::convert_line(&pos, &render_cutoff, &line);

        let p1 = parsed_line[0];
        let p2 = parsed_line[1];

        // //check if in angle view
        let mut dont_render = false;
        // for a in (rot-45.0) as i32..(rot+45.0) as i32{
        //     if p1[0] as i32 == a || p2[0] as i32  == a {dont_render = true}
        // }

        if dont_render == false {
            let a =     [p1[0], p1[1] + baseline];
            let b =     [p2[0], p2[1] + baseline];

            let c =     [p1[0], baseline - p1[1]];
            let d =     [p2[0], baseline - p2[1]];

            let polydata1 = [a,b,d];
            let polydata2 = [a,c,d];

            polys.push(polydata1);
            polys.push(polydata2);
        }
    }
    return polys;
}


pub fn prerender_points(y:f64, pos:[f64; 2], rot:f64, map:&Vec<[[i32;2];2]>) -> Vec<[f64;4]> {

    //rendering stuff
    let line_width = 15.0;
    let baseline = y;
    let render_cutoff = 100.0;

    //parse out points
    let mut shapes:Vec<[f64;4]> = Vec::new();
    for line in map {

        //parse line
        let parsed_line = triangulation::convert_line(&pos, &render_cutoff, &line);

        //render it
        for cord in parsed_line {
            shapes.push([cord[0], baseline-((cord[1]*10.0)/2.0), line_width, cord[1]*10.0],)
        }
    }

    return shapes;
}
