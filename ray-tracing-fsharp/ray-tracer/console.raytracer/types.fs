module types

type Vec3D =   
    { x : double; y: double; z: double} 
with
    static member apply (x, y, z) = { x = x; y = y; z = z }
    static member apply (v : Point3D) = Vec3D.apply(v.x, v.y, v.z)
    static member Zero = Vec3D.apply(0.0, 0.0, 0.0)
    static member X = Vec3D.apply(1.0, 0.0, 0.0)
    static member Y = Vec3D.apply(0.0, 1.0, 0.0)
    static member Z = Vec3D.apply(0.0, 0.0, 1.0)

    static member (~-) (v : Vec3D) =
        { x = -v.x; y = -v.y; z = -v.z }

    static member (+) (lhs: Vec3D, rhs: Vec3D) = 
        { x = lhs.x + rhs.x; y = lhs.y + rhs.y; z = lhs.z + rhs.z }
    static member (-) (lhs: Vec3D, rhs: Vec3D) = 
        { x = lhs.x - rhs.x; y = lhs.y - rhs.y; z = lhs.z - rhs.z }

    static member (*) (lhs: Vec3D, rhs: double) = 
        { x = lhs.x * rhs; y = lhs.y * rhs; z = lhs.z * rhs }
    static member (/) (lhs: Vec3D, rhs: double) = 
        { x = lhs.x / rhs; y = lhs.y / rhs; z = lhs.z / rhs }
    
    static member (*) (lhs: Vec3D, rhs: int) = 
        lhs * (double rhs)
    static member (/) (lhs: Vec3D, rhs: int) = 
        lhs / (double rhs)

    member this.HadamardProduct (other: Vec3D) = 
        { x = this.x * other.x; y = this.y * other.y; z = this.z * other.z }
 
    member this.DotProduct (other: Vec3D) : double = 
        let v = this |-| other
        v.x + v.y + v.z

    // really a 3d pseudovector
    // https://www.youtube.com/watch?v=htYh-Tq7ZBI
    member u.WedgeProduct (v: Vec3D) : Vec3D =
        {
            x = u.y * v.z - u.z * v.y
            y = u.z * v.x - u.x * v.z
            z = u.x * v.y - u.y * v.x
        }

    static member (|-|)(lhs: Vec3D, rhs: Vec3D) = lhs.HadamardProduct rhs
    static member (<.>) (lhs: Vec3D, rhs: Vec3D) = lhs.DotProduct rhs
    static member (<*>) (u: Vec3D, v: Vec3D) = u.WedgeProduct v 

    member v.norm_squared() : double = v <.> v
    member v.norm() : double = (v.norm_squared () |> sqrt)
    member v.normalize() : Vec3D = v / (v.norm ())

    static member lerp (p1: Vec3D, p2 : Vec3D) (t : double) : Vec3D =
        (p1 * (1.0 - t) + p2 * t)
    
    static member (+) (lhs: Vec3D, rhs: Point3D) : Vec3D =
        { 
            x = lhs.x + rhs.x
            y = lhs.y + rhs.y
            z = lhs.z + rhs.z
        }

    static member (-) (lhs: Vec3D, rhs: Point3D) : Vec3D =
        { 
            x = lhs.x - rhs.x
            y = lhs.y - rhs.y
            z = lhs.z - rhs.z
        }

and Point3D = 
    { x : double; y: double; z: double} 
with
    static member apply (x, y, z) = {x = x; y = y; z = z }
    static member apply (v : Vec3D) = Point3D.apply(v.x, v.y, v.z)
    static member Zero = Point3D.apply(0.0, 0.0, 0.0)
    static member Origin = Point3D.Zero
   
    static member (-) (lhs: Point3D, rhs: Point3D) : Vec3D =
        { 
            x = lhs.x - rhs.x
            y = lhs.y - rhs.y
            z = lhs.z - rhs.z
        }

