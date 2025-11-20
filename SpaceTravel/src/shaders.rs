use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::matrix::multiply_matrix_vector4;
use crate::fragment::Fragment;

fn transform_normal(normal: &Vector3, model_matrix: &Matrix) -> Vector3 {
    // Convierte el normal a coordenadas homogéneas (añade coordenada w = 0.0)
    let normal_vec4 = Vector4::new(normal.x, normal.y, normal.z, 0.0);

    let transformed_normal_vec4 = multiply_matrix_vector4(model_matrix, &normal_vec4);

    // Convierte de vuelta a Vector3 y normaliza
    let mut transformed_normal = Vector3::new(
        transformed_normal_vec4.x,
        transformed_normal_vec4.y,
        transformed_normal_vec4.z,
    );
    
    transformed_normal.normalize();
    transformed_normal
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Convert vertex position to homogeneous coordinates (Vec4) by adding a w-component of 1.0
    let position_vec4 = Vector4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    // Apply Model transformation
    let world_position = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);

    // Apply View transformation (camera)
    let view_position = multiply_matrix_vector4(&uniforms.view_matrix, &world_position);

    // Apply Projection transformation (perspective)
    let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);

    // Perform perspective division to get NDC (Normalized Device Coordinates)
    let ndc = if clip_position.w != 0.0 {
        Vector3::new(
            clip_position.x / clip_position.w,
            clip_position.y / clip_position.w,
            clip_position.z / clip_position.w,
        )
    } else {
        Vector3::new(clip_position.x, clip_position.y, clip_position.z)
    };
    
    // Apply Viewport transformation to get screen coordinates
    let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
    let screen_position = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc_vec4);
    
    let transformed_position = Vector3::new(
        screen_position.x,
        screen_position.y,
        screen_position.z,
    );
    
    // Create a new Vertex with the transformed position
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal: transform_normal(&vertex.normal, &uniforms.model_matrix),
    }
}

// Función de ruido pseudoaleatorio mejorada para efectos más exóticos
fn exotic_noise(x: f32, y: f32, z: f32, time: f32, frequency: f32) -> f32 {
    let freq = frequency * 2.0;
    let n1 = (x * freq * 1.5 + time * 0.7).sin() * (y * freq + time * 0.5).cos() * (z * freq * 2.0 + time * 0.3).sin();
    let n2 = (x * freq * 3.0 + time * 1.2).cos() * (y * freq * 1.5 + time * 0.8).sin() * (z * freq + time * 1.1).cos();
    let n3 = (x * freq * 6.0 + time * 2.0).sin() * (y * freq * 4.0 + time * 1.5).cos() * (z * freq * 3.0 + time * 0.9).sin();
    
    // Combinar diferentes frecuencias para efecto más complejo
    (n1 * 0.5 + n2 * 0.3 + n3 * 0.2).abs()
}

// Shader simple para cualquier objeto que no tenga un shader específico
pub fn fragment_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    // Color gris simple para ahorrar recursos
    fragment.color
}

// Shader específico para el sol con efectos exóticos de energía cósmica
pub fn sun_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // Calcular ruido en múltiples escalas para efecto de energía cósmica
    let cosmic_energy = exotic_noise(pos.x, pos.y, pos.z, time, 3.0) * 0.8 +
                       exotic_noise(pos.x * 2.0, pos.y * 2.0, pos.z * 2.0, time + 100.0, 2.0) * 0.4 +
                       exotic_noise(pos.x * 4.0, pos.y * 4.0, pos.z * 4.0, time + 200.0, 1.5) * 0.2;
    
    // Efecto de pulsación multidimensional
    let pulsation = (time * 1.5).sin().abs() * 0.3 + (time * 2.2).cos().abs() * 0.2 + 0.5;
    
    // Efecto basado en la distancia desde el centro para simular capas exóticas
    let distance_from_center = pos.length();
    
    // Colores exóticos de energía cósmica
    let core_color = Vector3::new(1.0, 0.1, 0.8);      // Rosa neón central
    let surface_color = Vector3::new(0.2, 0.9, 1.0);   // Cian eléctrico
    let corona_color = Vector3::new(0.9, 1.0, 0.1);    // Amarillo neón
    
    // Determinar zona de la estrella basada en la distancia
    let zone_factor = if distance_from_center < 0.6 {
        0.0  // núcleo
    } else if distance_from_center < 0.85 {
        (distance_from_center - 0.6) / 0.25  // superficie
    } else {
        (distance_from_center - 0.85) / 0.15  // corona
    }.min(1.0);
    
    // Mezclar colores según la zona con transiciones no lineales
    let base_color = if zone_factor < 0.3 {
        let t = zone_factor * 3.33;
        core_color * (1.0 - t) + surface_color * t
    } else if zone_factor < 0.7 {
        let t = (zone_factor - 0.3) * 2.5;
        surface_color * (1.0 - t) + corona_color * t
    } else {
        corona_color
    };
    
    // Aplicar efectos de energía cósmica y pulsación
    let intensity = (cosmic_energy * 2.0 + pulsation) * 0.7;
    
    // Efecto de "explosiones" de energía aleatorias
    let energy_burst = exotic_noise(pos.x * 0.3, pos.y * 0.3, pos.z * 0.3, time * 3.0, 0.5);
    let burst_effect = (energy_burst * 3.0 + (time * 4.0).sin().abs() * 0.7).min(1.0);
    
    // Combinar todo para el color final con efectos de energía
    let final_color = base_color * intensity * (1.0 - burst_effect * 0.4) + 
                     Vector3::new(1.0, 1.0, 0.5) * burst_effect * 0.6;
    
    // Asegurar que los valores estén en el rango [0, 1]
    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// Shader para Mercurio con colores metálicos exóticos
