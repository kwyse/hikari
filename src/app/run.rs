use std::thread;
use std::time::{Duration, Instant};

const MINIMUM_FPS: u8 = 24;

/// Process a callback function at a desired frequency per second
pub struct ExecutionLoop {
    frame_interval: Duration,
    prev: Instant,
}

impl ExecutionLoop {
    /// Initializes the loop
    ///
    /// If the target FPS is too low, a minimum FPS value will be used.
    pub fn new(target_fps: u8) -> Self {
        let target_duration = (1_000.0_f32 / target_fps.max(MINIMUM_FPS) as f32) as u64;
        Self {
            frame_interval: Duration::from_millis(target_duration),
            prev: Instant::now(),
        }
    }

    /// Run custom logic in an infinite loop, yielding deltas between frames
    pub fn run<F>(&mut self, mut callback: F)
    where
        F: FnMut(Duration) -> ExecutionFlow,
    {
        loop {
            let now = Instant::now();
            if let FrameAction::Run(delta) = skip_or_run(now, self.prev, self.frame_interval) {
                match callback(delta) {
                    ExecutionFlow::Continue => self.prev = now,
                    ExecutionFlow::Quit => break,
                }
            }
        }
    }
}

/// Signal returned by the callback function
pub enum ExecutionFlow {
    /// The program should continue to the next loop iteration
    Continue,

    /// The program should terminate after the current loop iteration
    Quit,
}

enum FrameAction {
    Run(Duration),
    Skip,
}

fn skip_or_run(now: Instant, prev: Instant, target: Duration) -> FrameAction {
    let delta = now - prev;

    if delta < target {
        thread::sleep(target - delta);
        FrameAction::Skip
    } else {
        FrameAction::Run(delta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_duration_is_set_to_target_when_above_minimum_fps() {
        let execution_loop = ExecutionLoop::new(200);
        assert_eq!(execution_loop.frame_interval, Duration::from_millis(1000 / 200));
    }

    #[test]
    fn frame_duration_falls_back_to_use_minimum_fps_if_target_is_too_large() {
        let execution_loop = ExecutionLoop::new(MINIMUM_FPS - 10);
        let frame_duration = (1000.0_f32 / (MINIMUM_FPS as f32)) as u64;
        assert_eq!(execution_loop.frame_interval, Duration::from_millis(frame_duration));
    }

    #[test]
    fn loop_exits_after_one_iteration_if_callback_instructs_it_to() {
        let mut execution_loop = ExecutionLoop::new(200);
        let mut loop_iterations = 0;

        execution_loop.run(|_| {
            loop_iterations += 1;
            ExecutionFlow::Quit
        });

        assert_eq!(loop_iterations, 1);
    }

    #[test]
    fn loop_iterates_if_callback_instructs_it_to() {
        let mut execution_loop = ExecutionLoop::new(200);
        let mut loop_iterations = 0;

        let prev_on_init = execution_loop.prev;

        execution_loop.run(|_| {
            loop_iterations += 1;
            if loop_iterations > 1 { ExecutionFlow::Quit } else { ExecutionFlow::Continue }
        });

        assert_eq!(loop_iterations, 2);
        assert!(execution_loop.prev > prev_on_init);
    }
}
