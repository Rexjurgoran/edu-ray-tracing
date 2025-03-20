use std::rc::Rc;

use crate::{
    aabb::Aabb,
    hittable_list::HittableList,
    interval::Interval,
    material::Material,
    sphere::{HitRecord, Hittable},
    vec3::{cross, dot, unit_vector, vec3, Vec3},
};

/// Quadrilateral, primitive defined by three geometric entities.
pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Material,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}
impl Quad {
    /// Creates a new quadrilateral
    /// 
    /// # Arguments
    /// 
    /// * `q` - Corner point of the quadrilateral
    /// * `u` - Vector along first edge
    /// * `v` - Vector along second edge
    /// * `mat` - Material of quadrilateral
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Material) -> Self {
        let n = cross(&u, &v);
        let normal = unit_vector(&n);
        let mut quad = Quad {
            q,
            u,
            v,
            w: n / dot(&n, &n),
            mat,
            bbox: Default::default(),
            normal,
            d: dot(&normal, &q),
        };
        quad.set_bounding_box();
        quad
    }

    /// Compute the bounding box of all four vertices.
    pub fn set_bounding_box(&mut self) {       
        let bbox_diagonal1 = Aabb::from_point(&self.q, &(self.q + self.u + self.v));
        let bbox_diagonal2 = Aabb::from_point(&(self.q + self.u), &(self.q + self.v));
        self.bbox = Aabb::from_aabb(&bbox_diagonal1, &bbox_diagonal2);
    }

    /// Given the hit point in plane coordinates, return false if it is outside the
    /// primitive, otherwise set the hit record UV coordinates and return true
    /// 
    /// # Arguments
    /// 
    /// * `a` - First coordinate of hit point
    /// * `b` - Second coordinate of hit point
    /// * `rec` - Hit record
    pub fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0); 

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::sphere::HitRecord,
    ) -> bool {
        let denom = dot(&self.normal, r.direction());

        // No hit if the ray is parallel to the plane.
        if f64::abs(denom) < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - dot(&self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);
        let planar_hitp_vector = intersection - self.q;
        let alpha = dot(&self.w, &cross(&planar_hitp_vector, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hitp_vector));

        if !Quad::is_interior(alpha, beta, rec) {
            return false;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return true.
        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal);

        true
    }

    fn bounding_box(&self) -> &crate::aabb::Aabb {
        &self.bbox
    }
}

pub fn bx(a: &Vec3, b: &Vec3, mat: Material) -> Rc<HittableList> {
    // Returns the 3D box (six sides) that contains the two opposite vertices a & b.
    let mut sides = HittableList::default();

    // Construct the two oppposite vertices with the minimum and maximum coordinates.
    let min = vec3(f64::min(a.x, b.x), f64::min(a.y, b.y), f64::min(a.z, b.z));
    let max = vec3(f64::max(a.x, b.x), f64::max(a.y, b.y), f64::max(a.z, b.z));

    let dx = vec3(max.x - min.x, 0.0, 0.0);
    let dy = vec3(0.0, max.y - min.y, 0.0);
    let dz = vec3(0.0, 0.0, max.z - min.z);

    sides.add(Rc::new(Quad::new(
        vec3(min.x, min.y, max.z),
        dx,
        dy,
        mat.clone(),
    ))); // front
    sides.add(Rc::new(Quad::new(
        vec3(max.x, min.y, max.z),
        -dz,
        dy,
        mat.clone(),
    ))); // right
    sides.add(Rc::new(Quad::new(
        vec3(max.x, min.y, min.z),
        -dx,
        dy,
        mat.clone(),
    ))); // back
    sides.add(Rc::new(Quad::new(
        vec3(min.x, min.y, min.z),
        dz,
        dy,
        mat.clone(),
    ))); // left
    sides.add(Rc::new(Quad::new(
        vec3(min.x, max.y, max.z),
        dx,
        -dz,
        mat.clone(),
    ))); // top
    sides.add(Rc::new(Quad::new(vec3(min.x, min.y, min.z), dx, dz, mat))); // bottom

    Rc::new(sides)
}
