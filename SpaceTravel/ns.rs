// src/main.rs
mod framebuffer;
mod triangle;
mod obj;
mod matrix;
mod fragment;
mod vertex;
mod camera;
mod shaders;
mod light;

use triangle::triangle;
use obj::Obj;
use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;
use matrix::{create_model_matrix, create_projection_matrix, create_viewport_matrix, multiply_matrix_vector4};
use vertex::Vertex;
use camera::Camera;
use shaders::{vertex_shader, fragment_shader, mercury_fragment_shader, sun_fragment_shader, earth_fragment_shader, mars_fragment_shader, nave_fragment_shader, zephyr_fragment_shader, pyrion_fragment_shader, glacia_fragment_shader, umbraleth_fragment_shader, verdis_fragment_shader};
use light::Light;

pub struct Uniforms {
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub time: f32, // elapsed time in seconds
    pub dt: f32, // delta time in seconds
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], light: &Light, planet_type: &str) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], light));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let final_color = match planet_type {
            "Voidheart" => umbraleth_fragment_shader(&fragment, uniforms), // Reutiliza shader oscuro o crea uno nuevo para rojo fuerte
            "Zephyr" => zephyr_fragment_shader(&fragment, uniforms),
            "Pyrion" => pyrion_fragment_shader(&fragment, uniforms),
            "Glacia" => glacia_fragment_shader(&fragment, uniforms),
            "Umbraleth" => umbraleth_fragment_shader(&fragment, uniforms),
            "Verdis" => verdis_fragment_shader(&fragment, uniforms),
            "Crystallos" => earth_fragment_shader(&fragment, uniforms), // Reutilizar o crear uno nuevo
            "Vulcanus" => mars_fragment_shader(&fragment, uniforms), // Reutilizar o crear uno nuevo
            "Lunaris" => mercury_fragment_shader(&fragment, uniforms), // Reutilizar o crear uno nuevo
            "Stellaris" => sun_fragment_shader(&fragment, uniforms), // Reutilizar o crear uno nuevo para verde radioactivo
            "Nave" => nave_fragment_shader(&fragment, uniforms),
            _ => fragment_shader(&fragment, uniforms), // Default
        };
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            final_color, //poner fragment.color si no se quiere nada de shading 
            fragment.depth,
        );
    }
}

// Función para dibujar una órbita circular en 3D
fn draw_orbit_3d(framebuffer: &mut Framebuffer, orbit_radius: f32, orbit_color: Color, view_matrix: &Matrix, projection_matrix: &Matrix, viewport_matrix: &Matrix, center_offset: Option<Vector3>) {
    let segments = 128; // Aumentamos el número de segmentos para una línea más suave
    let angle_increment = 2.0 * PI / segments as f32;
    // Crear un vértice temporal para transformar puntos
    let mut prev_x = 0;
    let mut prev_y = 0;
    let mut first_point = true;
    // Guardar el primer punto para cerrar el círculo
    let mut first_x = 0;
    let mut first_y = 0;

    let center = center_offset.unwrap_or(Vector3::zero());

    for i in 0..segments {
        let angle = i as f32 * angle_increment;
        // Punto en el círculo (en el plano XZ, Y=0) relativo al centro
        let x_rel = angle.cos() * orbit_radius;
        let y_rel = 0.0; // En el plano XZ
        let z_rel = angle.sin() * orbit_radius;

        let x = center.x + x_rel;
        let y = center.y + y_rel;
        let z = center.z + z_rel;

        // Transformar el punto a coordenadas de pantalla
        let position_vec4 = Vector4::new(x, y, z, 1.0);
        // Aplicar transformaciones
        let view_position = multiply_matrix_vector4(view_matrix, &position_vec4);
        let clip_position = multiply_matrix_vector4(projection_matrix, &view_position);
        // Perspectiva division
        let ndc = if clip_position.w != 0.0 {
            Vector3::new(
                clip_position.x / clip_position.w,
                clip_position.y / clip_position.w,
                clip_position.z / clip_position.w,
            )
        } else {
            Vector3::new(clip_position.x, clip_position.y, clip_position.z)
        };
        // Aplicar matriz de viewport
        let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
        let screen_position = multiply_matrix_vector4(viewport_matrix, &ndc_vec4);
        let screen_x = screen_position.x as i32;
        let screen_y = screen_position.y as i32;

        // Guardar el primer punto
        if i == 0 {
            first_x = screen_x;
            first_y = screen_y;
        }

        // Dibujar línea desde el punto anterior al actual
        if !first_point {
            // Dibujar la línea con una profundidad mayor (más lejos) que los planetas
            framebuffer.draw_line_with_depth(prev_x, prev_y, screen_x, screen_y, orbit_color, 1000.0);
        } else {
            first_point = false;
        }

        prev_x = screen_x;
        prev_y = screen_y;
    }
    // Cerrar el círculo conectando el último punto con el primero
    if segments > 0 {
        framebuffer.draw_line_with_depth(prev_x, prev_y, first_x, first_y, orbit_color, 1000.0);
    }
}

