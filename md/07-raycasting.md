# Raycasting
Semestre 02, 2025



## Introducción


El Raycasting es una técnica fundamental para la simulación de escenas tridimensionales en entornos 2D.


Exploraremos el concepto de campo de visión (FOV), analizando cómo influye en la forma en que un observador percibe el entorno virtual.



## Raycasting


Raycasting es una técnica de proyección en la que se lanzan rayos desde un punto (usualmente el jugador) hacia el entorno.


Cada rayo verifica colisiones con objetos o paredes para determinar qué es visible y a qué distancia.


A diferencia de un motor 3D completo, el raycasting puede simular una perspectiva tridimensional usando operaciones 2D.



## Aplicaciones


Motores de juegos tempranos como Wolfenstein 3D (1992).


Motores de colisión simples para juegos y simulaciones.


Análisis de línea de visión en IA y juegos de estrategia.



## Proceso de Raycasting


1. Para cada columna vertical de la pantalla, se lanza un rayo.


2. Se calcula la intersección del rayo con el entorno (muro, obstáculo).


3. Se determina la distancia desde el jugador hasta el punto de intersección.


4. Se calcula la altura de la "pared" a dibujar con base en esa distancia (cuanto más lejos, más pequeña).


5. Se dibuja la columna con una textura o color correspondiente.



## ¿Cómo podemos mostrar lo que el jugador está viendo?


Actualmente, un solo rayo nos da información únicamente de lo que hay directamente al frente. ¿Cómo representar todo el campo de visión?


A este concepto le llamamos **field of view**.


![fov0](../assets/img/fov0.png)



## Campo de Visón (FOV)


Podemos visualizar el campo de visión como la dirección hacia donde el jugador está mirando. 


Para simplificar los cálculos, se usa una medida en radianes, como $\frac{\pi}{3}$.


Simulamos un "rayo" que parte desde el jugador y se mueve en línea recta en la dirección del ángulo hasta encontrar una pared.


![trig1](../assets/img/trig1.png)


FOV bajo: mayor "zoom", menor percepción de periferia.


FOV alto: mayor distorsión, pero mejor panorama.



## Cálculo de la Distancia


El plano de proyección es perpendicular a la dirección de vista del jugador.


Cada rayo intersecta este plano a cierta distancia, y esa distancia determina la deformación (corrección de "fisheye").


Lo que nos interesa es determinar la distancia $d$ desde el jugador hasta la pared. Podemos usar identidades trigonométricas sencillas.


$x = d \cdot \cos(a),\ y = d \cdot \sin(a)$

![trig2](../assets/img/trig2.png)



## Usando el FOV


Para simular lo que el jugador ve, lanzamos múltiples rayos abarcando el ángulo total del FOV.


Se lanza un primer rayo en la dirección $a - \frac{fov}{2}$.


Luego incrementamos el ángulo con cada rayo sucesivo hasta $a + \frac{fov}{2}$.


Por ejemplo, si lanzamos 5 rayos, lo hacemos así:

```rust
for i in 0..5 {
    let angle = a - (fov / 2.0) + i as f32 * (fov / 5.0);
    // lanzar rayo en esa dirección
}
```


![FOV](../assets/img/fov1.png)



## Ventajas


Simplicidad: no requiere geometría tridimensional completa.


Velocidad: muy rápido para entornos 2D con simulación 3D.


Bajo consumo de recursos: ideal para sistemas embebidos o retro.



## Limitaciones


No puede renderizar superficies inclinadas ni objetos complejos 3D.


Solo simula muros verticales.


Dificultad para manejar luces y sombras realistas.
