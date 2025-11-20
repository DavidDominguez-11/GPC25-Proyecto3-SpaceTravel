# 🚀 Proyecto 3: Space Travel

## 👨‍💻 Autor

**David Domínguez**  
- Proyecto: GPC25 - Proyecto 3 Space Travel

Una simulación 3D completa de un sistema solar ficticio llamado, implementado con un motor de renderizado personalizado en Rust. Explora 10 cuerpos celestes únicos con física orbital realista, navegación 3D y efectos visuales avanzados.

## 📹 Video Demostración

### Proyecto 3 GPC: Space Travel  
[![Ver el vídeo en YouTube](https://img.youtube.com/vi/BWL1YLq7w_Y/0.jpg)](https://youtu.be/BWL1YLq7w_Y)  

> Haz clic en la imagen para ver “Proyecto 3 GPC: Space Travel”.


## ✨ Características Principales

### 🪐 Sistema Solar Completo
- **10 cuerpos celestes** únicos con características y comportamientos distintos
- **Física orbital realista** con rotación y traslación
- **Órbitas anidadas** (lunas orbitando planetas)
- **Planos eclípticos** alineados correctamente

### 🎮 Controles y Navegación
- **Cámara 3D completa** con movimiento libre en todas direcciones
- **Sistema de colisiones** que evita atravesar planetas
- **Nave espacial personalizada** que sigue a la cámara
- **Controles intuitivos** para exploración espacial

### 🎨 Renderizado Avanzado
- **Motor de renderizado personalizado** desde cero
- **Shaders personalizados** para cada cuerpo celeste
- **Sistema de iluminación** con cálculos por fragmento
- **Buffer de profundidad** para ordenamiento 3D correcto

## 🎯 Cuerpos Celestes del Sistema Xerion

| Nombre | Tipo | Características | Color Principal |
|--------|------|-----------------|-----------------|
| **Voidheart** | Singularidad | Centro del sistema, rojo intenso | 🔴 Rojo fuerte |
| **Zephyr** | Planeta gaseoso | Vientos alienígenas, azul cristalino | 🔵 Azul eléctrico |
| **Pyrion** | Planeta volcánico | Volcanes de azufre, superficie ardiente | 🟠 Naranja incandescente |
| **Glacia** | Planeta helado | Hielo alienígena, cristales exóticos | ❄️ Blanco azulado |
| **Umbraleth** | Gigante oscuro | Energía oscura, materia exótica | 🟣 Púrpura oscuro |
| **Verdis** | Planeta boscoso | Bosques bioluminiscentes | 🟢 Verde neón |
| **Crystallos** | Planeta cristalino | Superficie cristalina reflectante | 💎 Azul cristal |
| **Vulcanus** | Luna volcánica | Orbita Umbraleth, actividad volcánica | 🔥 Rojo volcánico |
| **Lunaris** | Luna helada | Orbita Glacia, hielo puro | ⚪ Blanco puro |
| **Stellaris** | Estrella secundaria | Energía verde radioactiva | 💚 Verde radioactivo |

## 🕹️ Controles

### Movimiento de Cámara
- **W/S**: Rotar cámara (arriba/abajo)
- **A/D**: Rotar cámara (izquierda/derecha)
- **Flechas ↑/↓**: Zoom in/out
- **Q/E**: Pan horizontal izquierda/derecha
- **R/F**: Pan vertical arriba/abajo
- **Flechas ←/→**: Pan horizontal alternativo

### Navegación Especial
- **Teclas 1-5**: Warping instantáneo a planetas
  - **1**: Zephyr
  - **2**: Pyrion  
  - **3**: Glacia
  - **4**: Umbraleth
  - **5**: Verdis

## 🛠️ Instalación y Ejecución

### Prerrequisitos
- **Rust** 1.70 o superior
- **Cargo** (incluido con Rust)

### Ejecutar el Proyecto
```bash
# Clonar el repositorio
git clone https://github.com/DavidDominguez-11/GPC25-Proyecto3-SpaceTravel.git
cd GPC25-Proyecto3-SpaceTravel/SpaceTravel

# Ejecutar el proyecto
cargo run --release
```

### Estructura del Proyecto
```
daviddominguez-11-gpc25-proyecto3-spacetravel/
├── README.md
└── SpaceTravel/
    ├── Cargo.toml
    ├── models/
    │   ├── sphere.obj
    │   └── nave.obj
    └── src/
        ├── main.rs          # Punto de entrada principal
        ├── camera.rs        # Sistema de cámara 3D
        ├── matrix.rs        # Operaciones matriciales 4x4
        ├── shaders.rs       # Shaders personalizados
        ├── triangle.rs      # Rasterización de triángulos
        ├── framebuffer.rs   # Buffer de renderizado
        ├── vertex.rs        # Estructuras de vértices
        ├── fragment.rs      # Fragmentos y interpolación
        ├── obj.rs          # Cargador de modelos OBJ
        └── light.rs        # Sistema de iluminación
```

## 🔬 Características Técnicas

### Motor de Renderizado Personalizado
- **Pipeline gráfico completo**: Vertex → Geometry → Rasterization → Fragment
- **Transformaciones 4x4**: Modelo, Vista, Proyección, Viewport
- **Coordenadas baricéntricas** para interpolación suave
- **Depth testing** con buffer Z

### Sistema Físico
- **Movimiento orbital** basado en parámetros reales
- **Rotación axial** independiente por cuerpo celeste
- **Detección de colisiones** esféricas
- **Interpolación de normales** para iluminación suave

### Shaders Personalizados
Cada cuerpo celeste tiene su propio fragment shader con efectos únicos:
- **Sun/Stellaris**: Energía cósmica pulsante
- **Mercury/Lunaris**: Metales y cristales exóticos  
- **Earth/Verdis**: Patrones de vida alienígena
- **Mars/Pyrion**: Terrenos volcánicos y desérticos
- **Nave**: Tecnología futurista con circuitos luminosos

## 📊 Especificaciones de Rendimiento

- **Resolución**: 1280x720 píxeles
- **Triángulos por frame**: ~2,000-5,000
- **FPS objetivo**: 60 FPS (estable en hardware moderno)
- **Optimizaciones**: Bounding box culling, depth testing

## 🎓 Criterios del Proyecto Implementados

### ✅ Completamente Implementado
- [x] **10 cuerpos celestes** (50 puntos)
- [x] **Órbitas renderizadas** (20 puntos) 
- [x] **Cámara 3D completa** (40 puntos)
- [x] **Nave espacial personalizada** (30 puntos)
- [x] **Sistema de colisiones** (10 puntos)
- [x] **Warping instantáneo** (20 puntos)
- [x] **Estética del sistema** (30 puntos estimados)
- [x] **Performance adecuado** (20 puntos estimados)

### 🔄 En Desarrollo
- [ ] Skybox con estrellas (10 puntos)

**Puntuación total estimada: 190/200 puntos**

## 🚀 Características Destacadas

### Sistema de Warping
```rust
// Teletransporte instantáneo a cualquier planeta
if window.is_key_pressed(KeyboardKey::KEY_ONE) {
    implement_warping(&mut camera, &celestial_bodies, "Zephyr", time);
}
```

### Física Orbital Realista
```rust
// Cálculo de posición orbital en tiempo real
body.translation.x = (time * body.orbit_speed).cos() * body.orbit_radius;
body.translation.z = (time * body.orbit_speed).sin() * body.orbit_radius;
```

### Colisiones Esféricas
```rust
fn check_collision(pos1: Vector3, radius1: f32, pos2: Vector3, radius2: f32) -> bool {
    let distance = /* cálculo de distancia 3D */;
    distance < (radius1 + radius2)
}
```

## 🎨 Personalización

Puedes modificar los parámetros orbitales en `main.rs`:

```rust
let planet = CelestialBody {
    name: "NuevoPlaneta".to_string(),
    translation: Vector3::new(0.0, 0.0, 0.0),
    scale: 4.0,
    rotation: Vector3::new(0.0, 0.0, 0.0),
    orbit_radius: 35.0,    // Distancia del centro
    orbit_speed: 0.3,      // Velocidad orbital
    rotation_speed: 1.5,   // Velocidad de rotación
    color: Color::new(255, 100, 150, 255),
};
```

## 🙏 Agradecimientos


- **UVG** por la oportunidad de crear este proyecto
- **GB** por su ayuda

---

**¡Explora el universo!** 🚀✨
