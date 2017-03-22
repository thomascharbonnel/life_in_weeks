extern crate iron;
// This may be overkill, I don't know Rust enough
extern crate time;

use iron::prelude::*;
use iron::status;
use time::{Duration, Timespec, get_time};

// I'd have liked to test duration_to_death by stubbing get_time,
// oh well...
// Every time I cheat on Ruby, I double back!

fn age_in_weeks() -> i64 {
  // 1992-08-30 20:00 CEST
  let birth = Timespec { sec: 715207876, nsec: 0 };

  let current = get_time();

  (current - birth).num_weeks()
}

fn main() {
  let mut html: String = "
<!doctype html>
<html>
  <head>
    <style type="text/css">
      td {
        width: 10px;
        height: 10px;
      }

      .black {
        background-color: black;
      }
    </style>
  </head>
  <body>
    <table>
      <tbody>
  ";

  Iron::new(|_: &mut Request| {
      Ok(Response::with((status::Ok, "Hello World!")))
  }).http("localhost:3000").unwrap();
}
