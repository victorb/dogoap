// Self-replicating spaceships take over the world
// Two factions, who wins?

// Uses dogoap for each individual spaceship
// And also uses dogoap for the commanders for each faction,
// which sets the overall goal for the spaceships in their faction

// By default, the probe

// We start with a commander

// Its goal is to win by eliminating the other opponent
// It n

use std::f32::consts::PI;
use std::time::Duration;

use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::Rng;

#[derive(Component)]
struct Probe {
    current_steering_angle: f32,
    target_steering_angle: f32,
    steering_speed: f32,
    movement_speed: f32,
}

impl Default for Probe {
    fn default() -> Self {
        Self {
            current_steering_angle: 0.0,
            target_steering_angle: 0.0,
            steering_speed: 0.25,
            movement_speed: 32.0,
        }
    }
}


#[derive(Component, Default)]
struct ProbeDetection {
    detected_entities: Vec<(Entity, Vec2)>,
}



fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    for i in 0..10 {
        let pos_x = (i * 150) - 800;
        commands.spawn((
            Probe::default(),
            ProbeDetection::default(),
            Transform::from_xyz(pos_x as f32, 0.0, 0.0),
            GlobalTransform::default(),
        ));
    }
}

fn vec3_to_vec2(v: Vec3) -> Vec2 {
    Vec2::new(v.x, v.y)
}

fn draw_ui(mut gizmos: Gizmos, q_probes: Query<(Entity, &Transform, &ProbeDetection), With<Probe>>, ) {
    for (_entity, transform, detection) in q_probes.iter() {
        gizmos.circle_2d(vec3_to_vec2(transform.translation), 16., NAVY);

        for (_e, position) in &detection.detected_entities {
            gizmos.circle_2d(*position, 32., RED);
        }

        let character_position = vec3_to_vec2(transform.translation);

        let direction_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        let fov_radius: f32 = 150.0;
        let fov_angle: f32 = PI / 3.0;

        let start_angle = direction_angle - fov_angle / 2.0;
        let end_angle = direction_angle + fov_angle / 2.0;

        // Calculate the midpoint angle of the arc
        let midpoint_angle = (start_angle + end_angle) / 2.0;

        // Draw the arc using the calculated midpoint angle
        gizmos
            .arc_2d(
                character_position,
                midpoint_angle - PI / 2.0,
                fov_angle,
                fov_radius,
                LIGHT_BLUE,
            )
            .resolution(8);

        let start_point = character_position + Vec2::from_angle(start_angle) * fov_radius;
        let end_point = character_position + Vec2::from_angle(end_angle) * fov_radius;
        gizmos.line_2d(character_position, start_point, LIGHT_BLUE);
        gizmos.line_2d(character_position, end_point, LIGHT_BLUE);
    }
}

fn update_probe_steering(mut q_probes: Query<(&mut Probe, &Transform)>) {
    let mut rng = rand::thread_rng();
    for (mut probe, transform) in q_probes.iter_mut() {
        let center = Vec3::new(0.0, 0.0, 0.0);
        let direction_to_center = center - transform.translation;
        let angle_to_center = direction_to_center.y.atan2(direction_to_center.x);
        let bias_strength = 0.5; // Tune this parameter to increase or decrease the influence
        
        // Calculate the angle difference
        let mut angle_diff = angle_to_center - probe.target_steering_angle;
        
        // Normalize the angle difference to be between -PI and PI
        angle_diff = (angle_diff + PI) % (2.0 * PI) - PI;
        
        // Update the target steering angle
        probe.target_steering_angle += 
            rng.gen_range(-0.5..0.5) * (1.0 - bias_strength) + angle_diff * bias_strength;
        
        // Normalize the target steering angle to be between -PI and PI
        probe.target_steering_angle = (probe.target_steering_angle + PI) % (2.0 * PI) - PI;
    }
}

fn update_probe(mut q_probes: Query<(&mut Probe, &mut Transform)>, time: Res<Time>) {
    for (mut probe, mut transform) in q_probes.iter_mut() {
        // Update the current steering angle towards the target steering angle
        if probe.current_steering_angle < probe.target_steering_angle {
            probe.current_steering_angle = (probe.current_steering_angle
                + probe.steering_speed * time.delta_seconds())
            .min(probe.target_steering_angle);
        } else if probe.current_steering_angle > probe.target_steering_angle {
            probe.current_steering_angle = (probe.current_steering_angle
                - probe.steering_speed * time.delta_seconds())
            .max(probe.target_steering_angle);
        }

        // Update the probe's rotation based on the current steering angle
        transform.rotation = Quat::from_rotation_z(probe.current_steering_angle);

        // Move the probe forward based on its current steering angle
        let forward = transform.rotation * Vec3::X;
        transform.translation += forward * probe.movement_speed * time.delta_seconds();
    }
}

fn update_probe_detections(
    mut q_probes: Query<(Entity, &Transform, &mut ProbeDetection), With<Probe>>,
) {
    let probes: Vec<(Entity, Transform)> = q_probes.iter().map(|(e, t, _)| (e, t.clone())).collect();

    for (entity, transform, mut detection) in q_probes.iter_mut() {
        detection.detected_entities.clear();

        let character_position = vec3_to_vec2(transform.translation);
        let direction_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        let fov_radius: f32 = 150.0;
        let fov_angle: f32 = PI / 3.0;

        for (other_entity, other_transform) in &probes {
            if *other_entity != entity {
                let other_position = vec3_to_vec2(other_transform.translation);
                if is_point_in_cone(
                    other_position,
                    character_position,
                    direction_angle,
                    fov_radius,
                    fov_angle,
                ) {
                    detection.detected_entities.push((*other_entity, other_position));
                }
            }
        }
    }
}

fn is_point_in_cone(
    point: Vec2,
    cone_origin: Vec2,
    cone_direction: f32,
    cone_radius: f32,
    cone_angle: f32,
) -> bool {
    let to_point = point - cone_origin;
    let distance = to_point.length();

    // Check if the point is within the radius
    if distance > cone_radius {
        return false;
    }

    // Check if the point is within the angle
    let angle_to_point = to_point.y.atan2(to_point.x);
    let angle_diff = (angle_to_point - cone_direction).abs();
    let half_cone_angle = cone_angle / 2.0;

    angle_diff <= half_cone_angle || angle_diff >= (2.0 * PI - half_cone_angle)
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, (update_probe, update_probe_detections, draw_ui))
        .add_systems(Update, update_probe_steering.run_if(on_timer(Duration::from_millis(3000))))
        .run();
}
