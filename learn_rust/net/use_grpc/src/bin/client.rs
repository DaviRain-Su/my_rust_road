use use_grpc::foobar::*;
use use_grpc::foobar_grpc::*;
use futures::executor;


use grpc::ClientStubExt;

fn main() {

    let client = 
        FooBarServiceClient::new_plain("127.0.0.1",
        9001, Default::default()).unwrap();
    
    let mut req =  CabLocationRequest::new();
    req.set_name("foo".to_string());

    let mut location = Location::new();
    location.latitude = 40.730610;
    location.longitude = -73.935242;
    req.set_Location(location);

    let resp =  client
        .record_cab_location(grpc::RequestOptions::new(), req)
        .join_metadata_result(); // future

    let resp = executor::block_on(resp);

    match resp {
        Err(e) => println!("{:?}", e),
        Ok((_, r, _)) => println!("{:?}", r),
    }

    let mut nearby_req = GetCabRequest::new();
    let mut location = Location::new();
    location.latitude = 40.730610;
    location.longitude = -73.935242;
    nearby_req.set_location(location);

    let nearby_req = client
        .get_cabs(grpc::RequestOptions::new(), nearby_req)
        .join_metadata_result();

    let nearby_req = executor::block_on(nearby_req);

    match nearby_req {
        Err(e) => panic!("{:}", e),
        Ok((_, cabs, _)) => println!("{:?}", cabs),
    }
    
        


}