pub fn mercury_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // Patrones complejos para superficie alienígena
    let crystal_pattern = exotic_noise(pos.x, pos.y, pos.z, time, 4.0);
    let metal_veins = exotic_noise(pos.x * 2.0, pos.y * 2.0, pos.z * 2.0, time + 50.0, 3.0);
    
    // Colores metálicos exóticos
    let base_metal = Vector3::new(0.2, 0.3, 0.2);      // Púrpura metálico
    let crystal_color = Vector3::new(0.4, 0.8, 0.9);   // Azul cristalino
    let vein_color = Vector3::new(0.9, 0.6, 0.3);      // Naranja metálico
    
    // Mezclar colores según patrones
    let crystal_factor = (crystal_pattern * 0.6 + 0.4).powf(1.5);
    let vein_factor = (metal_veins * 0.4 + 0.6).powf(2.0);
    
    let surface_color = base_metal * (1.0 - crystal_factor) + crystal_color * crystal_factor;
    let final_color = surface_color * (1.0 - vein_factor * 0.3) + vein_color * vein_factor * 0.3;
    
    // Efecto de reflexión iridiscente
    let iridescence = (pos.x * 8.0 + time * 2.0).sin().abs() * 0.2;
    let iridescent_color = final_color * (1.0 - iridescence) + Vector3::new(0.3, 0.9, 0.7) * iridescence;
    
    Vector3::new(
        iridescent_color.x.clamp(0.0, 1.0),
        iridescent_color.y.clamp(0.0, 1.0),
        iridescent_color.z.clamp(0.0, 1.0),
    )
}

// Shader para la Tierra con colores alienígenas
pub fn earth_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // Patrones alienígenas para continentes y océanos
    let continent_pattern = exotic_noise(pos.x, pos.y, pos.z, time, 2.0);
    let alien_rivers = exotic_noise(pos.x * 3.0, pos.y * 3.0, pos.z * 3.0, time + 30.0, 1.5);
    let bio_luminescence = exotic_noise(pos.x * 5.0, pos.y * 5.0, pos.z * 5.0, time * 2.0, 1.0);
    
    // Colores alienígenas exóticos
    let ocean_color = Vector3::new(0.1, 0.8, 0.6);     // Verde azulado fluorescente
    let land_color = Vector3::new(0.9, 0.4, 0.7);      // Rosa alienígena
    let river_color = Vector3::new(0.3, 0.9, 0.9);     // Cian brillante
    let bio_color = Vector3::new(0.8, 0.2, 0.9);       // Púrpura bioluminiscente
    
    // Determinar patrones
    let is_land = (continent_pattern * 0.8 + 0.2).max(0.0).min(1.0);
    let is_river = (alien_rivers * 0.5 + 0.5).max(0.0).min(1.0);
    let is_bio = (bio_luminescence * 0.7 + 0.3).max(0.0).min(1.0);
    
    // Mezclar colores base
    let base_color = ocean_color * (1.0 - is_land) + land_color * is_land;
    let with_rivers = base_color * (1.0 - is_river * 0.4) + river_color * is_river * 0.4;
    
    // Añadir bioluminiscencia que pulsa
    let bio_pulse = (time * 3.0).sin().abs() * 0.3 + 0.7;
    let final_color = with_rivers * (1.0 - is_bio * 0.2) + bio_color * is_bio * 0.2 * bio_pulse;
    
    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// Shader para Marte con colores de paisaje alienígena
