#[path="./triangulation.rs"]
mod triangulation;

pub fn prerender_objects(y:f64, pos:[f64; 2], rot:f64, map:&Vec<[[i32;2];2]> ) -> Vec<[[f64;2];3]> {

    let baseline = y;
    let render_cutoff = 100.0;

    //parse out lines
    let mut polys:Vec<[[f64;2];3]> = Vec::new();
    for line in map {

        //parse line
        let parsed_line = triangulation::convert_line(&pos, &render_cutoff, &line);

        let p1 = parsed_line[0];
        let p2 = parsed_line[1];

        let a =     [p1[0], p1[1] + baseline];
        let b =     [p2[0], p2[1] + baseline];

        let c =     [p1[0], baseline - p1[1]];
        let d =     [p2[0], baseline - p2[1]];

        let polydata1 = [a,b,d];
        let polydata2 = [a,c,d];

        polys.push(polydata1);
        polys.push(polydata2);
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
