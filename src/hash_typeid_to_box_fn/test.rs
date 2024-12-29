use super::scene_day_3;

use super::scene_day_2::*;


#[derive(Default)]
struct VertexReady {
    data: Vec<f32>,
}

#[derive(Default)]
struct TextureReady {
    image_data: Vec<u8>,
}

#[derive(Default)]
struct MeshPaint {
    draw_count: i32,
}

// 实现相应的 trait
impl Ready for VertexReady {
    fn ready(&mut self, gfx: &Gfx) {
        println!("VertexReady: preparing {} vertices", self.data.len());
        // 模拟一些顶点数据准备工作
        self.data.push(1.0);
    }
}

impl Ready for TextureReady {
    fn ready(&mut self, gfx: &Gfx) {
        println!("TextureReady: loading texture data of {} bytes", self.image_data.len());
        // 模拟纹理加载
        self.image_data.push(255);
    }
}

impl Paint for MeshPaint {
    fn paint(&mut self, gfx: &Gfx) {
        self.draw_count += 1;
        println!("MeshPaint: drawing frame {}", self.draw_count);
    }
}

pub fn test_day_2() {
    let gfx = 0; // 模拟图形上下文

    // 创建并配置场景
    let mut scene = Scene::new("TestScene");

    // 添加准备阶段的组件
    scene.add_ready(VertexReady::default())
         .add_ready(TextureReady::default())
         .add_paint(MeshPaint::default());

    // 运行场景
    println!("Running scene: {}", scene.name);

    // 模拟几帧渲染
    for i in 0..3 {
        println!("\nFrame {}:", i);
        scene.run(&gfx);

        // 可以通过 get_mut 来访问和修改组件
        if let Some(mesh_paint) = scene.get_mut::<MeshPaint>() {
            println!("Current draw count: {}", mesh_paint.draw_count);
        }
    }



    scene_day_3::main();
}
