use ureq;
use std::time::Instant;
use std::thread;

fn main() {
    let totaltime = Instant::now();

    let urls: Vec<String> = vec![
        "https://www.youtube.com/".to_string(),
        "https://www.google.com/".to_string(),
        "https://www.chess.com/home".to_string(),
        "https://leetcode.com/".to_string(),
    ];

    let mut handles = vec![];

    for url in urls {
        let handle = thread::spawn(move || {
            let now = Instant::now();

            
            let res = ureq::get(&url).call();

            match res {
                Ok(response) => {
                    
                    println!(
                        "HTTP GET, no path interpolation, no query parameters:\n- URL: {}\n- res code: {}",
                        url,
                        response.status()
                    );

                    let elapsed = now.elapsed();
                    println!("Response time for {}: {:.2?}", url, elapsed);

                    if response.status() == 200 {
                        println!("No errors encountered!\n\n");
                    } else {
                        println!("ERROR with status code: {}\n\n", response.status());
                    }
                }
                Err(e) => {
                   
                    println!("Failed to request URL {}: {}", url, e);
                }
            }
        });

        handles.push(handle);
    }

 
    for handle in handles {
        handle.join().unwrap();
    }

    let totalelapsed = totaltime.elapsed();
    println!("Total time taken: {:.2?}", totalelapsed);
}