#[derive(Clone)]
struct CelestialBody {
    name: String,
    translation: Vector3,
    scale: f32,
    rotation: Vector3,
    orbit_radius: f32,
    orbit_speed: f32,
    rotation_speed: f32,
    color: Color,
}

// Función para verificar colisión entre dos esferas
fn check_collision(pos1: Vector3, radius1: f32, pos2: Vector3, radius2: f32) -> bool {
    let distance = ((pos1.x - pos2.x).powi(2) + (pos1.y - pos2.y).powi(2) + (pos1.z - pos2.z).powi(2)).sqrt();
    distance < (radius1 + radius2)
}

// Función para evitar colisiones
fn avoid_collision(camera_pos: Vector3, target_pos: Vector3, celestial_bodies: &[CelestialBody], time: f32) -> (Vector3, Vector3) {
    let mut new_camera_pos = camera_pos;
    let mut new_target_pos = target_pos;

    // Verificar colisiones con cada cuerpo celeste
    for body in celestial_bodies {
        let body_pos = if body.orbit_radius > 0.0 {
            // Calcular posición actual del cuerpo en su órbita
            let x = (time * body.orbit_speed).cos() * body.orbit_radius;
            let z = (time * body.orbit_speed).sin() * body.orbit_radius;
            Vector3::new(x, 0.0, z)
        } else {
            // Posición fija
            body.translation
        };

        // Calcular radios efectivos (considerando el tamaño del cuerpo)
        let camera_radius = 2.0; // Radio de colisión de la cámara
        let body_radius = body.scale * 0.8; // Radio de colisión del cuerpo celeste

        // Verificar si hay colisión con la cámara
        if check_collision(new_camera_pos, camera_radius, body_pos, body_radius) {
            // Calcular vector de separación
            let diff_x = new_camera_pos.x - body_pos.x;
            let diff_y = new_camera_pos.y - body_pos.y;
            let diff_z = new_camera_pos.z - body_pos.z;
            let distance = (diff_x.powi(2) + diff_y.powi(2) + diff_z.powi(2)).sqrt();
            if distance > 0.0 {
                // Normalizar el vector de separación
                let norm_x = diff_x / distance;
                let norm_y = diff_y / distance;
                let norm_z = diff_z / distance;

                // Calcular nueva posición para evitar la colisión
                let min_distance = body_radius + camera_radius;
                new_camera_pos.x = body_pos.x + norm_x * min_distance;
                new_camera_pos.y = body_pos.y + norm_y * min_distance;
                new_camera_pos.z = body_pos.z + norm_z * min_distance;
            }
        }

        // Verificar si hay colisión con el punto de mira
        if check_collision(new_target_pos, camera_radius, body_pos, body_radius) {
            // Calcular vector de separación
            let diff_x = new_target_pos.x - body_pos.x;
            let diff_y = new_target_pos.y - body_pos.y;
            let diff_z = new_target_pos.z - body_pos.z;
            let distance = (diff_x.powi(2) + diff_y.powi(2) + diff_z.powi(2)).sqrt();
            if distance > 0.0 {
                // Normalizar el vector de separación
                let norm_x = diff_x / distance;
                let norm_y = diff_y / distance;
                let norm_z = diff_z / distance;

                // Calcular nueva posición para evitar la colisión
                let min_distance = body_radius + camera_radius;
                new_target_pos.x = body_pos.x + norm_x * min_distance;
                new_target_pos.y = body_pos.y + norm_y * min_distance;
                new_target_pos.z = body_pos.z + norm_z * min_distance;
            }
        }
    }

    (new_camera_pos, new_target_pos)
}

