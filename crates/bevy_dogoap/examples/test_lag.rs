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
struct ComputePlan(Task<usize>);

#[derive(Resource, Default)]
struct Counter(usize);

fn laggy_system(mut commands: Commands, count: Res<Counter>) {
    let thread_pool = AsyncComputeTaskPool::get();
    // std::thread::sleep(Duration::from_millis(500));
    for i in 0..10 {
        let current_count = count.0;
        let task = thread_pool.spawn(async move {
            // println!("Sleeping...");
            // async_std::task::sleep(Duration::from_millis(100)).await;
            let mut rng = rand::thread_rng();
            let n = rng.gen_range(0..1000);
            std::thread::sleep(Duration::from_millis(n));
            // count.0 += 1;
            current_count + 1
            // count.0
        });
        commands.spawn(ComputePlan(task));
    }
}

fn handle_tasks(mut commands: Commands, mut query: Query<(Entity, &mut ComputePlan)>, mut counter: ResMut<Counter>) {
    for (entity, mut task) in query.iter_mut() {
        if let Some(num) = future::block_on(future::poll_once(&mut task.0)) {
            counter.0 += num;
            println!("Result: {}", counter.0);
            commands.entity(entity).remove::<ComputePlan>();
        }
    }
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .init_resource::<Counter>()
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
