use crate::{matrix, Float, Matrix};

pub fn identity() -> Matrix {
    matrix([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn translation<A: Into<Float>, B: Into<Float>, C: Into<Float>>(x: A, y: B, z: C) -> Matrix {
    let x = x.into();
    let y = y.into();
    let z = z.into();

    matrix([
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn scaling<A: Into<Float>, B: Into<Float>, C: Into<Float>>(x: A, y: B, z: C) -> Matrix {
    let x = x.into();
    let y = y.into();
    let z = z.into();

    matrix([
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_x<A: Into<Float>>(r: A) -> Matrix {
    let r = r.into();

    matrix([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, r.cos(), -r.sin(), 0.0],
        [0.0, r.sin(), r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_y<A: Into<Float>>(r: A) -> Matrix {
    let r = r.into();

    matrix([
        [r.cos(), 0.0, r.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-r.sin(), 0.0, r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_z<A: Into<Float>>(r: A) -> Matrix {
    let r = r.into();

    matrix([
        [r.cos(), -r.sin(), 0.0, 0.0],
        [r.sin(), r.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shearing<
    A: Into<Float>,
    B: Into<Float>,
    C: Into<Float>,
    D: Into<Float>,
    E: Into<Float>,
    F: Into<Float>,
>(
    xy: A,
    xz: B,
    yx: C,
    yz: D,
    zx: E,
    zy: F,
) -> Matrix {
    let xy = xy.into();
    let xz = xz.into();
    let yx = yx.into();
    let yz = yz.into();
    let zx = zx.into();
    let zy = zy.into();

    matrix([
        [1.0, xy, xz, 0.0],
        [yx, 1.0, yz, 0.0],
        [zx, zy, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

impl Matrix {
    pub fn translation<A: Into<Float>, B: Into<Float>, C: Into<Float>>(
        &self,
        x: A,
        y: B,
        z: C,
    ) -> Matrix {
        translation(x, y, z) * *self
    }

    pub fn scaling<A: Into<Float>, B: Into<Float>, C: Into<Float>>(
        &self,
        x: A,
        y: B,
        z: C,
    ) -> Matrix {
        scaling(x, y, z) * *self
    }

    pub fn rotation_x<A: Into<Float>>(&self, r: A) -> Matrix {
        rotation_x(r) * *self
    }

    pub fn rotation_y<A: Into<Float>>(&self, r: A) -> Matrix {
        rotation_y(r) * *self
    }

    pub fn rotation_z<A: Into<Float>>(&self, r: A) -> Matrix {
        rotation_z(r) * *self
    }

    pub fn shearing<
        A: Into<Float>,
        B: Into<Float>,
        C: Into<Float>,
        D: Into<Float>,
        E: Into<Float>,
        F: Into<Float>,
    >(
        &self,
        xy: A,
        xz: B,
        yx: C,
        yz: D,
        zx: E,
        zy: F,
    ) -> Matrix {
        shearing(xy, xz, yx, yz, zx, zy) * *self
    }
}
