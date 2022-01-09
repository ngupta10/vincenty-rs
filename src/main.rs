use std::str::FromStr;

use tide::prelude::*;
mod vincenty;

#[derive(Deserialize, Debug)]
#[serde(default)]
struct Query {
    src: String,
    dst: String
}

#[derive(Debug, Serialize, Clone)]
struct GeoCoordinate {
    lat: f64,
    lng: f64
}

impl FromStr for GeoCoordinate {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (src, dst) = s.split_once(",").expect("Incorrect format!");

        Ok(GeoCoordinate {
            lat: src.trim().parse().unwrap(),
            lng: dst.trim().parse().unwrap(),
        })
    }
}

impl Default for Query {
    fn default() -> Self {
        Self {
            src: "".to_string(),
            dst: "".to_string()
        }
    }
}

impl From<GeoCoordinate> for (f64, f64) {
    fn from(c: GeoCoordinate) -> (f64, f64) {
        let GeoCoordinate {lat, lng} = c;
        (lat, lng)
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/distance").post(|req: tide::Request<()>| async move {
        let q: Query = req.query()?;
        let c1: GeoCoordinate = GeoCoordinate::from_str(&q.src)?;
        let c2: GeoCoordinate = GeoCoordinate::from_str(&q.dst)?;
        let l1: vincenty::Coordinate = c1.clone().into();
        let l2: vincenty::Coordinate = c2.clone().into();
        Ok(json!({"data": {"src": c1, "dst": c2, "distance": vincenty::distance(l1, l2)}}))
    });

    app.listen("localhost:5000").await?;

    Ok(())

}
