#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(unsafe_code)]

use eframe::egui;

use egui::mutex::Mutex;
use std::sync::Arc;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(350.0, 380.0)),
        multisampling: 8,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Custom 3D painting in eframe using glow",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}

struct MyApp {
    /// Behind an `Arc<Mutex<…>>` so we can pass it to [`egui::PaintCallback`] and paint later.
    fractal: Arc<Mutex<Fractal>>,
    angle: f32,
    selected: bool,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");
        Self {
            fractal: Arc::new(Mutex::new(Fractal::new(gl))),
            angle: 0.0,
            selected: true,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.toggle_value(&mut self.selected, "Yes or Now?");
            if self.selected {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    self.custom_painting(ui);
                });

                ui.label("Drag to rotate!");
            }

            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }
        });
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.fractal.lock().destroy(gl);
        }
    }
}

impl MyApp {
    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

        self.angle += response.drag_delta().x * 0.01;

        // Clone locals so we can move them into the paint callback:
        let angle = self.angle;
        let fractal = self.fractal.clone();

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                fractal.lock().paint(painter.gl(), angle);
            })),
        };
        ui.painter().add(callback);
    }
}

struct Fractal {
    program: glow::Program,
    vertex_array: glow::VertexArray,
}

impl Fractal {
    fn new(gl: &glow::Context) -> Self {
        use glow::HasContext as _;
        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let (vertex_shader_source, fragment_shader_source) = (
                r#"
                const vec2 verts[3] = vec2[3](
                    vec2(0.0, 1.0),
                    vec2(-1.0, -1.0),
                    vec2(1.0, -1.0)
                );
                void main() {
                    gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
                }
                "#,
                r#"
                precision highp float;
                uniform vec2 u_fractalPosition;
                uniform vec2 u_cJulia;
                uniform float u_fractalZoom;
                uniform float u_brightness;
                uniform float u_contrast;
                uniform int u_highQuality;

                float computeLowQuality(in vec2 z)//, out float value)
                {
                    const int MAX_ITER = 1024;
                    const float N = 2.0;

                    vec2 z2;
                    float tmp;
                    int iterNumber = MAX_ITER;

                    // start at 1, since Log(0) = NaN
                    for(int i = 1; i < MAX_ITER; i++)
                    {
                        z2 = vec2(z.x * z.x, z.y * z.y); // z2x = zx * zx; z2y = zy * zy

                        if( (z2.x + z2.y) > N*N)
                        {
                                iterNumber = i;
                                break;
                        }

                        tmp = z2.x - z2.y + u_cJulia.x;
                        z.y = 2.0 * z.x * z.y + u_cJulia.y;
                        z.x = tmp;
                    }

                    return log(float(iterNumber + 1));
                }

                float computeHighQuality(in vec2 z)//, out float value)
                {
                    const int MAX_ITER = 4096;
                    const float N = 4.0;

                    vec2 z2;
                    float tmp, value;
                    int iterNumber = MAX_ITER;

                    // start at 1, since Log(0) = NaN
                    for(int i = 1; i < MAX_ITER; i++)
                    {
                        z2 = vec2(z.x * z.x, z.y * z.y); // z2x = zx * zx; z2y = zy * zy

                        if( (z2.x + z2.y) > N * N)
                        {
                                iterNumber = i;
                                break;
                        }

                        tmp = z2.x - z2.y + u_cJulia.x;
                        z.y = 2.0 * z.x * z.y + u_cJulia.y;
                        z.x = tmp;

                    }

                    // Smoothing the fractal: result = IterNumber - log2( log( abs(z) / log(N) ) )
                    value = float(iterNumber + 1) - log2( log( sqrt( z.x * z.x + z.y * z.y )/log(N)) );
                    return log(value);

                }
                out vec4 out_color;
                void main(void)
                {
                    vec2 z = vec2( (gl_FragCoord.x/u_fractalZoom - u_fractalPosition.x), (gl_FragCoord.y/u_fractalZoom - u_fractalPosition.y) );
                    float value;

                    if(u_highQuality != 0)
                        value = computeHighQuality(z);
                    else
                        value = computeLowQuality(z);

                    out_color =  vec4( u_brightness + u_contrast * vec3(value, value, value) * vec3(0.8, 0.75, 1.0), 1.0);
                }
                "#,
            );

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

    fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }

    fn paint(&self, gl: &glow::Context, angle: f32) {
        use glow::HasContext as _;
        unsafe {
            gl.use_program(Some(self.program));

            let c_julia = gl.get_uniform_location(self.program, "u_cJulia");
            gl.uniform_2_f32(c_julia.as_ref(), -0.76, -0.08);

            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_fractalZoom")
                    .as_ref(),
                4913.0,
            );

            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_brightness")
                    .as_ref(),
                -0.60,
            );

            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_contrast").as_ref(),
                0.30,
            );

            gl.uniform_1_i32(
                gl.get_uniform_location(self.program, "u_highQuality")
                    .as_ref(),
                0,
            );

            let u_fractal_position = gl.get_uniform_location(self.program, "u_fractalPosition");
            gl.uniform_2_f32(
                u_fractal_position.as_ref(),
                -1.071142577614754,
                0.005293330282132441,
            );

            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }
}