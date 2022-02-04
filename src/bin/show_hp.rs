extern crate healpix2mesh;

use std::{
    rc::Rc
    , cell::RefCell
    , thread::sleep
    , time::Duration
    , path::Path
};

use healpix2mesh::{
    hp2vertices
};

use clap::{
    App, Arg
};

use healpix_fits::{
    read_map
};

use kiss3d::{
    nalgebra::{
        Point3
        , Point2
        , Vector3
        , UnitQuaternion
    }
    , window::Window
    , light::Light
    , resource::Mesh
    , camera::ArcBall
    , builtin::NormalsMaterial
    , text::Font
};

const pi_f32:f32=std::f32::consts::PI;

fn main(){
    let matches=App::new("show hp")
    .arg(
        Arg::new("input")
        .short('i')
        .long("infile")
        .required(true)
        .takes_value(true)
        .value_name("healpix file")
        .required(true)
    )
    .arg(
        Arg::new("grid_iter")
        .short('n')
        .long("grid")
        .required(true)
        .takes_value(true)
        .value_name("grid num iter")
        .default_missing_value("5")

    )
    .arg(
        Arg::new("ground")
        .short('g')
        .long("ground")
        .takes_value(false)
        .required(false)
    )
    .get_matches();

    let with_ground=matches.occurrences_of("ground")>0;

    let hpdata=read_map::<f64>(matches.value_of("input").unwrap(), &["TEMPERATURE"], 1).pop().unwrap();
    let grid_niter=matches.value_of("grid_iter").unwrap().parse::<usize>().unwrap();

    let max_value=hpdata.iter().cloned().reduce(|a,b|{
        if a>b{a}else{b}
    }).unwrap();
    println!("max: {}", max_value);
    let tes=hp2vertices(&hpdata, grid_niter, &|x| x/max_value);

    let points:Vec<_>=tes.vertices.iter().map(|&p|{
        Point3::new(p.x as f32, p.y as f32, p.z as f32)}).collect();
    let faces:Vec<Point3::<u16>>=tes.faces.iter().map(|p|{Point3::new(p[0] as u16, p[1] as u16, p[2] as u16)}).collect();

    let mut window=Window::new_with_size("test", 1024, 768);
    let mut camera=ArcBall::new(Point3::new(1.5, 2., 4.0), Point3::new(0.0, 0.0, 0.0));
    let mesh=Mesh::new(points, faces, None, None, true);
    

    let mut beam=window.add_mesh(Rc::new(RefCell::new(mesh)), Vector3::new(1.0, 1.0, 1.0));
    beam.enable_backface_culling(false);




    let rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -pi_f32/2.0);
    let rotz = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), pi_f32/4.0);


    if with_ground{
        let ground=Mesh::new(vec![
            Point3::new(1.0, -1.0, -0.01),
            Point3::new(1.0, 1.0, -0.01),
            Point3::new(-1.0, 1.0, -0.01),
            Point3::new(-1.0, -1.0, -0.01)
        ],vec![Point3::new(0,1,2),
        Point3::new(2,3,0)],None, None, true);
        let mut ground=window.add_mesh(Rc::new(RefCell::new(ground)), Vector3::new(1.0, 1.0, 1.0));
        ground.set_color(0.5, 0.5, 0.5);
        ground.enable_backface_culling(false);
    
        ground.prepend_to_local_rotation(&rot);
        ground.prepend_to_local_rotation(&rotz);    
    }


    beam.prepend_to_local_rotation(&rot);
    beam.prepend_to_local_rotation(&rotz);
    //beam.set_material(Rc::new(RefCell::new(Box::new(NormalsMaterial::new()))));
    beam.set_color(0.0, 1.0, 0.0);
    window.set_light(Light::StickToCamera);
    beam.set_points_size(0.0);
    beam.set_lines_width(1.0);
    beam.set_surface_rendering_activation(false);

    //let font=Font::new(&Path::new("./FandolHei-Bold.otf")).unwrap();
    let font = Font::default();

    //window.draw_text(&freq_label, &Point2::new(0.0, 0.0), 1.0_f32, &font, &Point3::new(1.0, 1.0, 1.0));

    //window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(1.0, 0.0, 0.0), &Point3::new(1.0, 0.0, 0.0));
    //window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, 1.0, 0.0), &Point3::new(0.0, 1.0, 0.0));
    //window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, 0.0, 1.0), &Point3::new(0.0, 0.0, 1.0));



    while window.render_with_camera(&mut camera){
        //sleep(Duration::from_millis(100));
        //let img = window.snap_image();
        //let img_path = Path::new("screenshot.png");
        //img.save(img_path).unwrap();
        //println!("Screeshot saved to `screenshot.png`");
        //break;
        //window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(1.0, 0.0, 0.0), &Point3::new(1.0, 0.0, 0.0));
        //window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, 1.0, 0.0), &Point3::new(1.0, 1.0, 1.0));
        //window.draw_line(&Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, 0.0, 1.0), &Point3::new(0.0, 0.0, 1.0));
    }
}