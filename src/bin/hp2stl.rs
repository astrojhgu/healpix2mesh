extern crate healpix2mesh;

use std::{
    rc::Rc
    , cell::RefCell
    , thread::sleep
    , time::Duration
    , path::Path
    , fs::{
        File
    }
    , io::{
        Write
    }
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

const pi_f32:f32=std::f32::consts::PI;

fn main(){
    let matches=App::new("show hp")
    .arg(
        Arg::new("input")
        .short('i')
        .long("infile")
        .takes_value(true)
        .value_name("healpix file")
        .required(true)
    )
    .arg(
        Arg::new("grid_iter")
        .short('n')
        .long("grid")
        .takes_value(true)
        .value_name("grid num iter")
        .default_missing_value("5")

    )
    .arg(
        Arg::new("slope")
        .short('k')
        .long("slope")
        .takes_value(true)
        .value_name("k")
        .required(true)
    )
    .arg(
        Arg::new("shift")
        .short('s')
        .long("shift")
        .takes_value(true)
        .value_name("shift")
        .required(true)
    )
    .arg(
        Arg::new("output")
        .short('o')
        .long("out")
        .takes_value(true)
        .value_name("output")
        .required(true)
    )
    .arg(
        Arg::new("bin")
        .short('b')
        .long("bin")
        .takes_value(false)
        .required(false)
    )
    .arg(
        Arg::new("neg")
        .short('z')
        .long("neg")
        .takes_value(false)
        .required(false)
    )
    .get_matches();

    
    let bin_stl=matches.occurrences_of("bin")>0;
    let k=matches.value_of("slope").unwrap().parse::<f64>().unwrap();
    let b=matches.value_of("shift").unwrap().parse::<f64>().unwrap();

    let clip_neg=matches.occurrences_of("neg")>0;

    let hpdata=read_map::<f64>(matches.value_of("input").unwrap(), &["TEMPERATURE"], 1).pop().unwrap();
    let grid_niter=matches.value_of("grid_iter").unwrap().parse::<usize>().unwrap();

    let max_data_value=hpdata.iter().cloned().reduce(|a,b|{
        if a>b{a}else{b}
    }).unwrap();
    let min_data_value=hpdata.iter().cloned().reduce(|a,b|{
        if a<b{a}else{b}
    }).unwrap();
    
    let mut outfile=File::create(matches.value_of("output").unwrap()).unwrap();
    let mut tes=
    if clip_neg{
        hp2vertices(&hpdata, grid_niter, &|x| if x<=0.0 {0.0} else {k*x+b})
    }else{
        hp2vertices(&hpdata, grid_niter, &|x| k*x+b)
    };
    
    //let mut tes=hp2vertices(&hpdata, grid_niter, &|x| 1.0 );


    if bin_stl{
        let norms=tes.regulate_norm();
        outfile.write(&[0_u8;80]).unwrap();
        let ntriangles=tes.faces.len() as u32;
        outfile.write(&ntriangles.to_le_bytes()).unwrap();
        for (face, &norm) in tes.faces.iter().zip(norms.iter()){
            for i in 0..3{
                let n=norm[i] as f32;
                outfile.write(&n.to_le_bytes()).unwrap();
            }
            for &vid in face{
                let vertex=tes.vertices[vid];
                for i in 0..3{
                    let v=vertex[i] as f32;
                    outfile.write(&v.to_le_bytes()).unwrap();
                }   
            }
            outfile.write(&8_u16.to_le_bytes()).unwrap();
        }
    }else{
        writeln!(&mut outfile, "solid obj").unwrap();
        let norms=tes.regulate_norm();
        for (face, &norm) in tes.faces.iter().zip(norms.iter()){
            writeln!(&mut outfile, "facet normal {} {} {}", norm.x, norm.y, norm.z);
            writeln!(&mut outfile, "outer loop");
            for &vid in face{
                let vertex=tes.vertices[vid];
                writeln!(&mut outfile, "vertex {} {} {}", vertex.x, vertex.y, vertex.z);
            }
            writeln!(&mut outfile, "endloop");
            writeln!(&mut outfile, "endfacet");
        }
        writeln!(&mut outfile, "endsolid obj");
    } 
}