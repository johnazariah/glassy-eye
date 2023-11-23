open types

let generateRedAndGreenGradient(width : int, height : int, filename) =
    let image = PPMCanvas(width * 1, height * 1)
    let PPM_X_RATIO = 256.0 / double width
    let PPM_Y_RATIO = 256.0 / double height
    let ppm_scale_x x = (double x) * PPM_X_RATIO |> uint8
    let ppm_scale_y y = (double y) * PPM_Y_RATIO |> uint8
    
    image.Scan (fun _ x y -> { PPMPixel.Zero with r = ppm_scale_x x; g = ppm_scale_y y})
    image.WriteToFile filename

let generateGreenAndBlueGradient(width : int, height : int, filename) =
    let image = PPMCanvas(width * 1, height * 1)
    let PPM_X_RATIO = 256.0 / double width
    let PPM_Y_RATIO = 256.0 / double height
    let ppm_scale_x x = (double x) * PPM_X_RATIO |> uint8
    let ppm_scale_y y = (double y) * PPM_Y_RATIO |> uint8
    
    image.Scan (fun _ x y -> { PPMPixel.Zero with g = ppm_scale_x x; b = ppm_scale_y y})
    image.WriteToFile filename

let generateSceneRays(filename) =
    let white = Vec3D.apply(1.0, 1.0, 1.0)
    let sky_blue = Vec3D.apply(0.5, 0.7, 1.0)

    let image = PPMCanvas(800, 600)
    let scene = Scene.apply (2.0) (1.0) image []

    image.Scan (fun _ i j -> 
        let pixel_center = scene.PixelOrigin + (scene.ViewPort.du * double i) + (scene.ViewPort.dv * double j)
        let ray_direction = pixel_center - scene.CameraLocation
        Ray.apply(scene.CameraLocation, ray_direction.normalize()).LerpColor(white, sky_blue))

    image.WriteToFile filename

let simpleRedSphere(filename) =
    let red = Vec3D.apply(1.0, 0.0, 0.0)
    let white = Vec3D.apply(1.0, 1.0, 1.0)
    let sky_blue = Vec3D.apply(0.5, 0.7, 1.0)

    let image = PPMCanvas(800, 600)
    let sphere = Sphere(Point3D.apply(0.0, 0.0, -1.0), 0.5)

    let scene = Scene.apply (2.0) (1.0) image [sphere]

    let ray_color(ray : Ray) =
        match (scene.Items[0].HitTest ray) with
        | Some _ -> red |> PPMPixel.FromVec3D
        | None -> ray.LerpColor(white, sky_blue)

    image.Scan (fun _ i j -> 
        let pixel_center = scene.PixelOrigin + (scene.ViewPort.du * double i) + (scene.ViewPort.dv * double j)
        let ray_direction = pixel_center - scene.CameraLocation
        let ray = Ray.apply(scene.CameraLocation, ray_direction.normalize())
        ray_color ray)

    image.WriteToFile filename        

let surfaceNormalShadedSphere(filename) =
    let offset = Vec3D.apply(1.0, 1.0, 1.0) + Point3D.apply(0.0, 0.0, -1.0)

    let white = Vec3D.apply(1.0, 1.0, 1.0)
    let sky_blue = Vec3D.apply(0.5, 0.7, 1.0)

    let image = PPMCanvas(800, 600)
    let sphere = Sphere(Point3D.apply(0.0, 0.0, -1.0), 0.5)

    let scene = Scene.apply (2.0) (1.0) image [sphere]

    let ray_color(ray : Ray) =
        match (scene.Items[0].HitTest ray) with
        | Some t -> ((ray <@> t) + offset) * 0.5 |> PPMPixel.FromVec3D
        | None -> ray.LerpColor(white, sky_blue)

    image.Scan (fun _ i j -> 
        let pixel_center = scene.PixelOrigin + (scene.ViewPort.du * double i) + (scene.ViewPort.dv * double j)
        let ray_direction = pixel_center - scene.CameraLocation
        let ray = Ray.apply(scene.CameraLocation, ray_direction.normalize())
        ray_color ray)

    image.WriteToFile filename        

generateGreenAndBlueGradient(1024, 768, "green-blue.ppm")
generateSceneRays("rays.ppm")
simpleRedSphere("simple_red_sphere.ppm")
surfaceNormalShadedSphere("surfaceNormalShadedSphere.ppm")