use ureq;
use std::time::Instant;

fn main() {
   

   let urls: Vec<String> = vec!["https://www.youtube.com/".to_string(),"https://www.google.com/".to_string(), "https://www.chess.com/home".to_string()];

   for url in urls{
   let now = Instant::now();
   //let url = "https://www.youtube.com/";
   let res: ureq::Response = ureq::get(&url).call().unwrap();
   println!(
        "HTTP GET, no path interpolation, no query parameters:\n- URL: {}\n- res code: {},\n",
        url,
        res.status(),
        //res.into_string().unwrap(),
        
   );
   let elapsed = now.elapsed();
   println!("Reponse time: {:.2?}", elapsed);
   if res.status() == 200 {
        println!("No errors encountered!");
   }else{
        println!("ERROR");
   }
}
    //println!("Hello, world!");
}