use clap::Parser;
use rand::Rng;
use std::io::Write;

#[derive(clap::Parser)]
struct Args {
    pairs: u64,
    output_json: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    let start = std::time::Instant::now();
    fast_json_write(args.pairs, &args.output_json).unwrap();
    let time = start.elapsed().as_millis();
    println!("produced {} haversine pairs in {:?}ms", args.pairs, time);    
}

struct Point {
    x: f64,
    y: f64,
}

fn rand_lat(rng: &mut impl Rng) -> f64 {
    rng.gen_range(-90.0..90.0)
}

fn fast_json_write(pairs: u64, output_json: &std::path::Path) -> std::io::Result<()> {
    let mut rng = rand::thread_rng();
    let file = std::fs::File::create(&output_json)?;
    let mut writer = std::io::BufWriter::with_capacity(1024*1024, file);
    writer.write("{ \"pairs\": [\n".as_bytes())?;

    let mut distance_sum = 0.0;
    for _ in 0..pairs - 1 {
        let p0 = Point { x: rng.gen_range(-180.0..180.0), y: rng.gen_range(-90.0..90.0) };
        let p1 = Point { x: rng.gen_range(-180.0..180.0), y: rng.gen_range(-90.0..90.0) };
        distance_sum += haversine_distance(&p0, &p1, 6371.0);
        
        writer.write("    ".as_bytes())?;
        write_pair(&mut writer, &p0, &p1)?;
        writer.write(",\n".as_bytes())?;
    }
    let p0 = Point { x: rng.gen_range(-180.0..180.0), y: rng.gen_range(-90.0..90.0) };
    let p1 = Point { x: rng.gen_range(-180.0..180.0), y: rng.gen_range(-90.0..90.0) };
    distance_sum += haversine_distance(&p0, &p1, 6371.0);
    writer.write("    ".as_bytes())?;
    write_pair(&mut writer, &p0, &p1)?;

    let average_distance = distance_sum / (pairs as f64);
    println!("average distance: {}", average_distance);
    write!(writer, "\n], \"average_distance\": {} }}", average_distance)?;
    Ok(())
}

fn write_pair(writer: &mut impl Write, p0: &Point, p1: &Point) -> std::io::Result<()> {
    writer.write("{ \"x0\": ".as_bytes())?;
    write!(writer, "{}", p0.x)?;
    writer.write(", \"y0\": ".as_bytes())?;
    write!(writer, "{}", p0.y)?;
    writer.write(", \"x1\": ".as_bytes())?;
    write!(writer, "{}", p1.x)?;
    writer.write(", \"y1\": ".as_bytes())?;
    write!(writer, "{}", p1.y)?;
    writer.write(" }".as_bytes())?;
    Ok(())
}


fn haversine_distance(p0: &Point, p1: &Point, radius: f64) -> f64 
{
    let mut lat1 = p0.y;
    let mut lat2 = p1.y;
    let lon1 = p0.x;
    let lon2 = p1.x;
    
    let dLat = (lat2 - lat1).to_radians();
    let dLon = (lon2 - lon1).to_radians();
    lat1 = lat1.to_radians();
    lat2 = lat2.to_radians();
    
    let a = (dLat/2.0).sin().powi(2) + lat1.cos()*lat2.cos()*(dLon/2.0).sin().powi(2);
    let c = 2.0*a.sqrt().asin();
    
    return radius * c;
}