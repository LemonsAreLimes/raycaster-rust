



pub fn convert_line(cam: &[i32; 2], line: &[[i32;2];2]) -> [[f64;2];2]{

    let line0 = triangulate_point(&cam, &line[0]);
    let line1 = triangulate_point(&cam, &line[1]);

    let parsed_line: [[f64;2];2] = [line0, line1];
    return parsed_line;
}

pub fn triangulate_point(cam:&[i32; 2], point:&[i32;2]) -> [f64; 2]{
    let mut _adj: f64;
    let mut _opp: f64;

    //parse adj so it does not give a negative value
    if cam[0] > point[0]{
        _adj = (cam[0] - point[0]) as f64;
    } else {
        _adj = (point[0] - cam[0]) as f64;
    }

    //parse hyp
    if cam[1] > point[1]{
        _opp = (cam[1] - point[1]) as f64;
    } else {
        _opp = (point[1] - cam[1]) as f64;
    }

    //find angle
    let angle = f64::atan2(_opp,_adj);

    //find hyp
    let a_sqr = i32::pow(_adj as i32 , 2);
    let b_sqr = i32::pow(_opp as i32, 2);
    let c = (a_sqr + b_sqr) as f64;
    let hyp = f64::sqrt(c);

    //return both
    return [angle.to_degrees(), hyp];
}