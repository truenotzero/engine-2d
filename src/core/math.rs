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

macro_rules! make_vec {
    // use: let x,y,z <- Vec3 type f32
    (let $($c:ident),+ <- $name:ident type $type_:ty) => {
        #[derive(Debug, Default, Clone, Copy)]
        pub struct $name {
        $(
            pub $c: $type_,
        )+
        }

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

        } // end of paste! {}

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
    };
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

// #[derive(Default, Clone, Copy)]
// pub struct Vec2 {
//     pub x: f32,
//     pub y: f32
// }

// impl From<[f32; 2]> for Vec2 {
//     fn from(value: [f32; 2]) -> Self {
//         Self {
//             x: value[0],
//             y: value[1],
//         }
//     }
// }

// pub struct IVec2 {
//     pub x: i32,
//     pub y: i32,
// }

// impl From<[i32; 2]> for IVec2 {
//     fn from(value: [i32; 2]) -> Self {
//         Self {
//             x: value[0],
//             y: value[1],
//         }
//     }
// }

// pub struct UVec2 {
//     pub x: u32,
//     pub y: u32,
// }

// impl From<[u32; 2]> for UVec2 {
//     fn from(value: [u32; 2]) -> Self {
//         Self {
//             x: value[0],
//             y: value[1],
//         }
//     }
// }

// #[derive(Default, Clone, Copy)]
// pub struct Vec3 {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
// }

// impl From<[f32; 2]> for Vec2 {
//     fn from(value: [f32; 2]) -> Self {
//         Self {
//             x: value[0],
//             y: value[1],
//         }
//     }
// }
