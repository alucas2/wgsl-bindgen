#![allow(unused, non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShaderRegistry {
    Main,
}
impl ShaderRegistry {
    pub fn create_pipeline_layout(&self, device: &wgpu::Device) -> wgpu::PipelineLayout {
        match self {
            Self::Main => main::create_pipeline_layout(device),
        }
    }
    pub fn create_shader_module_embed_source(
        &self,
        device: &wgpu::Device,
    ) -> wgpu::ShaderModule {
        match self {
            Self::Main => main::create_shader_module_embed_source(device),
        }
    }
    pub fn create_shader_module_from_path(
        &self,
        device: &wgpu::Device,
        shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>,
    ) -> wgpu::ShaderModule {
        match self {
            Self::Main => main::create_shader_module_from_path(device, shader_defs),
        }
    }
    pub fn shader_files(&self) -> &[&str] {
        match self {
            Self::Main => main::SHADER_FILES,
        }
    }
}
const _: () = {
    assert!(std::mem::size_of:: < glam::Vec3A > () == 16);
    assert!(std::mem::align_of:: < glam::Vec3A > () == 16);
    assert!(std::mem::size_of:: < glam::Vec4 > () == 16);
    assert!(std::mem::align_of:: < glam::Vec4 > () == 16);
    assert!(std::mem::size_of:: < glam::Mat3A > () == 48);
    assert!(std::mem::align_of:: < glam::Mat3A > () == 16);
    assert!(std::mem::size_of:: < glam::Mat4 > () == 64);
    assert!(std::mem::align_of:: < glam::Mat4 > () == 16);
};
mod _root {
    pub use super::*;
}
pub mod main {
    use super::{_root, _root::*};
    #[repr(C, align(16))]
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Style {
        /// size: 16, offset: 0x0, type: `vec4<f32>`
        pub color: glam::Vec4,
        /// size: 4, offset: 0x10, type: `f32`
        pub width: f32,
        pub _pad_width: [u8; 0x10 - core::mem::size_of::<f32>()],
    }
    impl Style {
        pub const fn new(color: glam::Vec4, width: f32) -> Self {
            Self {
                color,
                width,
                _pad_width: [0; 0x10 - core::mem::size_of::<f32>()],
            }
        }
    }
    unsafe impl bytemuck::Zeroable for Style {}
    unsafe impl bytemuck::Pod for Style {}
    const _: () = {
        assert!(std::mem::offset_of!(Style, color) == 0);
        assert!(std::mem::offset_of!(Style, width) == 16);
        assert!(std::mem::size_of:: < Style > () == 32);
    };
    #[repr(C)]
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct StyleInit {
        pub color: glam::Vec4,
        pub width: f32,
    }
    impl StyleInit {
        pub const fn build(&self) -> Style {
            Style {
                color: self.color,
                width: self.width,
                _pad_width: [0; 0x10 - core::mem::size_of::<f32>()],
            }
        }
    }
    impl From<StyleInit> for Style {
        fn from(data: StyleInit) -> Self {
            data.build()
        }
    }
    pub mod bind_groups {
        #[derive(Debug)]
        pub struct BindGroup0(wgpu::BindGroup);
        #[derive(Debug)]
        pub struct BindGroupLayout0<'a> {
            pub buffer: wgpu::BufferBinding<'a>,
        }
        const LAYOUT_DESCRIPTOR0: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage {
                            read_only: false,
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        };
        impl BindGroup0 {
            pub fn get_bind_group_layout(
                device: &wgpu::Device,
            ) -> wgpu::BindGroupLayout {
                device.create_bind_group_layout(&LAYOUT_DESCRIPTOR0)
            }
            pub fn from_bindings(
                device: &wgpu::Device,
                bindings: BindGroupLayout0,
            ) -> Self {
                let bind_group_layout = device
                    .create_bind_group_layout(&LAYOUT_DESCRIPTOR0);
                let bind_group = device
                    .create_bind_group(
                        &wgpu::BindGroupDescriptor {
                            layout: &bind_group_layout,
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: wgpu::BindingResource::Buffer(bindings.buffer),
                                },
                            ],
                            label: None,
                        },
                    );
                Self(bind_group)
            }
            pub fn set<'a>(&'a self, render_pass: &mut wgpu::ComputePass<'a>) {
                render_pass.set_bind_group(0, &self.0, &[]);
            }
        }
        #[derive(Debug)]
        pub struct BindGroup1(wgpu::BindGroup);
        #[derive(Debug)]
        pub struct BindGroupLayout1<'a> {
            pub ONE: wgpu::BufferBinding<'a>,
        }
        const LAYOUT_DESCRIPTOR1: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 11,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        };
        impl BindGroup1 {
            pub fn get_bind_group_layout(
                device: &wgpu::Device,
            ) -> wgpu::BindGroupLayout {
                device.create_bind_group_layout(&LAYOUT_DESCRIPTOR1)
            }
            pub fn from_bindings(
                device: &wgpu::Device,
                bindings: BindGroupLayout1,
            ) -> Self {
                let bind_group_layout = device
                    .create_bind_group_layout(&LAYOUT_DESCRIPTOR1);
                let bind_group = device
                    .create_bind_group(
                        &wgpu::BindGroupDescriptor {
                            layout: &bind_group_layout,
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 11,
                                    resource: wgpu::BindingResource::Buffer(bindings.ONE),
                                },
                            ],
                            label: None,
                        },
                    );
                Self(bind_group)
            }
            pub fn set<'a>(&'a self, render_pass: &mut wgpu::ComputePass<'a>) {
                render_pass.set_bind_group(1, &self.0, &[]);
            }
        }
        #[derive(Debug, Copy, Clone)]
        pub struct BindGroups<'a> {
            pub bind_group0: &'a BindGroup0,
            pub bind_group1: &'a BindGroup1,
        }
        impl<'a> BindGroups<'a> {
            pub fn set(&self, pass: &mut wgpu::ComputePass<'a>) {
                self.bind_group0.set(pass);
                self.bind_group1.set(pass);
            }
        }
    }
    pub fn set_bind_groups<'a>(
        pass: &mut wgpu::ComputePass<'a>,
        bind_group0: &'a bind_groups::BindGroup0,
        bind_group1: &'a bind_groups::BindGroup1,
    ) {
        bind_group0.set(pass);
        bind_group1.set(pass);
    }
    pub mod compute {
        pub const MAIN_WORKGROUP_SIZE: [u32; 3] = [1, 1, 1];
        pub fn create_main_pipeline_embed_source(
            device: &wgpu::Device,
        ) -> wgpu::ComputePipeline {
            let module = super::create_shader_module_embed_source(device);
            let layout = super::create_pipeline_layout(device);
            device
                .create_compute_pipeline(
                    &wgpu::ComputePipelineDescriptor {
                        label: Some("Compute Pipeline main"),
                        layout: Some(&layout),
                        module: &module,
                        entry_point: "main",
                    },
                )
        }
        pub fn create_main_pipeline_from_path(
            device: &wgpu::Device,
            shader_defs: std::collections::HashMap<
                String,
                naga_oil::compose::ShaderDefValue,
            >,
        ) -> wgpu::ComputePipeline {
            let module = super::create_shader_module_from_path(device, shader_defs);
            let layout = super::create_pipeline_layout(device);
            device
                .create_compute_pipeline(
                    &wgpu::ComputePipelineDescriptor {
                        label: Some("Compute Pipeline main"),
                        layout: Some(&layout),
                        module: &module,
                        entry_point: "main",
                    },
                )
        }
    }
    pub const ENTRY_MAIN: &str = "main";
    pub fn create_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
        device
            .create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[
                        &bind_groups::BindGroup0::get_bind_group_layout(device),
                        &bind_groups::BindGroup1::get_bind_group_layout(device),
                    ],
                    push_constant_ranges: &[],
                },
            )
    }
    pub fn create_shader_module_embed_source(
        device: &wgpu::Device,
    ) -> wgpu::ShaderModule {
        let source = std::borrow::Cow::Borrowed(SHADER_STRING);
        device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("main.wgsl"),
                source: wgpu::ShaderSource::Wgsl(source),
            })
    }
    const SHADER_STRING: &'static str = r#"
