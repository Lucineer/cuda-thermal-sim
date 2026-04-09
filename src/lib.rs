//! cuda-thermal-sim — GPU thermal simulation

#[derive(Debug, Clone, Copy)]
pub struct Cell { pub temp: f64, pub power_mw: f64, pub is_via: bool, pub is_boundary: bool }

#[derive(Debug)]
pub struct ThermalResult { pub peak: f64, pub avg: f64, pub iterations: u32, pub converged: bool }

pub struct ThermalSim { grid: Vec<Vec<Cell>>, pub ambient: f64, pub conductivity: f64, pub max_iter: u32, pub threshold: f64 }

impl ThermalSim {
    pub fn new(size: usize, ambient: f64) -> Self {
        ThermalSim { grid: vec![vec![Cell{temp:ambient, power_mw:0.0, is_via:false, is_boundary:false}; size]; size],
            ambient, conductivity: 150.0, max_iter: 10000, threshold: 0.01 }
    }
    pub fn add_power(&mut self, x: usize, y: usize, w: usize, h: usize, mw: f64) {
        for yi in y..(y+h).min(self.grid.len()) {
            for xi in x..(x+w).min(self.grid[0].len()) { self.grid[yi][xi].power_mw = mw/(w*h) as f64; }
        }
    }
    pub fn add_via(&mut self, x: usize, y: usize) {
        if y < self.grid.len() && x < self.grid[0].len() { self.grid[y][x].is_via = true; }
    }
    pub fn set_boundary(&mut self, y: usize, temp: f64) {
        if y < self.grid.len() { for c in &mut self.grid[y] { c.is_boundary = true; c.temp = temp; } }
    }
    pub fn simulate(&mut self) -> ThermalResult {
        let n = self.grid.len(); let dt = 0.01; let mut iters = 0u32; let mut max_delta = f64::MAX;
        for _ in 0..self.max_iter {
            let mut new = vec![vec![0.0f64;n];n]; max_delta = 0.0;
            for y in 0..n { for x in 0..n {
                let c = self.grid[y][x];
                if c.is_boundary { new[y][x] = c.temp; continue; }
                let k = if c.is_via { self.conductivity * 10.0 } else { self.conductivity };
                let mut nb = 0.0; let mut nc = 0;
                if x>0 { nb += self.grid[y][x-1].temp; nc+=1; }
                if x<n-1 { nb += self.grid[y][x+1].temp; nc+=1; }
                if y>0 { nb += self.grid[y-1][x].temp; nc+=1; }
                if y<n-1 { nb += self.grid[y+1][x].temp; nc+=1; }
                let avg = if nc>0 { nb/nc as f64 } else { self.ambient };
                let t = c.temp + k*dt*(avg-c.temp) + c.power_mw*1e-3*dt*100.0;
                new[y][x] = t.max(self.ambient);
                let d = (t-c.temp).abs(); if d>max_delta { max_delta=d; }
            }}
            for y in 0..n { for x in 0..n { self.grid[y][x].temp = new[y][x]; } }
            iters += 1; if max_delta < self.threshold { break; }
        }
        let temps: Vec<f64> = self.grid.iter().flat_map(|r| r.iter().map(|c| c.temp)).collect();
        ThermalResult { peak: temps.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            avg: temps.iter().sum::<f64>()/temps.len() as f64, iterations: iters, converged: max_delta < self.threshold }
    }
}

#[cfg(test)]
mod tests { use super::*;
    #[test] fn test_thermal() { let mut s = ThermalSim::new(20, 25.0); s.add_power(8,8,4,4,1000.0); s.set_boundary(0, 35.0); let r = s.simulate(); assert!(r.peak > 25.0); }
}
