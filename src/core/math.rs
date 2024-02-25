use std::fmt::Display;
use std::ops::Mul;

use paste::paste;

// commodity macro to create vectors easily
#[macro_export]
macro_rules! vec {
    ($x:expr, $y:expr) => {
        From::<_>::from([$x, $y])
    };
    ($x:expr, $y:expr, $z:expr) => {
        From::<_>::from([$x, $y, $z])
    };

    ($(_:tt)*) => {
        compile_error!("No vector type for specified number of components")
    };
}

macro_rules! repeat {
    ($repetition_token:tt, $what:tt) => ($what);
}

macro_rules! index {
    (x) => (0);
    (y) => (1);
    (z) => (2);
    (w) => (3);
}

macro_rules! make_vec {
    // use: let x,y,z <- Vec3 type f32
    // implements:
    // The vector struct (with debug, default, clone, copy)
    // Casting from the relevant array & tuple type
    // Pretty printing
    // Component wise operations (addition, multiplication)
    (let $($c:ident),+ <- $name:ident type $type_:tt) => {
        #[derive(Debug, Default, Clone, Copy)]
        pub struct $name {
        $(
            pub $c: $type_,
        )+
        }

        impl $name {
            pub fn new( $( $c: $type_),+ ) -> Self {
                Self {
                    $( $c ),+
                }
            }

            pub fn len2(self) -> $type_ {
                let mut ret = 0 as $type_;
                $( ret += self.$c * self.$c; )+
                ret
            }

            pub fn dot(self, rhs: Self) -> $type_ {
                let mut ret = 0 as $type_;
                $( ret += self.$c * rhs.$c; )+
                ret
            }
        }

        make_vec!(fimpl: $name; $type_; $($c),+);

        // inside this paste! {} macro
        // all identifiers within [< >] are concatenated
        paste! {
            // use [<$name LEN>]::$c as usize to get the index of the current component in a repeat context
            #[repr(usize)]
            #[allow(non_camel_case_types)]
            enum [<$name Indices>] {
                $($c),+
            }

            // use [<$name LEN>] to get the number of components (x,y,z = 3)
            #[allow(non_upper_case_globals)]
            const [<$name LEN>]: usize = [$([<$name Indices>]::$c),+].len();

            // example usage of both:
            // the expanded form would be:
            /*
            // in the next two lines, 3 would be acquired programatically
            impl From<[f32; 3]> for Vec3 {
                fn from(value: [f32; 3]) -> Self {
                    Self {
                        // as a repeated expression, where x and 0 are programatically acquired
                        x: value[0],
                    }
                }
            }
            */
            impl From<[$type_; [<$name LEN>]]> for $name {
                fn from(value: [$type_; [<$name LEN>]]) -> Self {
                    Self {
                    $(
                        $c: value[ [<$name Indices>]::$c as usize ],
                    )+
                    }
                }
            }

            // cheat a little bit
            // tuple -> array -> vector
            impl From<( $(repeat!($c, $type_)),+ )> for $name {
                fn from(value: ($(repeat!($c, $type_)),+)) -> Self {
                    Into::<[_; [<$name LEN>]]>::into(value).into()
                }
            }

        } // end of paste! {}

        impl std::ops::Index<usize> for $name {
            type Output=$type_;

            fn index(&self, idx: usize) -> &Self::Output {
                match idx {
                    $(
                    index!($c) => &self.$c,
                    )+
                    _ => panic!("Bad index"),
                }
            }
        }

        impl std::ops::IndexMut<usize> for $name {
            fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
                match idx {
                    $(
                    index!($c) => &mut self.$c,
                    )+
                    _ => panic!("Bad index"),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "( ")?;
                $(
                write!(f, "{:.2} ", self.$c)?;
                )+
                write!(f, ")")
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self {
                $(
                    $c: self.$c + other.$c,
                )+
                }
            }
        }

        impl std::ops::AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                *self = *self + other;
            }
        }

        impl std::ops::Mul for $name {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                Self {
                $(
                    $c: self.$c * other.$c,
                )+
                }
            }
        }

        impl std::ops::MulAssign for $name {
            fn mul_assign(&mut self, other: Self) {
                *self = *self * other;
            }
        }

        impl std::ops::Mul<$name> for $type_ {
            type Output = $name;

            fn mul(self, other: $name) -> $name {
                $name {
                $(
                    $c: self * other.$c,
                )+
                }
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self {
                $(
                    $c: self.$c - other.$c,
                )+
                }
            }
        }

        impl std::ops::SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                *self = *self - other;
            }
        }
    };

    (fimpl: $name:ident; f32; $($c:ident),+) =>  (make_vec!(fgenerate: $name; f32; $($c),+); );
    (fimpl: $name:ident; f64; $($c:ident),+) => (make_vec!(fgenerate: $name; f64; $($c),+); );
    (fgenerate: $name:ident; $type_:ty; $($c:ident),+) => {
        impl $name {
            pub fn is_zero(self) -> bool {
                let l = self.len2();
                const E: $type_ = 0.001;
                (-E..E).contains(&l)
            }

            pub fn is_unit(self) -> bool {
                let l = self.len2() - 1.0;
                const E: $type_ = 0.001;
                (-E..E).contains(&l)
            }

            pub fn len(self) -> $type_ {
                self.len2().sqrt()
            }

            pub fn normalize(self) -> Self {
                if self.is_zero() { return Self::default(); }
                let l = 1.0 / self.len();
                l * self
            }
        }
    };
    (fimpl: $name:ident; $type_:ty; $($c:ident),+) => ();
}

