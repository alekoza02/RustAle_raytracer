Progetto : Raytracer in Rust

Supporta:

- Geometrie:
  - Sfere
 
- Hardware:
  - CPU : multithread
 
- Materiali:
  - Diffuse
  - Specular
  - Glossiness
  - Glass
  - Light source

- Camera:
  - Movable
  - Antialiasing
  - FOV aggiustabile
  - Depth of field

- Post processing:
  - Tone mapping (sqrt)
 
- Algoritmi:
  - Monte carlo's sample generazione
  - Riflettanza del vetro usando la legge di Brew

- Modalità di renderizzazione:
  - Standard completa

----------------------------------------------------------------------------------------

Da aggiungere:

- Geometrie:
  - Parallelepipedi
  - Triangoli
  - Prismi triangolari

- Hardware:
  - GPU

- Materiali:
  - Subsurface
  - Volumes
  - TEXTURES
  - HDRI

- Camera:
  - Motion Blur

- Post processing:
  - Denoising
  - Bloom
  - Lens distorsion
 
- Algoritmi:
  - Blue noise
  - BVH
  - Direct light sample
  - Adaptive sampling

- Modalità di renderizzazione:
  - Ambient Occlusion
  - Depth Map
  - Normal Map
  - Sample map
  - Real Time Style -> blender's default


![rust_parallelo_tracer_high](https://github.com/alekoza02/RustAle_raytracer/assets/125405005/edd3c870-221a-4201-822f-497dda251281)
![test_roughness](https://github.com/alekoza02/RustAle_raytracer/assets/125405005/34215bd9-5ec5-4a40-9836-8f1613fecb4a)
![test_glossiness](https://github.com/alekoza02/RustAle_raytracer/assets/125405005/347262bb-c696-455a-ab83-8c2f703c37b6)
![test_depth_field](https://github.com/alekoza02/RustAle_raytracer/assets/125405005/0f4b97b5-954d-4074-9875-07517d198c02)


