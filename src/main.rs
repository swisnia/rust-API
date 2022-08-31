use warp::{Filter, Rejection, Reply};//build server
use std::collections::HashMap;
use rand::Rng;

#[tokio::main] //run server asyncrounsly
async fn main() {
    //start the server
    warp::serve(routes())
        .run(([127, 0, 0, 1], 3030))
        .await;
}
//handlers
async fn calculate_diesel_usage_for_distance (params: HashMap<String, i32>) -> Result<impl Reply, Rejection>{
    let distance = params.get("distance").unwrap(); 
    let _year_of_production = params.get("yearOfProduction").unwrap(); 
    let fuel_usage_per_100_km = params.get("fuelUsagePer100KM").unwrap();

    let fuel_usage = distance / 100 * fuel_usage_per_100_km;
    Ok(warp::reply::json(&fuel_usage).into_response())
    
}
async fn probability_of_unit_injector_fail (params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
    let _vin = params.get("VIN").unwrap();
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..100);
    let mut fail_probability = random_number  as f64;
    fail_probability = fail_probability / 100.00;

    Ok(warp::reply::json(&fail_probability).into_response()) 
}
//router
fn routes () -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone{
    diesel_usage_for_distance()
        .or(probability_of_fail())
}
fn diesel_usage_for_distance () -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone{
    warp::get()
        .and(warp::path("calculateDieselUsageForDistance"))
        .and(warp::query::<HashMap<String, i32>>())
        .and(warp::path::end())
        .and_then(calculate_diesel_usage_for_distance)
}
fn probability_of_fail () -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone{
    warp::get()
        .and(warp::path("probabilityOfUnitInjectorFail"))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path::end())
        .and_then(probability_of_unit_injector_fail)
}