pub fn mars_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // Patrones de terreno alienígena
    let desert_pattern = exotic_noise(pos.x, pos.y, pos.z, time, 2.5);
    let canyon_pattern = exotic_noise(pos.x * 1.5, pos.y * 1.5, pos.z * 1.5, time + 20.0, 2.0);
    let dust_storm = exotic_noise(pos.x * 0.5, pos.y * 0.5, pos.z * 0.5, time * 0.3, 0.8);
    
    // Colores de paisaje alienígena
    let base_color = Vector3::new(0.8, 0.2, 0.4);      // Rosa rojizo
    let canyon_color = Vector3::new(0.6, 0.8, 0.2);    // Verde amarillento
    let storm_color = Vector3::new(0.9, 0.7, 0.3);     // Amarillo dorado
    
    // Aplicar patrones
    let desert_factor = (desert_pattern * 0.7 + 0.3).powf(1.2);
    let canyon_factor = (canyon_pattern * 0.5 + 0.5).powf(1.5);
    let storm_factor = (dust_storm * 0.3 + 0.7).powf(0.8);
    
    let desert_surface = base_color * (1.0 - desert_factor) + canyon_color * desert_factor;
    let canyon_surface = desert_surface * (1.0 - canyon_factor * 0.4) + canyon_color * canyon_factor * 0.4;
    let final_color = canyon_surface * (1.0 - storm_factor * 0.2) + storm_color * storm_factor * 0.2;
    
    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// Shader para Urano con colores de gas nebular
pub fn uranus_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // Patrones de gas nebular
    let nebula_bands = exotic_noise(pos.x, pos.y, pos.z, time * 0.2, 1.2);
    let gas_vortices = exotic_noise(pos.x * 2.0, pos.y * 2.0, pos.z * 2.0, time * 0.5, 1.8);
    let energy_clouds = exotic_noise(pos.x * 0.7, pos.y * 0.7, pos.z * 0.7, time * 1.5, 0.9);
    
    // Colores de nebulosa
    let deep_nebula = Vector3::new(0.3, 0.1, 0.8);     // Azul profundo
    let vortex_color = Vector3::new(0.7, 0.3, 0.9);    // Púrpura vibrante
    let energy_color = Vector3::new(0.1, 0.9, 0.8);    // Verde azulado energético
    
    // Aplicar patrones nebulares
    let band_factor = (nebula_bands * 0.6 + 0.4).powf(1.3);
    let vortex_factor = (gas_vortices * 0.4 + 0.6).powf(1.7);
    let energy_factor = (energy_clouds * 0.5 + 0.5).powf(2.0);
    
    let banded_gas = deep_nebula * (1.0 - band_factor) + vortex_color * band_factor;
    let vortex_gas = banded_gas * (1.0 - vortex_factor * 0.3) + vortex_color * vortex_factor * 0.3;
    let final_color = vortex_gas * (1.0 - energy_factor * 0.4) + energy_color * energy_factor * 0.4;
    
    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// Shader para nave espacial con tecnología alienígena
pub fn nave_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // Patrones de tecnología alienígena
    let circuit_pattern = exotic_noise(pos.x, pos.y, pos.z, time * 2.0, 4.0);
    let energy_grid = exotic_noise(pos.x * 1.5, pos.y * 1.5, pos.z * 1.5, time * 1.2, 3.0);
    let hologram_effect = exotic_noise(pos.x * 0.8, pos.y * 0.8, pos.z * 0.8, time * 3.0, 2.0);
    
    // Colores de tecnología futurista
    let ship_base = Vector3::new(0.2, 0.1, 0.4);       // Púrpura oscuro
    let circuit_color = Vector3::new(0.1, 0.9, 0.6);   // Verde neón
    let energy_color = Vector3::new(0.8, 0.3, 0.9);    // Rosa eléctrico
    let hologram_color = Vector3::new(0.3, 0.7, 1.0);  // Azul holográfico
    
    // Aplicar patrones tecnológicos
    let circuit_factor = (circuit_pattern * 0.7 + 0.3).powf(2.0);
    let grid_factor = (energy_grid * 0.5 + 0.5).powf(1.8);
    let hologram_factor = (hologram_effect * 0.4 + 0.6).powf(1.5);
    
    let base_with_circuits = ship_base * (1.0 - circuit_factor) + circuit_color * circuit_factor;
    let with_energy_grid = base_with_circuits * (1.0 - grid_factor * 0.3) + energy_color * grid_factor * 0.3;
    let final_color = with_energy_grid * (1.0 - hologram_factor * 0.2) + hologram_color * hologram_factor * 0.2;
    
    // Efecto de pulsación de energía
    let energy_pulse = (time * 4.0).sin().abs() * 0.4 + 0.6;
    let pulsed_color = final_color * energy_pulse;
    
    Vector3::new(
        pulsed_color.x.clamp(0.0, 1.0),
        pulsed_color.y.clamp(0.0, 1.0),
        pulsed_color.z.clamp(0.0, 1.0),
    )
}

