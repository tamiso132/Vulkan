# Details

Date : 2023-11-19 14:58:46

Directory /home/tom/projects/rust/Vulkan

Total : 47 files,  17518 codes, 435 comments, 2624 blanks, all 20577 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [README.md](/README.md) | Markdown | 3 | 0 | 2 | 5 |
| [compile.sh](/compile.sh) | Shell Script | 2 | 0 | 0 | 2 |
| [src/buffer.rs](/src/buffer.rs) | Rust | 74 | 0 | 19 | 93 |
| [src/constant.rs](/src/constant.rs) | Rust | 26 | 1 | 7 | 34 |
| [src/device.rs](/src/device.rs) | Rust | 120 | 12 | 30 | 162 |
| [src/lib.rs](/src/lib.rs) | Rust | 189 | 13 | 30 | 232 |
| [src/main.rs](/src/main.rs) | Rust | 288 | 33 | 54 | 375 |
| [src/pipeline.rs](/src/pipeline.rs) | Rust | 163 | 3 | 47 | 213 |
| [src/platform.rs](/src/platform.rs) | Rust | 66 | 2 | 9 | 77 |
| [src/types.rs](/src/types.rs) | Rust | 7 | 3 | 4 | 14 |
| [src/utility.rs](/src/utility.rs) | Rust | 25 | 0 | 7 | 32 |
| [vulkan-tutorial-rust/README.md](/vulkan-tutorial-rust/README.md) | Markdown | 59 | 0 | 23 | 82 |
| [vulkan-tutorial-rust/azure-pipelines.yml](/vulkan-tutorial-rust/azure-pipelines.yml) | YAML | 32 | 41 | 10 | 83 |
| [vulkan-tutorial-rust/ci/azure-install-rust.yml](/vulkan-tutorial-rust/ci/azure-install-rust.yml) | YAML | 21 | 0 | 1 | 22 |
| [vulkan-tutorial-rust/ci/azure-steps.yml](/vulkan-tutorial-rust/ci/azure-steps.yml) | YAML | 6 | 13 | 4 | 23 |
| [vulkan-tutorial-rust/download_asset.py](/vulkan-tutorial-rust/download_asset.py) | Python | 27 | 5 | 11 | 43 |
| [vulkan-tutorial-rust/src/lib.rs](/vulkan-tutorial-rust/src/lib.rs) | Rust | 1 | 0 | 1 | 2 |
| [vulkan-tutorial-rust/src/tutorials/00_base_code.rs](/vulkan-tutorial-rust/src/tutorials/00_base_code.rs) | Rust | 48 | 1 | 13 | 62 |
| [vulkan-tutorial-rust/src/tutorials/01_instance_creation.rs](/vulkan-tutorial-rust/src/tutorials/01_instance_creation.rs) | Rust | 110 | 4 | 25 | 139 |
| [vulkan-tutorial-rust/src/tutorials/02_validation_layers.rs](/vulkan-tutorial-rust/src/tutorials/02_validation_layers.rs) | Rust | 232 | 10 | 42 | 284 |
| [vulkan-tutorial-rust/src/tutorials/03_physical_device_selection.rs](/vulkan-tutorial-rust/src/tutorials/03_physical_device_selection.rs) | Rust | 223 | 8 | 41 | 272 |
| [vulkan-tutorial-rust/src/tutorials/04_logical_device.rs](/vulkan-tutorial-rust/src/tutorials/04_logical_device.rs) | Rust | 214 | 6 | 39 | 259 |
| [vulkan-tutorial-rust/src/tutorials/05_window_surface.rs](/vulkan-tutorial-rust/src/tutorials/05_window_surface.rs) | Rust | 279 | 6 | 45 | 330 |
| [vulkan-tutorial-rust/src/tutorials/06_swap_chain_creation.rs](/vulkan-tutorial-rust/src/tutorials/06_swap_chain_creation.rs) | Rust | 493 | 7 | 80 | 580 |
| [vulkan-tutorial-rust/src/tutorials/07_image_view.rs](/vulkan-tutorial-rust/src/tutorials/07_image_view.rs) | Rust | 188 | 6 | 30 | 224 |
| [vulkan-tutorial-rust/src/tutorials/08_graphics_pipeline.rs](/vulkan-tutorial-rust/src/tutorials/08_graphics_pipeline.rs) | Rust | 153 | 6 | 27 | 186 |
| [vulkan-tutorial-rust/src/tutorials/09_shader_modules.rs](/vulkan-tutorial-rust/src/tutorials/09_shader_modules.rs) | Rust | 209 | 7 | 36 | 252 |
| [vulkan-tutorial-rust/src/tutorials/10_fixed_functions.rs](/vulkan-tutorial-rust/src/tutorials/10_fixed_functions.rs) | Rust | 316 | 16 | 47 | 379 |
| [vulkan-tutorial-rust/src/tutorials/11_render_passes.rs](/vulkan-tutorial-rust/src/tutorials/11_render_passes.rs) | Rust | 366 | 16 | 52 | 434 |
| [vulkan-tutorial-rust/src/tutorials/12_graphics_pipeline_complete.rs](/vulkan-tutorial-rust/src/tutorials/12_graphics_pipeline_complete.rs) | Rust | 357 | 16 | 49 | 422 |
| [vulkan-tutorial-rust/src/tutorials/13_framebuffers.rs](/vulkan-tutorial-rust/src/tutorials/13_framebuffers.rs) | Rust | 207 | 5 | 36 | 248 |
| [vulkan-tutorial-rust/src/tutorials/14_command_buffers.rs](/vulkan-tutorial-rust/src/tutorials/14_command_buffers.rs) | Rust | 278 | 5 | 45 | 328 |
| [vulkan-tutorial-rust/src/tutorials/15_hello_triangle.rs](/vulkan-tutorial-rust/src/tutorials/15_hello_triangle.rs) | Rust | 372 | 7 | 61 | 440 |
| [vulkan-tutorial-rust/src/tutorials/16_swap_chain_recreation.rs](/vulkan-tutorial-rust/src/tutorials/16_swap_chain_recreation.rs) | Rust | 359 | 9 | 55 | 423 |
| [vulkan-tutorial-rust/src/tutorials/17_vertex_input.rs](/vulkan-tutorial-rust/src/tutorials/17_vertex_input.rs) | Rust | 600 | 12 | 78 | 690 |
| [vulkan-tutorial-rust/src/tutorials/18_vertex_buffer.rs](/vulkan-tutorial-rust/src/tutorials/18_vertex_buffer.rs) | Rust | 760 | 16 | 111 | 887 |
| [vulkan-tutorial-rust/src/tutorials/19_staging_buffer.rs](/vulkan-tutorial-rust/src/tutorials/19_staging_buffer.rs) | Rust | 865 | 12 | 125 | 1,002 |
| [vulkan-tutorial-rust/src/tutorials/20_index_buffer.rs](/vulkan-tutorial-rust/src/tutorials/20_index_buffer.rs) | Rust | 820 | 12 | 116 | 948 |
| [vulkan-tutorial-rust/src/tutorials/21_descriptor_layout.rs](/vulkan-tutorial-rust/src/tutorials/21_descriptor_layout.rs) | Rust | 745 | 12 | 108 | 865 |
| [vulkan-tutorial-rust/src/tutorials/22_descriptor_sets.rs](/vulkan-tutorial-rust/src/tutorials/22_descriptor_sets.rs) | Rust | 850 | 12 | 122 | 984 |
| [vulkan-tutorial-rust/src/tutorials/23_texture_image.rs](/vulkan-tutorial-rust/src/tutorials/23_texture_image.rs) | Rust | 1,055 | 12 | 143 | 1,210 |
| [vulkan-tutorial-rust/src/tutorials/24_sampler.rs](/vulkan-tutorial-rust/src/tutorials/24_sampler.rs) | Rust | 914 | 12 | 125 | 1,051 |
| [vulkan-tutorial-rust/src/tutorials/25_texture_mapping.rs](/vulkan-tutorial-rust/src/tutorials/25_texture_mapping.rs) | Rust | 946 | 18 | 123 | 1,087 |
| [vulkan-tutorial-rust/src/tutorials/26_depth_buffering.rs](/vulkan-tutorial-rust/src/tutorials/26_depth_buffering.rs) | Rust | 1,080 | 14 | 135 | 1,229 |
| [vulkan-tutorial-rust/src/tutorials/27_model_loading.rs](/vulkan-tutorial-rust/src/tutorials/27_model_loading.rs) | Rust | 998 | 14 | 138 | 1,150 |
| [vulkan-tutorial-rust/src/tutorials/28_mipmapping.rs](/vulkan-tutorial-rust/src/tutorials/28_mipmapping.rs) | Rust | 1,197 | 14 | 159 | 1,370 |
| [vulkan-tutorial-rust/src/tutorials/29_multisampling.rs](/vulkan-tutorial-rust/src/tutorials/29_multisampling.rs) | Rust | 1,175 | 14 | 159 | 1,348 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)