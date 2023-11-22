use crate::geom::vec3::{Point3, Vec3};
use rand::Rng;

fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum: f64 = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                    * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                    * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                    * c[i][j][k].dot(weight);
            }
        }
    }
    accum
}

#[derive(Clone)]
pub struct Perlin {
    rd_vec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Perlin {
        fn permute(mut v: Vec<usize>, n: usize) -> Vec<usize> {
            let mut rng = rand::thread_rng();
            for i in (0..n as usize).rev() {
                let target = rng.gen_range(0..=i);
                v.swap(i, target);
            }
            v
        }

        Perlin {
            rd_vec: (0..256)
                .map(|_| Vec3::random_in_unit_sphere())
                .collect::<Vec<_>>(),
            perm_x: permute((0..256).collect::<Vec<_>>(), 256),
            perm_y: permute((0..256).collect::<Vec<_>>(), 256),
            perm_z: permute((0..256).collect::<Vec<_>>(), 256),
        }
    }

    pub fn perlin(&self, p: &Point3, scale: f64) -> f64 {
        let mut u = scale * p.x - (scale * p.x).floor();
        let mut v = scale * p.y - (scale * p.y).floor();
        let mut w = scale * p.z - (scale * p.z).floor();
        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);
        let i = (scale * p.x).floor() as usize;
        let j = (scale * p.y).floor() as usize;
        let k = (scale * p.z).floor() as usize;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rd_vec[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]]
                }
            }
        }

        perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Vec3, scale: f64, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        (0..depth).for_each(|_| {
            accum += weight * self.perlin(&temp_p, scale);
            weight *= 0.5;
            temp_p *= 2.0;
        });
        accum.abs()
    }
}
