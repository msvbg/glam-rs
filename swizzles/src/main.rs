use std::io::{self, Write};

fn print_head(out: &mut impl Write) -> std::io::Result<()> {
    write!(
        out,
        r#"// Generated by swizzles. Do not edit.
use super::{{Vec2, Vec3, Vec3A, Vec4}};

#[cfg(all(vec4_sse2, target_arch = "x86"))]
use core::arch::x86::*;
#[cfg(all(vec4_sse2, target_arch = "x86_64"))]
use core::arch::x86_64::*;
"#
    )
}

fn print_vec4(out: &mut impl Write) -> std::io::Result<()> {
    const SIZE: usize = 4;
    const E: [char; SIZE] = ['x', 'y', 'z', 'w']; // element name
    const B: [&str; SIZE] = ["00", "01", "10", "11"]; // shuffle bits

    write!(out, "\nimpl Vec4 {{")?;

    // to vec4
    for e0 in 0..SIZE {
        for e1 in 0..SIZE {
            for e2 in 0..SIZE {
                for e3 in 0..SIZE {
                    write!(
                        out,
                        r#"
    #[inline]
    pub fn {}{}{}{}(self) -> Vec4 {{
        #[cfg(vec4_sse2)]
        unsafe {{
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b{}_{}_{}_{}))
        }}

        #[cfg(vec4_f32)]
        {{
            Vec4(self.{}, self.{}, self.{}, self.{})
        }}
    }}"#,
                        E[e0], E[e1], E[e2], E[e3], B[e0], B[e1], B[e2], B[e3], e0, e1, e2, e3,
                    )?;
                }
            }
        }
    }

    // to vec3
    for e0 in 0..SIZE {
        for e1 in 0..SIZE {
            for e2 in 0..SIZE {
                write!(
                    out,
                    r#"
    #[inline]
    pub fn {}{}{}(self) -> Vec3 {{
        #[cfg(vec4_sse2)]
        unsafe {{
            Vec3::from(Vec4(_mm_shuffle_ps(self.0, self.0, 0b{}_{}_{}_00)))
        }}

        #[cfg(vec4_f32)]
        {{
            Vec3(self.{}, self.{}, self.{})
        }}
    }}"#,
                    E[e0], E[e1], E[e2], B[e0], B[e1], B[e2], e0, e1, e2
                )?;
            }
        }
    }

    // to vec2
    for e0 in 0..SIZE {
        for e1 in 0..SIZE {
            write!(
                out,
                r#"
    #[inline]
    pub fn {}{}(self) -> Vec2 {{
        #[cfg(vec4_sse2)]
        unsafe {{
            Vec2::from(Vec4(_mm_shuffle_ps(self.0, self.0, 0b{}_{}_00_00)))
        }}

        #[cfg(vec4_f32)]
        {{
            Vec2(self.{}, self.{})
        }}
    }}"#,
                E[e0], E[e1], B[e0], B[e1], e0, e1
            )?;
        }
    }

    write!(out, "\n}}\n")
}

fn print_vec3a(out: &mut impl Write) -> std::io::Result<()> {
    const SIZE: usize = 3;
    const E: [char; SIZE] = ['x', 'y', 'z']; // element name
    const B: [&str; SIZE] = ["00", "01", "10"]; // shuffle bits

    write!(out, "\nimpl Vec3A {{")?;

    // to vec4
    for e0 in 0..SIZE {
        for e1 in 0..SIZE {
            for e2 in 0..SIZE {
                for e3 in 0..SIZE {
                    write!(
                        out,
                        r#"
    #[inline]
    pub fn {}{}{}{}(self) -> Vec4 {{
        #[cfg(vec3a_sse2)]
        unsafe {{
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b{}_{}_{}_{}))
        }}

        #[cfg(vec3a_f32)]
        {{
            Vec4(self.0.{}, self.0.{}, self.0.{}, self.0.{})
        }}
    }}"#,
                        E[e0], E[e1], E[e2], E[e3], B[e0], B[e1], B[e2], B[e3], e0, e1, e2, e3,
                    )?;
                }
            }
        }
    }

    // to vec3
    for e0 in 0..SIZE {
        for e1 in 0..SIZE {
            for e2 in 0..SIZE {
                write!(
                    out,
                    r#"
    #[inline]
    pub fn {}{}{}(self) -> Vec3A {{
        #[cfg(vec3a_sse2)]
        unsafe {{
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b{}_{}_{}_00))
        }}

        #[cfg(vec3a_f32)]
        {{
            Vec3A(Vec3(self.0.{}, self.0.{}, self.0.{}))
        }}
    }}"#,
                    E[e0], E[e1], E[e2], B[e0], B[e1], B[e2], e0, e1, e2
                )?;
            }
        }
    }

    // to vec2
    for e0 in 0..SIZE {
        for e1 in 0..SIZE {
            write!(
                out,
                r#"
    #[inline]
    pub fn {}{}(self) -> Vec2 {{
        #[cfg(vec3a_sse2)]
        unsafe {{
            Vec2::from(Vec3A(_mm_shuffle_ps(self.0, self.0, 0b{}_{}_00_00)))
        }}

        #[cfg(vec3a_f32)]
        {{
            Vec2(self.0.{}, self.0.{})
        }}
    }}"#,
                E[e0], E[e1], B[e0], B[e1], e0, e1
            )?;
        }
    }

    write!(out, "\n}}\n")
}

