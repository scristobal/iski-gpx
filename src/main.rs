use geo_types::{coord, Point};
use gpx::{Gpx, GpxVersion, Track, TrackSegment, Waypoint};
use std::{error::Error, fs::File, io::BufWriter};

fn main() -> Result<(), Box<dyn Error>> {
    let sample_file = include_str!("../sample.json");

    let json: serde_json::Value =
        serde_json::from_str(sample_file).expect("file should be proper JSON");

    let sample_track = json
        .get("track")
        .expect("file should have track key")
        .to_string()
        .replace('\"', "");

    let track_segment = TrackSegment { points: vec![] };

    let track = Track {
        name: Some("Sample Track".to_string()),
        comment: None,
        description: None,
        source: None,
        links: vec![],
        type_: None,
        number: None,
        segments: vec![track_segment],
    };
    let mut gpx = Gpx {
        version: GpxVersion::Gpx11,
        creator: None,
        metadata: None,
        waypoints: vec![],
        tracks: vec![track],
        routes: vec![],
    };

    let gpx_file = File::create("sample_out2.gpx")?;
    let buf = BufWriter::new(gpx_file);

    sample_track.split(' ').for_each(|data_point| {
        let data = data_point.split(',').collect::<Vec<_>>();

        let geo_coord = coord! { x: data[0].parse().unwrap(), y: data[1].parse().unwrap() };
        let geo_point: Point = geo_coord.into();

        let waypoint = Waypoint::new(geo_point);

        gpx.tracks[0].segments[0].points.push(waypoint);
    });

    gpx::write(&gpx, buf)?;

    Ok(())
}