struct Style {
    color: vec4<f32>,
    width: f32,
}

@group(1) @binding(11) 
var<uniform> ONEX_naga_oil_mod_XMJUW4ZDJNZTXGX: f32;
@group(0) @binding(0) 
var<storage, read_write> buffer: array<f32>;
var<push_constant> const_style: Style;

@compute @workgroup_size(1, 1, 1) 
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let _e5 = ONEX_naga_oil_mod_XMJUW4ZDJNZTXGX;
    let _e11 = const_style.color.w;
    let _e15 = const_style.width;
    let _e17 = buffer[id.x];
    buffer[id.x] = (_e17 * (((2f * _e5) * _e11) * _e15));
    return;
}
"#;
    pub const SHADER_ENTRY_FILE: &str = include_file_path::include_file_path!(
        "../shaders/basic/main.wgsl"
    );
    pub const BINDINGS_FILE: &str = include_file_path::include_file_path!(
        "../shaders/basic/bindings.wgsl"
    );
    pub const TYPES_FILE: &str = include_file_path::include_file_path!(
        "../shaders/additional/types.wgsl"
    );
    pub const SHADER_FILES: &[&str] = &[SHADER_ENTRY_FILE, BINDINGS_FILE, TYPES_FILE];
    pub fn load_shader_modules_from_path(
        composer: &mut naga_oil::compose::Composer,
        shader_defs: &std::collections::HashMap<
            String,
            naga_oil::compose::ShaderDefValue,
        >,
    ) {
        composer
            .add_composable_module(naga_oil::compose::ComposableModuleDescriptor {
                source: &std::fs::read_to_string(BINDINGS_FILE).unwrap(),
                file_path: "../shaders/basic/bindings.wgsl",
                language: naga_oil::compose::ShaderLanguage::Wgsl,
                shader_defs: shader_defs.clone(),
                as_name: Some("bindings".into()),
                ..Default::default()
            })
            .expect("failed to add composer module");
        composer
            .add_composable_module(naga_oil::compose::ComposableModuleDescriptor {
                source: &std::fs::read_to_string(TYPES_FILE).unwrap(),
                file_path: "../shaders/additional/types.wgsl",
                language: naga_oil::compose::ShaderLanguage::Wgsl,
                shader_defs: shader_defs.clone(),
                as_name: Some("types".into()),
                ..Default::default()
            })
            .expect("failed to add composer module");
    }
    pub fn load_naga_module_from_path(
        composer: &mut naga_oil::compose::Composer,
        shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>,
    ) -> wgpu::naga::Module {
        composer
            .make_naga_module(naga_oil::compose::NagaModuleDescriptor {
                source: &std::fs::read_to_string(SHADER_ENTRY_FILE).unwrap(),
                file_path: "../shaders/basic/main.wgsl",
                shader_defs,
                ..Default::default()
            })
            .expect("failed to build naga module")
    }
    pub fn create_shader_module_from_path(
        device: &wgpu::Device,
        shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>,
    ) -> wgpu::ShaderModule {
        let mut composer = naga_oil::compose::Composer::default();
        load_shader_modules_from_path(&mut composer, &shader_defs);
        let module = load_naga_module_from_path(&mut composer, shader_defs);
        let info = wgpu::naga::valid::Validator::new(
                wgpu::naga::valid::ValidationFlags::empty(),
                wgpu::naga::valid::Capabilities::all(),
            )
            .validate(&module)
            .unwrap();
        let shader_string = wgpu::naga::back::wgsl::write_string(
                &module,
                &info,
                wgpu::naga::back::wgsl::WriterFlags::empty(),
            )
            .expect("failed to convert naga module to source");
        let source = std::borrow::Cow::Owned(shader_string);
        device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("main.wgsl"),
                source: wgpu::ShaderSource::Wgsl(source),
            })
    }
}