fn print_vec3(out: &mut impl Write) -> std::io::Result<()> {
    const SIZE: usize = 3;
    const E: [char; SIZE] = ['x', 'y', 'z']; // element name

    write!(out, "\nimpl Vec3 {{")?;

    // to vec4
    for e0 in 0..SIZE {
        for e1 in 0..SIZE {
            for e2 in 0..SIZE {
                for e3 in 0..SIZE {
                    write!(
                        out,
                        r#"
    #[inline]
    pub fn {}{}{}{}(self) -> Vec4 {{
        Vec4::new(self.{}, self.{}, self.{}, self.{})
    }}"#,
                        E[e0], E[e1], E[e2], E[e3], e0, e1, e2, e3,
                    )?;
                }
            }
        }
    }

    // to vec3
    for e0 in 0..SIZE {
        for e1 in 0..SIZE {
            for e2 in 0..SIZE {
                write!(
                    out,
                    r#"
    #[inline]
    pub fn {}{}{}(self) -> Vec3 {{
        Vec3(self.{}, self.{}, self.{})
    }}"#,
                    E[e0], E[e1], E[e2], e0, e1, e2
                )?;
            }
        }
    }

    // to vec2
    for e0 in 0..SIZE {
        for e1 in 0..SIZE {
            write!(
                out,
                r#"
    #[inline]
    pub fn {}{}(self) -> Vec2 {{
        Vec2(self.{}, self.{})
    }}"#,
                E[e0], E[e1], e0, e1
            )?;
        }
    }

    write!(out, "\n}}\n")
}

fn print_vec2(out: &mut impl Write) -> std::io::Result<()> {
    const SIZE: usize = 2;
    const E: [char; SIZE] = ['x', 'y']; // element name

    write!(out, "\nimpl Vec2 {{")?;

    // to vec4
    for e0 in 0..SIZE {
        for e1 in 0..SIZE {
            for e2 in 0..SIZE {
                for e3 in 0..SIZE {
                    write!(
                        out,
                        r#"
    #[inline]
    pub fn {}{}{}{}(self) -> Vec4 {{
        Vec4::new(self.{}, self.{}, self.{}, self.{})
    }}"#,
                        E[e0], E[e1], E[e2], E[e3], e0, e1, e2, e3,
                    )?;
                }
            }
        }
    }

    // to vec3
    for e0 in 0..SIZE {
        for e1 in 0..SIZE {
            for e2 in 0..SIZE {
                write!(
                    out,
                    r#"
    #[inline]
    pub fn {}{}{}(self) -> Vec3 {{
        Vec3(self.{}, self.{}, self.{})
    }}"#,
                    E[e0], E[e1], E[e2], e0, e1, e2
                )?;
            }
        }
    }

    // to vec2
    for e0 in 0..SIZE {
        for e1 in 0..SIZE {
            write!(
                out,
                r#"
    #[inline]
    pub fn {}{}(self) -> Vec2 {{
        Vec2(self.{}, self.{})
    }}"#,
                E[e0], E[e1], e0, e1
            )?;
        }
    }

    write!(out, "\n}}\n")
}

fn main() -> std::io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    print_head(&mut handle)?;
    print_vec4(&mut handle)?;
    print_vec3a(&mut handle)?;
    print_vec3(&mut handle)?;
    print_vec2(&mut handle)
}
