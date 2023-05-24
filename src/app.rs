use  actix_web::{App,web};
use  crate::routers::{supscription::subscribe, ping_controller,user_controller};
use  crate::routers::health_check::health_check;
use log::{info};



pub fn config(cf: &mut web::ServiceConfig) {
    info!("Configuring routes... ");
    
    cf.service(
        web::scope("/api")
                    .service(ping_controller::ping)
                    //.service(user_controller::signup)
                    .service(
                        web::scope("/auth")
                        .service(
                            web::resource("/signup").route(web::post().to(user_controller::signup)),
                           //user_controller::signup
                        )
                        .service(
                            web::resource("/login").route(web::post().to(user_controller::login)),
                        )
                        .service(
                            web::resource("/logout").route(web::post().to(user_controller::logout)),
                        )
                    )
                    // .service(

                    // ),
    );
    
        

  
}