make_vec!(let x,y     <- Vec2 type f32);
make_vec!(let x,y,z   <- Vec3 type f32);
make_vec!(let x,y,z,w <- Vec4 type f32);

make_vec!(let x,y     <- DVec2 type f64);
make_vec!(let x,y,z   <- DVec3 type f64);
make_vec!(let x,y,z,w <- DVec4 type f64);

make_vec!(let x,y     <- IVec2 type i32);
make_vec!(let x,y,z   <- IVec3 type i32);
make_vec!(let x,y,z,w <- IVec4 type i32);

make_vec!(let x,y     <- UVec2 type u32);
make_vec!(let x,y,z   <- UVec3 type u32);
make_vec!(let x,y,z,w <- UVec4 type u32);

impl Vec2 {
    // gets the angle in degrees to the up vector
    // note that the angle is in the range (-180,180)
    pub fn angle(self) -> f32 {
        let up = Vec2::new(0.0, 1.0);
        // a.b = |a|*|b|*cos(th)
        self.dot(up).acos().to_degrees()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mat3([f32;9]);

impl Mat3 {
    pub fn identity() -> Self {
        Self([
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ])
    }

    pub fn scale(scale: Vec2) -> Self {
        Self([
            scale.x,    0.0,        0.0,
            0.0,        scale.y,    0.0,
            0.0,        0.0,        1.0,
        ])
    }

    pub fn translate(translate: Vec2) -> Self {
        Self([
            1.0,            0.0,            0.0,
            0.0,            1.0,            0.0,
            translate.x,    translate.y,    1.0,
        ])
    }

    pub fn rotate(angle: f32) -> Self {
        let angle = angle.to_radians();
        let sin = angle.sin();
        let cos = angle.cos();

        Self([
            cos, -sin, 0.0,
            sin,  cos, 0.0,
            0.0,  0.0, 1.0,
        ])
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.0.as_ptr()
    }
}

impl Default for Mat3 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Display for Mat3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {:.2} {:.2} {:.2} ]\n", self.0[0], self.0[3], self.0[6])?;
        write!(f, "[ {:.2} {:.2} {:.2} ]\n", self.0[1], self.0[4], self.0[7])?;
        write!(f, "[ {:.2} {:.2} {:.2} ]"  , self.0[2], self.0[5], self.0[8])
    }
}

impl Mul for Mat3 {
    type Output=Mat3;

    fn mul(self, rhs: Self) -> Self::Output {
        const N: usize = 3;
        let mut ret = Self::default();

        for y in 0..N {
            for x in 0..N {
                let mut sum = 0.0;
                for e in 0..N {
                    sum += self.0[N * e + x] * rhs.0[N * y + e];
                }
                ret.0[N * y + x] = sum;
            }
        }

        ret
    }
}

impl Mul<Vec3> for Mat3 {
    type Output=Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        const N: usize = 3;
        let mut ret = Vec3::default();

        for x in 0..N {
            let mut sum = 0.0;
            for e in 0..N {
                sum += self.0[N * e + x] * rhs[e];
            }
            ret[x] = sum;
        }

        ret
    }
}
