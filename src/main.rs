use anyhow::Result;
use klystron::{
    runtime_3d::{launch, App},
    DrawType, Engine, FramePacket, Material, Mesh, Object, Vertex, UNLIT_FRAG, UNLIT_VERT,
};
use nalgebra::{Matrix4, Point3};

struct MyApp {
    material: Material,
    mesh: Mesh,
    time: f32,
}

impl App for MyApp {
    const NAME: &'static str = "MyApp";

    type Args = ();

    fn new(engine: &mut dyn Engine, _args: Self::Args) -> Result<Self> {
        let material = engine.add_material(UNLIT_VERT, UNLIT_FRAG, DrawType::Triangles)?;

        let (vertices, indices) = ravioli(1., 1., 30);
        let mesh = engine.add_mesh(&vertices, &indices)?;

        Ok(Self {
            mesh,
            material,
            time: 0.0,
        })
    }

    fn next_frame(&mut self, engine: &mut dyn Engine) -> Result<FramePacket> {
        let transform = Matrix4::identity();
        let object = Object {
            material: self.material,
            mesh: self.mesh,
            transform,
        };
        engine.update_time_value(self.time)?;
        self.time += 0.01;
        Ok(FramePacket {
            objects: vec![object],
        })
    }
}

fn main() -> Result<()> {
    let vr = std::env::args().skip(1).next().is_some();
    launch::<MyApp>(vr, ())
}

fn ravioli(width: f32, height: f32, steps: usize) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::with_capacity(steps * steps);
    let mut indices = Vec::with_capacity(vertices.len() * 2 * 3);

    for x in 0..steps {
        let x = x as f32 / steps as f32;
        for z in 0..steps {
            let z = z as f32 / steps as f32;
            let x = x * 2. - 1.;
            let z = z * 2. - 1.;

            let radius = 1.;
            let r = x * x + z * z;
            let height = (radius * radius - r).sqrt();
            vertices.push(Vertex {
                pos: [x * width, height, z * width],
                //color: [x, z, 0.],
                color: [x, z, 1. - x],
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
