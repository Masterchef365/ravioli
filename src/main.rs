use anyhow::{format_err, Context, Result};
use klystron::{
    runtime_3d::{launch, App},
    DrawType, Engine, FramePacket, Material, Mesh, Object, Vertex,
};
use nalgebra::{Matrix4, Vector3, Vector4};
use notify::{watcher, DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use shaderc::{CompileOptions, Compiler, ShaderKind};
use std::fs;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

struct MyApp {
    file_watcher: RecommendedWatcher,
    file_watch_rx: Receiver<DebouncedEvent>,
    material: Material,
    compiler: Compiler,
    vert: Vec<u8>,
    frag: Vec<u8>,
    mesh: Mesh,
    time: f32,
}

impl MyApp {
    fn update_shader(&mut self, path: &Path, engine: &mut dyn Engine) -> Result<()> {
        println!("GOT {:?}", path);
        let vert = match path.extension().and_then(|v| v.to_str()) {
            Some("vert") => true,
            Some("frag") => false,
            None | Some(_) => return Ok(()),
        };

        if path.file_stem().unwrap() != "unlit" {
            return Ok(());
        }

        let source = fs::read_to_string(path)
            .with_context(|| format_err!("File error loading {:?}", path))?;

        let spv = self
            .compiler
            .compile_into_spirv(
                &source,
                if vert {
                    ShaderKind::Vertex
                } else {
                    ShaderKind::Fragment
                },
                path.to_str().unwrap(),
                "main",
                None,
            )
            .context("Failed to compile shader")?;
        let spv = spv.as_binary_u8().to_vec();

        if vert {
            self.vert = spv;
        } else {
            self.frag = spv;
        }

        engine.remove_material(self.material)?;
        self.material = engine.add_material(&self.vert, &self.frag, DrawType::Triangles)?;

        Ok(())
    }
}

impl App for MyApp {
    const NAME: &'static str = "Ravioli";

    type Args = ();

    fn new(engine: &mut dyn Engine, _args: Self::Args) -> Result<Self> {
        let vert = fs::read("./shaders/unlit.vert.spv")?;
        let frag = fs::read("./shaders/unlit.frag.spv")?;

        let compiler = Compiler::new().context("Shaderc failed to create compiler")?;

        let material = engine.add_material(&vert, &frag, DrawType::Triangles)?;

        let (tx, file_watch_rx) = channel();
        let mut file_watcher = watcher(tx, Duration::from_millis(500))?;
        file_watcher.watch("./shaders", RecursiveMode::NonRecursive)?;

        let (vertices, indices) = ravioli(1., 1.8, 1.6, 30);
        let mesh = engine.add_mesh(&vertices, &indices)?;

        Ok(Self {
            compiler,
            vert,
            frag,
            file_watcher,
            file_watch_rx,
            mesh,
            material,
            time: 0.0,
        })
    }

    fn next_frame(&mut self, engine: &mut dyn Engine) -> Result<FramePacket> {
        match self.file_watch_rx.try_recv() {
            Ok(DebouncedEvent::Create(p)) | Ok(DebouncedEvent::Write(p)) => {
                if let Err(e) = self.update_shader(&p, engine) {
                    println!("Shader compilation error: {:?}", e);
                }
            }
            _ => (),
        }

        //let scale = 200.;
        let scale = 1.;
        let trans = Matrix4::new_translation(&Vector3::new(1., 1., -1.))
            * Matrix4::from_diagonal(&Vector4::new(scale, scale, scale, 1.));

        let top = Object {
            material: self.material,
            mesh: self.mesh,
            transform: trans * Matrix4::identity(),
        };
        let bottom = Object {
            material: self.material,
            mesh: self.mesh,
            transform: trans * Matrix4::from_diagonal(&Vector4::new(1., -1., 1., 1.)),
        };

        engine.update_time_value(self.time)?;
        self.time += 0.01;
        Ok(FramePacket {
            objects: vec![top, bottom],
        })
    }
}

fn main() -> Result<()> {
    let vr = std::env::args().skip(1).next().is_some();
    launch::<MyApp>(vr, ())
}

fn ravioli(width: f32, radius: f32, offset: f32, steps: usize) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::with_capacity(steps * steps);
    let mut indices = Vec::with_capacity(vertices.len() * 2 * 3);

    for x in 0..steps {
        let x = x as f32 / steps as f32;
        for z in 0..steps {
            let z = z as f32 / steps as f32;
            let x = x * 2. - 1.;
            let z = z * 2. - 1.;

            let r = x * x + z * z;
            let height = (radius * radius - r).sqrt();
            vertices.push(Vertex {
                pos: [x * width, height - offset, z * width],
                //color: [x, z, 0.],
                color: [x, z, height - offset],
            });
        }
    }

    for i in 0..vertices.len() - steps {
        if i % steps == steps - 1 {
            continue;
        };

        let i = i as u16;
        let tl = i + 0;
        let tr = i + 1;
        let bl = i + 0 + steps as u16;
        let br = i + 1 + steps as u16;

        // Outside
        indices.push(tl);
        indices.push(tr);
        indices.push(bl);

        indices.push(bl);
        indices.push(tr);
        indices.push(br);

        // Inside
        indices.push(bl);
        indices.push(tr);
        indices.push(tl);

        indices.push(br);
        indices.push(tr);
        indices.push(bl);
    }

    (vertices, indices)
}
