//! System data collection via `sysinfo`.

use std::time::{SystemTime, UNIX_EPOCH};

/// A snapshot of system state taken at one refresh cycle.
#[derive(Debug, Clone)]
pub struct SystemSnapshot {
    /// Overall CPU usage percentage (0.0–100.0).
    pub cpu_usage_pct: f64,
    /// Used memory in GB.
    pub memory_used_gb: f64,
    /// Total memory in GB.
    pub memory_total_gb: f64,
    /// Memory usage percentage (0.0–100.0).
    pub memory_usage_pct: f64,
    /// Disk usage percentage (0.0–100.0).
    pub disk_usage_pct: f64,
    /// Last 60 CPU readings for the sparkline.
    pub cpu_history: Vec<f64>,
    /// Top processes by CPU usage.
    pub processes: Vec<ProcessInfo>,
    /// Human-readable timestamp of this snapshot.
    pub timestamp: String,
}

/// Information about a single running process.
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    /// Process ID.
    pub pid: String,
    /// Process name.
    pub name: String,
    /// CPU usage percentage.
    pub cpu_usage_pct: f64,
    /// Memory usage percentage.
    pub memory_usage_pct: f64,
    /// Memory usage in megabytes.
    pub memory_mb: f64,
    /// Process status string.
    pub status: String,
}

impl SystemSnapshot {
    /// Creates an empty snapshot.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cpu_usage_pct: 0.0,
            memory_used_gb: 0.0,
            memory_total_gb: 0.0,
            memory_usage_pct: 0.0,
            disk_usage_pct: 0.0,
            cpu_history: Vec::new(),
            processes: Vec::new(),
            timestamp: String::new(),
        }
    }

    /// Refreshes system data.
    pub fn refresh(&mut self, demo_mode: bool) {
        if demo_mode {
            self.refresh_demo();
        } else {
            self.refresh_real();
        }
        self.timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }

    fn refresh_real(&mut self) {
        use sysinfo::System;

        let mut sys = System::new_all();
        sys.refresh_all();

        // CPU
        let cpu_pct = sys.global_cpu_usage();
        self.cpu_usage_pct = cpu_pct as f64;
        self.cpu_history.push(self.cpu_usage_pct);
        if self.cpu_history.len() > 60 {
            self.cpu_history.remove(0);
        }

        // Memory
        let total = sys.total_memory();
        let used = sys.used_memory();
        self.memory_total_gb = total as f64 / 1_073_741_824.0;
        self.memory_used_gb = used as f64 / 1_073_741_824.0;
        self.memory_usage_pct = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        // Disk
        let total_disk: u64 = sysinfo::Disks::new_with_refreshed_list()
            .iter()
            .map(|d| d.total_space())
            .sum();
        let used_disk: u64 = sysinfo::Disks::new_with_refreshed_list()
            .iter()
            .map(|d| d.total_space() - d.available_space())
            .sum();
        self.disk_usage_pct = if total_disk > 0 {
            (used_disk as f64 / total_disk as f64) * 100.0
        } else {
            0.0
        };

        // Processes
        let mut procs: Vec<ProcessInfo> = sys
            .processes()
            .iter()
            .map(|(pid, p)| {
                let mem_bytes = p.memory();
                let mem_total = sys.total_memory().max(1);
                ProcessInfo {
                    pid: pid.as_u32().to_string(),
                    name: p.name().to_string_lossy().into_owned(),
                    cpu_usage_pct: p.cpu_usage() as f64,
                    memory_usage_pct: (mem_bytes as f64 / mem_total as f64) * 100.0,
                    memory_mb: mem_bytes as f64 / 1_048_576.0,
                    status: format!("{:?}", p.status()),
                }
            })
            .collect();
        procs.sort_by(|a, b| b.cpu_usage_pct.partial_cmp(&a.cpu_usage_pct).unwrap_or(std::cmp::Ordering::Equal));
        procs.truncate(50);
        self.processes = procs;
    }

    #[allow(clippy::cast_precision_loss, reason = "Timestamps are within u64 range for realistic values")]
    fn refresh_demo(&mut self) {
        // Oscillate CPU 30-65%
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        let base = 45.0 + (now * 0.5).sin() * 15.0;
        self.cpu_usage_pct = (base * 10.0).round() / 10.0;
        self.cpu_history.push(self.cpu_usage_pct);
        if self.cpu_history.len() > 60 {
            self.cpu_history.remove(0);
        }

        self.memory_total_gb = 32.0;
        self.memory_used_gb = 20.1;
        self.memory_usage_pct = 62.8;
        self.disk_usage_pct = 71.3;

        self.processes = vec![
            ProcessInfo { pid: "1234".into(), name: "firefox".into(), cpu_usage_pct: 12.3, memory_usage_pct: 8.7, memory_mb: 2784.3, status: "Running".into() },
            ProcessInfo { pid: "5678".into(), name: "cargo".into(), cpu_usage_pct: 8.9, memory_usage_pct: 2.1, memory_mb: 672.0, status: "Running".into() },
            ProcessInfo { pid: "9012".into(), name: "rust-analyzer".into(), cpu_usage_pct: 5.4, memory_usage_pct: 3.2, memory_mb: 1024.0, status: "Sleeping".into() },
            ProcessInfo { pid: "3456".into(), name: "terminal".into(), cpu_usage_pct: 2.1, memory_usage_pct: 1.5, memory_mb: 480.0, status: "Sleeping".into() },
            ProcessInfo { pid: "7890".into(), name: "spotify".into(), cpu_usage_pct: 1.8, memory_usage_pct: 4.3, memory_mb: 1376.0, status: "Sleeping".into() },
        ];
    }
}

impl Default for SystemSnapshot {
    fn default() -> Self {
        Self::new()
    }
}
