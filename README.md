# cuda-thermal-sim

GPU-accelerated thermal simulation for chip floorplan validation.

## Features
- 2D/3D finite-difference heat diffusion
- Configurable power sources, thermal vias, boundary conditions
- ASCII thermal map generation
- Hotspot detection

## CUDA Parallelism
Each grid cell updates independently → trivial GPU parallelism.
3D simulation with 1000x1000x10 grid: ~100M cells on GPU.