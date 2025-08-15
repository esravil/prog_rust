use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: String,
    m: String,
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        let r = n % m;
        n = m;
        m = r;
    }
    n
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:8080...");
    server
        .bind("127.0.0.1:8080")? // 3000 is for nodejs
        .run()
        .await
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>GCD Calculator</title>
                <style>
                    body { font-family: Arial, sans-serif; margin: 50px; }
                    form { max-width: 400px; }
                    input, button { padding: 10px; margin: 5px; width: 200px; }
                    button { background-color: #4CAF50; color: white; border: none; cursor: pointer; }
                    .error { color: red; }
                </style>
            </head>
            <body>
                <h1>GCD Calculator</h1>
                <form action="/gcd" method="post">
                    <div>
                        <label>First number:</label><br>
                        <input type="text" name="n" placeholder="Enter first number" required/>
                    </div>
                    <div>
                        <label>Second number:</label><br>
                        <input type="text" name="m" placeholder="Enter second number" required/>
                    </div>
                    <div>
                        <button type="submit">Compute GCD</button>
                    </div>
                </form>
            </body>
            </html>
            "#,
        )
}

async fn post_gcd(form: web::Form<GcdParameters>) -> Result<HttpResponse> {
    let n = match form.n.trim().parse::<u64>() {
        Ok(num) => num,
        Err(_) => {
            return Ok(HttpResponse::BadRequest()
                .content_type("text/html")
                .body(format!(
                    r#"
                    <!DOCTYPE html>
                    <html>
                    <body>
                        <h1>Error</h1>
                        <p class="error">Invalid input: '{}' is not a valid positive number.</p>
                        <a href="/">Go back</a>
                    </body>
                    </html>
                    "#,
                    form.n
                )));
        }
    };

    let m = match form.m.trim().parse::<u64>() {
        Ok(num) => num,
        Err(_) => {
            return Ok(HttpResponse::BadRequest()
                .content_type("text/html")
                .body(format!(
                    r#"
                    <!DOCTYPE html>
                    <html>
                    <body>
                        <h1>Error</h1>
                        <p class="error">Invalid input: '{}' is not a valid positive number.</p>
                        <a href="/">Go back</a>
                    </body>
                    </html>
                    "#,
                    form.m
                )));
        }
    };

    if n == 0 && m == 0 {
        return Ok(HttpResponse::BadRequest()
            .content_type("text/html")
            .body(
                r#"
                <!DOCTYPE html>
                <html>
                <body>
                    <h1>Error</h1>
                    <p class="error">Both numbers cannot be zero.</p>
                    <a href="/">Go back</a>
                </body>
                </html>
                "#,
            ));
    }

    let result = gcd(n, m);
    
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>GCD Result</title>
                <style>
                    body {{ font-family: Arial, sans-serif; margin: 50px; }}
                    .result {{ font-size: 24px; color: #4CAF50; margin: 20px 0; }}
                </style>
            </head>
            <body>
                <h1>GCD Calculator Result</h1>
                <p>The greatest common divisor of {} and {} is:</p>
                <p class="result">{}</p>
                <a href="/">Calculate another GCD</a>
            </body>
            </html>
            "#,
            n, m, result
        )))
}