type PPMPixel = { r: uint8; g: uint8; b: uint8 }
with
    override this.ToString() = $"{this.r} {this.g} {this.b}\n"
    static member Zero = { r = 0uy; g = 0uy; b = 0uy }
    static member FromVec3D(other: Vec3D) : PPMPixel =
        {
            r = 255.99 * other.x |> uint8
            g = 255.99 * other.y |> uint8
            b = 255.99 * other.z |> uint8
        }

type PPMCanvas(width: int, height: int) = class
    let (w, h) = (int width, int height)
    let pixels : PPMPixel[,] = Array2D.zeroCreate (int width) (int height)
    
    member val Width  = width  with get
    member val Height = height with get


    member _.Item 
        with get(x, y) = pixels[x, y]
        and set(x, y) v = pixels[x, y] <- v

    member _.Scan compute_pixel =
        for y in 0..h-1 do
            for x in 0..w-1 do
                pixels[x, y] <- compute_pixel pixels[x, y] x y
        
    override this.ToString() =
        let emitPixel (sb: System.Text.StringBuilder) (p: PPMPixel) _ _ =
            ignore <| sb.AppendFormat("{0}", p)
            p

        let sb = new System.Text.StringBuilder()
        
        ignore <| sb.AppendFormat("P3 {0} {1} 255\n", width, height)
        this.Scan (emitPixel sb)

        sb.ToString()

    member public this.WriteToFile(filename) = 
        System.IO.File.WriteAllText(filename, this.ToString())
end

type Ray = 
    { origin: Point3D; direction: Vec3D }
with
    static member apply(origin, direction) = { origin = origin; direction = direction }
    static member (<@>)(lhs: Ray, rhs: double) : Vec3D =
        lhs.direction * rhs + lhs.origin

    member ray.LerpColor (start: Vec3D, finish: Vec3D) =
        (ray.direction.y + 1.0) * 0.5
        |> Vec3D.lerp (start, finish)
        |> PPMPixel.FromVec3D

type Viewport = 
    { 
        width: double
        height: double
        u: Vec3D
        v: Vec3D
        du: Vec3D
        dv: Vec3D
    }
with
    static member apply (height: double) (image: PPMCanvas) =
        let aspect_ratio = (double image.Width / double image.Height)
        let width = height * aspect_ratio
        let u = Vec3D.X * width
        let v = Vec3D.Y * -height
        { 
            width = width
            height = height
            u = u
            v = v
            du = u/image.Width
            dv = v/image.Height
        }

type IHitTestable = interface
    abstract Z : double
    abstract HitTest : Ray -> Option<double>
end

type Sphere(origin: Point3D, radius: double) = class
    member val Origin = origin with get
    member val Radius = radius with get
    interface IHitTestable with
        member this.Z = this.Origin.z
        member this.HitTest (r: Ray) =        
            let oc = r.origin - this.Origin
            let a = r.direction <.> r.direction
            let half_b = oc <.> r.direction
            let c = (oc <.> oc) - (this.Radius * this.Radius)
            let discriminant = half_b * half_b - a * c
            if (discriminant > 0.0) then
                let v = (-half_b - (sqrt discriminant)) / a
                Some v
            else
                None
end

type Scene =
    { 
        CameraLocation: Point3D
        FocalLength: double
        ViewPort: Viewport
        PixelOrigin: Vec3D
        Items : List<IHitTestable>
    }
with
    static member apply (height: double) (focalLength: double) (canvas : PPMCanvas) (items)=
        let viewPort = Viewport.apply height canvas
        let cameraLocation = Point3D.Origin
        let viewPortZOffset = Vec3D.Z * focalLength
        let pixelOrigin = Vec3D.apply cameraLocation - viewPortZOffset - (viewPort.u / 2.0) - (viewPort.v / 2.0)

        {
            CameraLocation = cameraLocation
            FocalLength = focalLength
            ViewPort = viewPort
            PixelOrigin = pixelOrigin
            Items = items 
        }
