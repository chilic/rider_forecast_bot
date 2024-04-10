use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Point {
    properties: PointProperties,
}

#[derive(Deserialize, Debug)]
struct PointProperties {
    #[serde(alias = "gridId")]
    grid_id: String,
    #[serde(alias = "gridX")]
    grid_x: u32,
    #[serde(alias = "gridY")]
    grid_y: u32,
    forecast: String,
    #[serde(alias = "forecastHourly")]
    forecast_hourly: String,
}

struct Client {
    client: reqwest::Client,
    base_url: &'static str,
}

fn build_client() -> Client {
    Client {
        client: reqwest::Client::builder()
            .user_agent("Rider Forecast")
            .build()
            .unwrap(),
        base_url: "https://api.weather.gov",
    }
}

impl Client {
    async fn get_forecast(self) -> Result<(), Box<dyn std::error::Error>> {

        Ok(())
    }

    async fn get_point(self, lat: f32, long: f32) -> Result<Point, Box<dyn std::error::Error>> {
        let url = format!("{0}/points/{1},{2}", self.base_url, lat, long);
        let point = self.client.get(url).send().await?.json::<Point>().await?;
        Ok(point)
    }

    async fn get_gridpoint_forecast(self, point: &Point) -> Result<(), Box<dyn std::error::Error>> {
         
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::wather::build_client;

    #[tokio::test]
    async fn it_works() {
        let c = build_client();
        let point = c.get_point(29.7949, -95.8165).await.unwrap();
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
