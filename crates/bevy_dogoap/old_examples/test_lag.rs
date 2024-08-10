use std::time::Duration;

use bevy::{
    prelude::*,
    tasks::{futures_lite::future, AsyncComputeTaskPool, Task},
    time::common_conditions::on_timer,
};
use rand::Rng;

fn log_delta(time: Res<Time>) {
    println!("Delta: {}", time.delta_seconds());
}

#[derive(Component)]
struct ComputePlan(Task<u64>);

fn laggy_system(mut commands: Commands) {
    let thread_pool = AsyncComputeTaskPool::get();
    for _i in 0..10 {
        let task = thread_pool.spawn(async move {
            let mut rng = rand::thread_rng();
            let n = rng.gen_range(0..1000);
            std::thread::sleep(Duration::from_millis(n));
            n
        });
        commands.spawn(ComputePlan(task));
    }
}

fn handle_tasks(mut commands: Commands, mut query: Query<(Entity, &mut ComputePlan)>) {
    for (entity, mut task) in query.iter_mut() {
        if let Some(num) = future::block_on(future::poll_once(&mut task.0)) {
            println!("Result: {}", num);
            commands.entity(entity).remove::<ComputePlan>();
        }
    }
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(
            Update,
            laggy_system.run_if(on_timer(Duration::from_millis(500))),
        )
        .add_systems(
            FixedUpdate,
            log_delta.run_if(on_timer(Duration::from_millis(100))),
        )
        .add_systems(Update, handle_tasks)
        .run();
}
