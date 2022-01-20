use scorus::{
    healpix::{
        interp::{
            get_interpol_ring
        }
        , utils::{
            npix2nside
        }
    }
    , coordinates::{
        SphCoord
    }
    , sph_tessellation::{
        Tessellation
    }
};


pub fn hp2vertices(hpdata: &[f64], refine_niter: usize, f_v2r: &dyn Fn(f64)->f64)->Tessellation<f64>{
    let nside=npix2nside(hpdata.len());
    let mut tes=Tessellation::<f64>::octahedron();

    for _i in 0..refine_niter{
        tes.refine();
    }
    tes.regulate_norm();
    let r_list:Vec<_>=tes.vertices.iter().map(|&p|{
        let sph=SphCoord::from_vec3d(p);
        let (pix, w)=get_interpol_ring::<f64>(nside, sph);
        let value=pix.iter().zip(w.iter()).map(|(&p, &w)| hpdata[p]*w).sum::<f64>();
        f_v2r(value)
    }).collect();
    tes.vertices.iter_mut().zip(r_list.iter()).for_each(|(v, &r)|{
        *v=*v*r;
    });
    tes
}