// Shader para Zephyr con colores de tormenta de cristal
pub fn zephyr_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // Patrones de tormenta de cristal
    let crystal_storm = exotic_noise(pos.x, pos.y, pos.z, time * 1.5, 2.5);
    let wind_currents = exotic_noise(pos.x * 1.8, pos.y * 1.8, pos.z * 1.8, time * 0.8, 2.2);
    let electric_arcs = exotic_noise(pos.x * 0.6, pos.y * 0.6, pos.z * 0.6, time * 2.5, 1.5);
    
    // Colores de tormenta exótica
    let storm_base = Vector3::new(0.1, 0.3, 0.7);      // Azul eléctrico
    let crystal_color = Vector3::new(0.4, 0.9, 0.8);   // Verde azulado cristalino
    let electric_color = Vector3::new(0.9, 0.5, 1.0);  // Rosa eléctrico
    
    let storm_factor = (crystal_storm * 0.8 + 0.2).powf(1.4);
    let wind_factor = (wind_currents * 0.6 + 0.4).powf(1.6);
    let electric_factor = (electric_arcs * 0.4 + 0.6).powf(2.2);
    
    let stormy_sky = storm_base * (1.0 - storm_factor) + crystal_color * storm_factor;
    let with_winds = stormy_sky * (1.0 - wind_factor * 0.3) + crystal_color * wind_factor * 0.3;
    let final_color = with_winds * (1.0 - electric_factor * 0.5) + electric_color * electric_factor * 0.5;
    
    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// Shader para Pyrion con colores de volcanes de azufre
pub fn pyrion_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // Patrones de volcanes exóticos
    let sulfur_flows = exotic_noise(pos.x, pos.y, pos.z, time * 0.7, 2.0);
    let volcanic_cracks = exotic_noise(pos.x * 2.2, pos.y * 2.2, pos.z * 2.2, time * 1.1, 1.8);
    let magma_pools = exotic_noise(pos.x * 0.9, pos.y * 0.9, pos.z * 0.9, time * 0.5, 1.3);
    
    // Colores de volcanes alienígenas
    let crust_color = Vector3::new(0.8, 0.6, 0.1);     // Amarillo sulfúrico
    let sulfur_color = Vector3::new(0.9, 0.8, 0.2);    // Amarillo brillante
    let magma_color = Vector3::new(1.0, 0.4, 0.1);     // Naranja incandescente
    let crack_color = Vector3::new(0.6, 0.3, 0.1);     // Marrón oscuro
    
    let sulfur_factor = (sulfur_flows * 0.7 + 0.3).powf(1.3);
    let crack_factor = (volcanic_cracks * 0.5 + 0.5).powf(1.8);
    let magma_factor = (magma_pools * 0.6 + 0.4).powf(2.0);
    
    let sulfur_surface = crust_color * (1.0 - sulfur_factor) + sulfur_color * sulfur_factor;
    let with_cracks = sulfur_surface * (1.0 - crack_factor * 0.4) + crack_color * crack_factor * 0.4;
    let final_color = with_cracks * (1.0 - magma_factor * 0.6) + magma_color * magma_factor * 0.6;
    
    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// Shader para Glacia con colores de hielo alienígena