fn main() {
    let window_width = 1280;
    let window_height = 720;
    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Proyecto 3 - Graficas - Sistema Xerion")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width, window_height);

    // Posición inicial de la cámara
    let initial_camera_pos = Vector3::new(0.0, 20.0, 75.0);
    let initial_camera_target = Vector3::new(0.0, 0.0, 0.0);
    let initial_camera_up = Vector3::new(0.0, 1.0, 0.0);

    // Inicializar cámara
    let mut camera = Camera::new(
        initial_camera_pos,
        initial_camera_target,
        initial_camera_up,
    );

    // Light (Usamos Voidheart como fuente de luz central)
    let light = Light::new(Vector3::new(0.0, 0.0, 0.0)); // Posición del Voidheart

    let obj = Obj::load("./models/sphere.obj").expect("Failed to load obj");
    let vertex_array = obj.get_vertex_array();

    // Cargar la nave espacial
    let nave_obj = Obj::load("./models/nave.obj").expect("Failed to load nave.obj");
    let nave_vertex_array = nave_obj.get_vertex_array();

    framebuffer.set_background_color(Color::new(35, 35, 40, 255));    // --- DEFINICIÓN DE 10 CUERPOS CELESTES FICTICIOS ---
    
    let voidheart = CelestialBody {
        name: "Voidheart".to_string(), // Singularidad/objeto central oscuro -> ROJO FUERTE
        translation: Vector3::new(0.0, 0.0, 0.0), // Posición central
        scale: 15.0,
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 0.0,
        orbit_speed: 0.0,
        rotation_speed: 0.1,
        color: Color::new(255, 50, 50, 255), // Rojo fuerte
    };

    let zephyr = CelestialBody {
        name: "Zephyr".to_string(), // Planeta azulado con vientos
        translation: Vector3::new(0.0, 0.0, 0.0),
        scale: 4.0,
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 20.0, // Distancia desde la estrella central
        orbit_speed: 0.6,   // Velocidad orbital
        rotation_speed: 1.8, // Velocidad de rotación
        color: Color::new(100, 150, 255, 255), // Azul claro
    };

    let pyrion = CelestialBody {
        name: "Pyrion".to_string(), // Planeta rojo ardiente
        translation: Vector3::new(0.0, 0.0, 0.0),
        scale: 3.5,
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 28.0,
        orbit_speed: 0.4,
        rotation_speed: 1.3,
        color: Color::new(255, 100, 50, 255), // Rojo anaranjado
    };

    let glacia = CelestialBody {
        name: "Glacia".to_string(), // Planeta helado
        translation: Vector3::new(0.0, 0.0, 0.0),
        scale: 3.0,
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 38.0,
        orbit_speed: 0.25,
        rotation_speed: 1.0,
        color: Color::new(200, 230, 255, 255), // Blanco azulado
    };

    let umbraleth = CelestialBody {
        name: "Umbraleth".to_string(), // Planeta oscuro
        translation: Vector3::new(0.0, 0.0, 0.0),
        scale: 5.5,
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 48.0,
        orbit_speed: 0.15,
        rotation_speed: 0.7,
        color: Color::new(50, 30, 80, 255), // Morado oscuro
    };

    let verdis = CelestialBody {
        name: "Verdis".to_string(), // Planeta verde boscoso
        translation: Vector3::new(0.0, 0.0, 0.0),
        scale: 3.2,
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 58.0,
        orbit_speed: 0.12,
        rotation_speed: 1.1,
        color: Color::new(50, 200, 100, 255), // Verde
    };

    let crystallos = CelestialBody {
        name: "Crystallos".to_string(), // Planeta cristalino
        translation: Vector3::new(0.0, 0.0, 0.0),
        scale: 2.8,
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 68.0,
        orbit_speed: 0.10,
        rotation_speed: 1.4,
        color: Color::new(180, 220, 255, 255), // Azul claro brillante
    };

    let vulcanus = CelestialBody {
        name: "Vulcanus".to_string(), // Luna volcánica de Umbraleth
        translation: Vector3::new(0.0, 0.0, 0.0),
        scale: 1.5,
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 6.0, // Orbita alrededor de Umbraleth
        orbit_speed: 1.0,
        rotation_speed: 2.0,
        color: Color::new(220, 80, 40, 255), // Rojo intenso
    };

    let lunaris = CelestialBody {
        name: "Lunaris".to_string(), // Luna de Glacia
        translation: Vector3::new(0.0, 0.0, 0.0),
        scale: 1.2,
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 4.5, // Orbita alrededor de Glacia
        orbit_speed: 1.2,
        rotation_speed: 1.5,
        color: Color::new(230, 240, 250, 255), // Blanco puro
    };

    let stellaris = CelestialBody {
        name: "Stellaris".to_string(), // Estrella secundaria (menor) -> VERDE RADIOACTIVO
        translation: Vector3::new(10.0, 0.0, 10.0), // Posición fija relativa al centro
        scale: 8.0,
        rotation: Vector3::new(0.0, 0.0, 0.0),
        orbit_radius: 0.0,  // No orbita en torno al Sol principal
        orbit_speed: 0.0,
        rotation_speed: 0.3,
        color: Color::new(50, 255, 50, 255), // Verde radioactivo
    };

    // Vector con todos los 10 cuerpos celestes
    let celestial_bodies = vec![
        voidheart.clone(), zephyr.clone(), pyrion.clone(), glacia.clone(),
        umbraleth.clone(), verdis.clone(), crystallos.clone(), vulcanus.clone(),
        lunaris.clone(), stellaris.clone()
    ];

    // Vector con los cuerpos elegidos para warp (5 de los 10)
    let warp_bodies = vec![zephyr.clone(), pyrion.clone(), glacia.clone(), umbraleth.clone(), verdis.clone()];

    let mut time = 0.0;

    while !window.window_should_close() {
        let dt = window.get_frame_time();
        time += dt;

        // Procesar entrada de cámara con movimiento 3D
        camera.process_input(&window);

        // Verificar colisiones y ajustar la posición de la cámara si es necesario
        let (adjusted_eye, adjusted_target) = avoid_collision(camera.eye, camera.target, &celestial_bodies, time);
        camera.eye = adjusted_eye;
        camera.target = adjusted_target;

        framebuffer.clear();
        framebuffer.set_current_color(Color::new(0, 0, 0, 255));

        // Render each celestial body FIRST
        for mut body in celestial_bodies.clone() {
            // Calcular posición orbital y rotación
            if body.orbit_radius > 0.0 && body.name != "Vulcanus" && body.name != "Lunaris" {
                // Cuerpos que orbitan alrededor del Voidheart
                body.translation.x = (time * body.orbit_speed).cos() * body.orbit_radius;
                body.translation.z = (time * body.orbit_speed).sin() * body.orbit_radius;
            } else if body.name == "Vulcanus" {
                 // Vulcanus orbita alrededor de Umbraleth
                 let umbraleth_x = (time * umbraleth.orbit_speed).cos() * umbraleth.orbit_radius;
                 let umbraleth_z = (time * umbraleth.orbit_speed).sin() * umbraleth.orbit_radius;
                 let vulcanus_angle = time * vulcanus.orbit_speed;
                 body.translation.x = umbraleth_x + vulcanus_angle.cos() * vulcanus.orbit_radius;
                 body.translation.z = umbraleth_z + vulcanus_angle.sin() * vulcanus.orbit_radius;
            } else if body.name == "Lunaris" {
                 // Lunaris orbita alrededor de Glacia
                 let glacia_x = (time * glacia.orbit_speed).cos() * glacia.orbit_radius;
                 let glacia_z = (time * glacia.orbit_speed).sin() * glacia.orbit_radius;
                 let lunaris_angle = time * lunaris.orbit_speed;
                 body.translation.x = glacia_x + lunaris_angle.cos() * lunaris.orbit_radius;
                 body.translation.z = glacia_z + lunaris_angle.sin() * lunaris.orbit_radius;
            } // Stellaris y Voidheart tienen posición fija
            body.rotation.y += dt * body.rotation_speed;

            // Set color for the body
            framebuffer.set_current_color(body.color);

            // Crear matrices de transformación para este cuerpo celeste
            let model_matrix = create_model_matrix(
                body.translation,
                body.scale,
                body.rotation
            );
            let view_matrix = camera.get_view_matrix();
            let projection_matrix = create_projection_matrix(PI / 3.0, window_width as f32 / window_height as f32, 0.1, 100.0);
            let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

            // Crear uniforms
            let uniforms = Uniforms {
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                dt,
            };

            render(&mut framebuffer, &uniforms, &vertex_array, &light, &body.name);
        }

        // Crear matrices de transformación comunes
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(PI / 3.0, window_width as f32 / window_height as f32, 0.1, 100.0);
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

        // Dibujar las órbitas de los cuerpos que orbitan (orbit_radius > 0) en blanco AFTER rendering the planets
        for body in &celestial_bodies {
            if body.orbit_radius > 0.0 && body.name != "Vulcanus" && body.name != "Lunaris" {
                // Dibujar órbitas principales
                let orbit_color = Color::new(200, 200, 200, 50); // Gris claro para órbitas principales
                draw_orbit_3d(&mut framebuffer, body.orbit_radius, orbit_color, &view_matrix, &projection_matrix, &viewport_matrix, None);
            } else if body.name == "Umbraleth" {
                 // Dibujar órbita de Vulcanus alrededor de Umbraleth
                 let umbraleth_pos = Vector3::new(
                     (time * body.orbit_speed).cos() * body.orbit_radius,
                     0.0,
                     (time * body.orbit_speed).sin() * body.orbit_radius
                 );
                 let orbit_color = Color::new(255, 100, 100, 30); // Rojo claro para la luna
                 draw_orbit_3d(&mut framebuffer, vulcanus.orbit_radius, orbit_color, &view_matrix, &projection_matrix, &viewport_matrix, Some(umbraleth_pos));
            } else if body.name == "Glacia" {
                 // Dibujar órbita de Lunaris alrededor de Glacia
                 let glacia_pos = Vector3::new(
                     (time * body.orbit_speed).cos() * body.orbit_radius,
                     0.0,
                     (time * body.orbit_speed).sin() * body.orbit_radius
                 );
                 let orbit_color = Color::new(200, 220, 255, 30); // Azul claro para la luna
                 draw_orbit_3d(&mut framebuffer, lunaris.orbit_radius, orbit_color, &view_matrix, &projection_matrix, &viewport_matrix, Some(glacia_pos));
            }
        }

        // === NUEVA IMPLEMENTACIÓN DE LA NAVE HUD ===
        // Renderizar la nave espacial como elemento HUD 3D (siempre visible)
        {
            // Configuración de posición HUD - siempre frente a la cámara
            let hud_distance = 25.0; // Distancia fija desde la cámara
            
            // Calcular vectores de dirección de la cámara
            let forward_vec = Vector3::new(
                camera.target.x - camera.eye.x,
                camera.target.y - camera.eye.y,
                camera.target.z - camera.eye.z
            );
            let forward_len = (forward_vec.x * forward_vec.x + forward_vec.y * forward_vec.y + forward_vec.z * forward_vec.z).sqrt();
            let camera_forward = Vector3::new(
                forward_vec.x / forward_len,
                forward_vec.y / forward_len,
                forward_vec.z / forward_len
            );
            
            // Cross product: camera_forward x camera.up
            let right_vec = Vector3::new(
                camera_forward.y * camera.up.z - camera_forward.z * camera.up.y,
                camera_forward.z * camera.up.x - camera_forward.x * camera.up.z,
                camera_forward.x * camera.up.y - camera_forward.y * camera.up.x
            );
            let right_len = (right_vec.x * right_vec.x + right_vec.y * right_vec.y + right_vec.z * right_vec.z).sqrt();
            let camera_right = Vector3::new(
                right_vec.x / right_len,
                right_vec.y / right_len,
                right_vec.z / right_len
            );
            
            // Cross product: camera_right x camera_forward
            let up_vec = Vector3::new(
                camera_right.y * camera_forward.z - camera_right.z * camera_forward.y,
                camera_right.z * camera_forward.x - camera_right.x * camera_forward.z,
                camera_right.x * camera_forward.y - camera_right.y * camera_forward.x
            );
            let up_len = (up_vec.x * up_vec.x + up_vec.y * up_vec.y + up_vec.z * up_vec.z).sqrt();
            let camera_up_adjusted = Vector3::new(
                up_vec.x / up_len,
                up_vec.y / up_len,
                up_vec.z / up_len
            );
            
            // Offset en la pantalla (esquina inferior derecha)
            let screen_offset_right = 8.0;    // Más a la derecha
            let screen_offset_down = -6.0;    // Más abajo (valor negativo)
            let screen_offset_forward = hud_distance;
            
            // Posición base HUD (relativa a la cámara)
            let hud_base_position = Vector3::new(
                camera.eye.x + camera_forward.x * screen_offset_forward + camera_right.x * screen_offset_right + camera_up_adjusted.x * screen_offset_down,
                camera.eye.y + camera_forward.y * screen_offset_forward + camera_right.y * screen_offset_right + camera_up_adjusted.y * screen_offset_down,
                camera.eye.z + camera_forward.z * screen_offset_forward + camera_right.z * screen_offset_right + camera_up_adjusted.z * screen_offset_down
            );
            
            // Movimiento orbital pequeño para dar vida a la nave
            let nave_orbit_radius = 2.5;
            let nave_orbit_speed = 1.5;
            let nave_angle = time * nave_orbit_speed;
            
            // Offset de movimiento suave (flotación en el espacio)
            let orbit_offset = Vector3::new(
                (nave_angle * 0.7).cos() * nave_orbit_radius * 0.1,
                (nave_angle * 1.3).sin() * nave_orbit_radius * 0.15,
                (nave_angle * 0.9).sin() * nave_orbit_radius * 0.1
            );
            
            // Posición final de la nave
            let nave_position = Vector3::new(
                hud_base_position.x + orbit_offset.x,
                hud_base_position.y + orbit_offset.y,
                hud_base_position.z + orbit_offset.z
            );
            
            // Calcular rotación para que la nave mire en dirección general de la cámara
            let look_target = Vector3::new(
                camera.target.x + 5.0,
                camera.target.y,
                camera.target.z + 5.0
            );
            let look_vec = Vector3::new(
                look_target.x - nave_position.x,
                look_target.y - nave_position.y,
                look_target.z - nave_position.z
            );
            let look_len = (look_vec.x * look_vec.x + look_vec.y * look_vec.y + look_vec.z * look_vec.z).sqrt();
            let look_direction = Vector3::new(
                look_vec.x / look_len,
                look_vec.y / look_len,
                look_vec.z / look_len
            );
            
            // Calcular rotaciones en Y y X basadas en la dirección de mirada
            let rotation_y = look_direction.x.atan2(look_direction.z);
            let rotation_x = (-look_direction.y).asin().max(-0.3).min(0.3); // Limitar inclinación
            
            // Rotación adicional para efecto dinámico
            let additional_roll = (time * 0.5).sin() * 0.1;
            
            // Crear matriz de modelo para la nave HUD
            let nave_model_matrix = create_model_matrix(
                nave_position,
                0.08, // Escala más pequeña para HUD
                Vector3::new(rotation_x, rotation_y, additional_roll)
            );

            // Crear uniforms para la nave
            let nave_uniforms = Uniforms {
                model_matrix: nave_model_matrix,
                view_matrix: view_matrix.clone(),
                projection_matrix: projection_matrix.clone(),
                viewport_matrix: viewport_matrix.clone(),
                time,
                dt,
            };

            // Renderizar la nave con su shader específico
            render(&mut framebuffer, &nave_uniforms, &nave_vertex_array, &light, "Nave");
        }

        framebuffer.swap_buffers(&mut window, &raylib_thread);
        thread::sleep(Duration::from_millis(16));
    }
}