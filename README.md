# Oxidised Ray Tracing

## Build Instructions
```bash
# renders the final image showcasing nearly all of the features added
cargo run --release -- --img/final_scene.ppm
```
## Features
* **Direct File Output:** Saves `.ppm` files directly to the destination without requiring external shell redirection.
* **Progress Indicator:** Displays a real-time countdown of the remaining lines to be rendered.
* **Dedicated Camera Class:** A separate class for flexible camera management.
* **Diverse Material Support:** Includes dielectrics (glass/water), metals, and matte (diffuse) objects.
* **Antialiasing:** Eliminates sharp, jagged pixel edges for smoother visuals.
* **Defocus Blur:** Optional depth-of-field/defocus blurring effect.
* **Motion Blur:** Simulates moving objects during the shutter interval.
* **Bounding Volumes (BVH):** Uses bounding boxes to accelerate rendering for complex scenes.
* **Image Textures:** Supports mapping `.jpg` images onto objects.
* **Procedural Textures:** Features Perlin noise-based texture generation.
* **Quadrilaterals:** Support for rendering 4-sided flat shapes.
* **Light Sources:** Dedicated light class to support emissive objects.
* **Transformations:** Supports translation and rotation (currently restricted to the Y-axis).
* **Volumetric Media:** Simulates constant-density mediums like fog and smoke.

## Gallery

![First PPM Image](img/SampleImg1.jpg)  
*First PPM Image*

![Sky(Gradient based rendering)](img/SampleImg2.jpg)  
*Sky (Gradient-based rendering)*

![First Sphere](img/SampleImg3.jpg)  
*First Sphere*

![Colorful Sphere](img/SampleImg4.jpg)  
*Colorful Sphere*

![Colorful Sphere with ground](img/SampleImg5.jpg)  
*Colorful Sphere with ground*

![Antialiasing to remove sharp pixel edges](img/SampleImg6.jpg)  
*Antialiasing to remove sharp pixel edges(Zoom this image and previous image to see the difference)*

![Matte material](img/SampleImg7.jpg)  
*Matte material*

![Fixed shadow acne](img/SampleImg9.jpg)  
*Fixed shadow acne*

![Matte material with Lambertian reflection](img/SampleImg10.jpg)  
*Matte material with Lambertian reflection(Notice the darker shadow compared to previous images, this is because rays are almost near the normal)*

![After applying gamma correction](img/SampleImg11.jpg)  
*After applying gamma correction*

![Metal Spheres](img/SampleImg12.jpg)  
*Metal Spheres*

![Fuzz for metal Spheres](img/SampleImg13.jpg)  
*Fuzz for metal spheres*

![Dielectrics](img/SampleImg15.jpg)  
*Dielectrics*

![Total internal reflection](img/SampleImg16.jpg)  
*Total internal reflection*

![Schliks Approcimation](img/SampleImg17.jpg)  
*Schlick's Approximation (If you look at the left sphere carefully to see the difference from previous image)*

![Positionable camera with 90degree fov](img/SampleImg18.jpg)  
*Positionable camera with 90-degree FOV*

![Positionable camera with 20degree fov](img/SampleImg19.jpg)  
*Positionable camera with 20-degree FOV*

![With defocus blur](img/SampleImg20.jpg)  
*With defocus blur*

![Book1 final image](img/final_image.jpg)  
*Ray Tracing in One Weekend (Book 1) final image*

![Book1 final image with motion blurr](img/SampleImg21.jpg)  
*Ray Tracing in One Weekend (Book 1) final image with motion blur*

![Checkerboard ground](img/SampleImg23.jpg)  
*Checkerboard ground*

![2 schered spheres in proximity](img/SampleImg24.jpg)  
*Two checkered spheres in proximity*

![Globe using image texture](img/SampleImg25.jpg)  
*Globe using image texture*

![Perlin noise without interpolation](img/SampleImg26.jpg)  
*Perlin noise without interpolation*

![Perlin noise with trilinear interpolation](img/SampleImg27.jpg)  
*Perlin noise with trilinear interpolation*

![Scale factor of 4 added to perlin noise](img/SampleImg28.jpg)  
*Scale factor of 4 applied to Perlin noise*

![Perlin noise with perlin interpolation](img/SampleImg29.jpg)  
*Perlin noise with Perlin interpolation*

![Added turbulence to perlin noise](img/SampleImg30.jpg)  
*Turbulence added to Perlin noise*

![Marble like texture made with perlin noise and turbulence](img/SampleImg31.jpg)  
*Marble-like texture made with Perlin noise and turbulence*

![Simple scene with quadilaterals](img/SampleImg32.jpg)  
*Simple scene with quadrilaterals*

![Simple rectangular light](img/SampleImg34.jpg)  
*Simple rectangular light source*

![Simple spherical light added](img/SampleImg35.jpg)  
*Simple spherical light source added*

![Empty Cornell Box](img/SampleImg36.jpg)  
*Empty Cornell Box*

![Cornell Box with 2 blocks](img/SampleImg37.jpg)  
*Cornell Box with two blocks*

![Rotated blocks along y axis](img/SampleImg38.jpg)  
*Blocks rotated along the Y-axis*

![Smoke blocks in cornell box](img/SampleImg39.jpg)  
*Smoke blocks inside the Cornell Box (You can see the wall behind black smoke box)*

![Book2 Final Scene Low quality](img/SampleImg40.jpg)  
*Ray Tracing: The Next Week (Book 2) final scene (Low Quality)*

![Book2 Final scene High Quality](img/final_scene.jpg)  
*Ray Tracing: The Next Week (Book 2) final scene (High Quality)*

## Known Issues & Limitations
* **Single-threaded:** The renderer runs entirely on a single thread.
* **BVH Overhead:** Using a bounding volume hierarchy (BVH) does not reduce render times if the scene contains only a few objects.
* **High Render Times:** Producing high-quality images requires a large number of samples per pixel and deep ray recursion depths.
* **Compiler Warnings:** The code compiles with minor warnings (e.g., the project name violates Rust's case conventions and a few unused functions exist).
* **Limited Rotations:** Object rotation is currently only implemented for the Y-axis; X and Z-axis rotation support will be added in a future update.