pub fn glacia_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // Patrones de hielo exótico
    let alien_ice = exotic_noise(pos.x, pos.y, pos.z, time * 0.3, 2.8);
    let frozen_gas = exotic_noise(pos.x * 1.7, pos.y * 1.7, pos.z * 1.7, time * 0.9, 2.1);
    let crystal_growth = exotic_noise(pos.x * 0.8, pos.y * 0.8, pos.z * 0.8, time * 1.7, 1.4);
    
    // Colores de hielo alienígena
    let ice_base = Vector3::new(0.7, 0.9, 1.0);        // Azul hielo
    let alien_ice_color = Vector3::new(0.4, 0.8, 0.5); // Verde hielo
    let gas_color = Vector3::new(0.8, 0.5, 0.9);       // Púrpura congelado
    let crystal_color = Vector3::new(0.3, 0.7, 0.9);   // Azul cristal
    
    let ice_factor = (alien_ice * 0.6 + 0.4).powf(1.2);
    let gas_factor = (frozen_gas * 0.5 + 0.5).powf(1.5);
    let crystal_factor = (crystal_growth * 0.4 + 0.6).powf(1.9);
    
    let icy_surface = ice_base * (1.0 - ice_factor) + alien_ice_color * ice_factor;
    let with_gas = icy_surface * (1.0 - gas_factor * 0.3) + gas_color * gas_factor * 0.3;
    let final_color = with_gas * (1.0 - crystal_factor * 0.4) + crystal_color * crystal_factor * 0.4;
    
    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// Shader para Umbraleth con colores de energía oscura
pub fn umbraleth_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // Patrones de energía oscura y materia exótica
    let dark_energy = exotic_noise(pos.x, pos.y, pos.z, time * 0.4, 1.5);
    let void_vortices = exotic_noise(pos.x * 1.3, pos.y * 1.3, pos.z * 1.3, time * 0.6, 1.8);
    let quantum_fluctuations = exotic_noise(pos.x * 0.5, pos.y * 0.5, pos.z * 0.5, time * 2.0, 0.9);
    
    // Colores de energía oscura
    let void_color = Vector3::new(0.05, 0.02, 0.1);    // Negro violáceo
    let energy_color = Vector3::new(0.4, 0.1, 0.6);    // Púrpura oscuro energético
    let vortex_color = Vector3::new(0.2, 0.05, 0.4);   // Púrpura muy oscuro
    let quantum_color = Vector3::new(0.6, 0.2, 0.8);   // Púrpura brillante
    
    let energy_factor = (dark_energy * 0.5 + 0.5).powf(1.7);
    let vortex_factor = (void_vortices * 0.4 + 0.6).powf(2.0);
    let quantum_factor = (quantum_fluctuations * 0.3 + 0.7).powf(2.5);
    
    let energy_void = void_color * (1.0 - energy_factor) + energy_color * energy_factor;
    let with_vortices = energy_void * (1.0 - vortex_factor * 0.5) + vortex_color * vortex_factor * 0.5;
    let final_color = with_vortices * (1.0 - quantum_factor * 0.7) + quantum_color * quantum_factor * 0.7;
    
    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// Shader para Verdis con colores de bosque bioluminiscente
pub fn verdis_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // Patrones de flora y fauna alienígena
    let alien_flora = exotic_noise(pos.x, pos.y, pos.z, time * 0.8, 2.3);
    let bio_lights = exotic_noise(pos.x * 1.6, pos.y * 1.6, pos.z * 1.6, time * 1.4, 1.9);
    let fungal_networks = exotic_noise(pos.x * 0.7, pos.y * 0.7, pos.z * 0.7, time * 0.9, 1.2);
    
    // Colores de bosque alienígena
    let flora_base = Vector3::new(0.2, 0.7, 0.3);      // Verde alienígena
    let bio_color = Vector3::new(0.1, 0.9, 0.5);       // Verde neón
    let fungal_color = Vector3::new(0.8, 0.3, 0.6);    // Rosa micológico
    let light_color = Vector3::new(0.4, 1.0, 0.7);     // Verde brillante
    
    let flora_factor = (alien_flora * 0.7 + 0.3).powf(1.4);
    let bio_factor = (bio_lights * 0.6 + 0.4).powf(1.8);
    let fungal_factor = (fungal_networks * 0.5 + 0.5).powf(2.1);
    
    let forest_floor = flora_base * (1.0 - flora_factor) + bio_color * flora_factor;
    let with_lights = forest_floor * (1.0 - bio_factor * 0.4) + light_color * bio_factor * 0.4;
    let final_color = with_lights * (1.0 - fungal_factor * 0.3) + fungal_color * fungal_factor * 0.3;
    
    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}