use std::fs::File;

use super::State;

use std::io::Read;

pub struct FractalGl {
    program: glow::Program,
    vertex_array: glow::VertexArray,
}

impl FractalGl {
    pub fn new(gl: &glow::Context) -> Self {
        use glow::HasContext as _;
        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let mut vertex_shader_source = String::new();
            File::open("assets/vertex.shader")
                .and_then(|mut x| x.read_to_string(&mut vertex_shader_source))
                .expect("Cannot read the Vertex Shaders");

            let mut fragment_shader_source = String::new();
            File::open("assets/fragment.shader")
                .and_then(|mut x| x.read_to_string(&mut fragment_shader_source))
                .expect("Cannot read the Fragment Shaders");

            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];

            let shaders: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, shader_source)| {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("Cannot create shader");
                    gl.shader_source(shader, &format!("{}\n{}", "#version 330", shader_source));
                    gl.compile_shader(shader);
                    assert!(
                        gl.get_shader_compile_status(shader),
                        "Failed to compile Shader {shader_type} - {}:\n{}",
                        gl.get_shader_info_log(shader),
                        shader_source
                    );
                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            Self {
                program,
                vertex_array,
            }
        }
    }

    pub fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }

    pub fn paint(&self, gl: &glow::Context, state: State) {
        use glow::HasContext as _;
        unsafe {
            gl.use_program(Some(self.program));

            let c_julia = gl.get_uniform_location(self.program, "u_cJulia");
            gl.uniform_2_f32(c_julia.as_ref(), state.c_julia.x, state.c_julia.y);

            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_fractalZoom")
                    .as_ref(),
                state.zoom,
            );

            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_brightness")
                    .as_ref(),
                state.brightness,
            );

            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_contrast").as_ref(),
                state.contrast,
            );

            gl.uniform_1_i32(
                gl.get_uniform_location(self.program, "u_highQuality")
                    .as_ref(),
                0,
            );

            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_r")
                    .as_ref(),
                state.r,
            );

            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_g")
                    .as_ref(),
                state.g,
            );

            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_b")
                    .as_ref(),
                state.b,
            );

            let u_fractal_position = gl.get_uniform_location(self.program, "u_fractalPosition");
            gl.uniform_2_f32(u_fractal_position.as_ref(), state.pos.x, state.pos.y);

            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
}
