// Using the chrono crate to get the LocalTime
use chrono::{Timelike, Local};

// Constants to convert Radians to Degrees and vice-versa
const RAD_TO_DEG:f64 = 57.2958;
const DEG_TO_RAD:f64 = 0.0174533;

#[no_mangle]
pub extern fn get_second_hand(r: f64, origin_x: f64, origin_y: f64) -> f64{

    let seconds: f64 = Local::now().second() as f64;

    // R sin(theta) + y origin of the circle = x coordinate of a point on the circle for given theta
    // theta (0-360), in this case we are multiplying seconds * 6 therefore becomes(0 - 360)
    let x = r * (seconds * 6. * DEG_TO_RAD).sin()   + origin_y;
    let y = r * (seconds * 6. * DEG_TO_RAD).cos()  + origin_x;

    // We need the angle between the origin and the (x,y) point on the circle so we can
    // rotate the clock's second hand

    // Atan2 outputs in Radians so we have to convert it to degrees
    let angle = y.atan2(x) * RAD_TO_DEG;
    return angle;
}

#[no_mangle]
pub extern fn get_minute_hand(r: f64, origin_x: f64, origin_y: f64) -> f64{

    let minutes: f64 = Local::now().minute() as f64;

    // R sin(theta) + y origin of the circle = x coordinate of a point on the circle for given theta
    // theta (0-360), in this case we are multiplying minutes * 6 therefore becomes(0 - 360)
    let x = r * (minutes * 6. * DEG_TO_RAD).sin() + origin_y;
    let y = r * (minutes * 6. * DEG_TO_RAD).cos() + origin_x;

    // We need the angle between the origin and the (x,y) point on the circle so we can
    // rotate the clock's second hand

    // Atan2 outputs in Radians so we have to convert it to degrees
    let angle = y.atan2(x) * RAD_TO_DEG;
    return angle;
}


#[no_mangle]
pub extern fn get_hour_hand(r: f64, origin_x: f64, origin_y: f64) -> f64{

    let hour: f64 = Local::now().hour() as f64;

    // R sin(theta) + y origin of the circle = x coordinate of a point on the circle for given theta
    // theta (0-360), in this case we are multiplying hour * 30 therefore becomes(30..60..360)
    let x = r * (hour * 30. * DEG_TO_RAD).sin() + origin_y;
    let y = r * (hour * 30. * DEG_TO_RAD).cos() + origin_x;

    // We need the angle between the origin and the (x,y) point on the circle so we can
    // rotate the clock's second hand

    // Atan2 outputs in Radians so we have to convert it to degrees
    let angle = y.atan2(x) * RAD_TO_DEG;
    return angle;
}
