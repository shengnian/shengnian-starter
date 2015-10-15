/*
#[macro_use] extern crate nickel;

use nickel::Nickel;
use nickel::StaticFilesHandler;

fn main() {

  let mut server = Nickel::new();

  server.utilize(StaticFilesHandler::new("G:/WebstormProjects/shengnian-starter/build/public/"));


  server.listen("127.0.0.1:6767");
}
*/

#[macro_use]
extern crate nickel;
extern crate mysql;

use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::value::from_row;
use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, MediaType};

use std::collections::HashMap;

struct Blog {
  content: String, //text
  author: String, //varchar(32)
  createtime: String
}

fn save_db<'a, D>(_: &mut Request, res: Response<'a, D>) -> MiddlewareResult<'a, D> {


//  println!("PORT IS {}", port);
  let opts = MyOpts {
    user: Some("root".to_string()),
    pass: Some("yccheng".to_string()),
    ..Default::default()
  };
  let pool = MyPool::new(opts).unwrap();


  let blogs = vec![
    Blog { content: "My second blogpost".to_string(), author: "Mike".to_string(), createtime: "".to_string() },
    Blog { content: "My second blogpost1".to_string(), author: "Mike1".to_string(), createtime: "".to_string() },
    Blog { content: "My second blogpost2".to_string(), author: "Mike2".to_string(), createtime: "".to_string() },
    Blog { content: "My second blogpost3".to_string(), author: "Mike3".to_string(), createtime: "".to_string() },
    Blog { content: "My second blogpost4".to_string(), author: "Mike4".to_string(), createtime: "".to_string() },

  ];

/*  let blog = Blog {
    content: "My second blogpost".to_string(),
    author: "Mike".to_string(),
    createtime: "".to_string()
  };*/


  for mut stmt in pool.prepare(r"INSERT INTO tmp.payment
                                       (content, author)
                                   VALUES
                                       (?, ?)").into_iter() {
    for b in blogs.iter() {
      // `execute` takes ownership of `params` so we pass account name by reference.
      // Unwrap each result just to make sure no errors happended.
      stmt.execute((&b.content, &b.author)).unwrap();
    }
  }

let mut data = HashMap::<&str, String>::new();
data.insert("content","kk".to_string());
data.insert("author", "author".to_string());
data.insert("page_title", "Save blog data".to_string());
res.render("app/views/blog/save.tpl", &data)


 /* //connect to database
  let conn = Connection::connect("mysql://root:root@localhost/json-test", &SslMode::None).unwrap();
  let blog = Blog {
  id: 0,
  content: "My second blogpost".to_string(),
  author: "Mike".to_string(),
  datepost: "".to_string()
  };
  // insert data in DB
      conn.execute("INSERT INTO blogs (content, author) VALUES ($1, $2)",
      &[&blog.content, &blog.author]).unwrap();
  // render in template save.tpl
  let mut data = HashMap::<&str, String>::new();
    data.insert("content", blog.content);
    data.insert("author", blog.author);
    data.insert("page_title", "Save blog data".to_string());
    res.render("app/views/blog/save.tpl", &data)*/


}

//fn get_db_data<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
  /*let conn = Connection::connect("postgres://root:root@localhost/json-test", &SslMode::None).unwrap();
  // select data with condition by id WHERE id = $1
  let stmt = conn.prepare("SELECT id, content, author, datepost FROM blogs WHERE id = $1").unwrap();
  // get request blogid and parse it from &str to i32
  let id: Option<i32> = req.param("blogid").trim().parse::<i32>().ok();
  //run query
  let rows = match stmt.query(&[&id]) {
      Ok(rows) => rows,
      Err(err) => panic!("Error running query: {:?}", err)
    };
  // create init date. Should be new fn in real app
  let mut blog = Blog {
  id: 0,
  content: "".to_string(),
  author: "".to_string(),
  datepost: "".to_string()
  };
  //get data from query and break it becouse we need one row data
  for row in &rows {
    blog = Blog {
    id: row.get(0),
    content: row.get(1),
    author: row.get(2),
    datepost: row.get(3)
    };
    break;
  }
  let mut data = HashMap::<&str, String>::new();
  data.insert("content", blog.content);
  data.insert("author", blog.author);
  data.insert("datepost", blog.datepost);
  data.insert("page_title", "Show blog data".to_string());
  res.render("app/views/blog/show.tpl", &data)*/
//}

fn main() {
  let mut server = Nickel::new();
  // start using router
  let mut router = Nickel::router();

  //http://localhost:8080/test-save-db
  router.get("/test-save-db", save_db);
  //http://localhost:8080/get-blog/1   where 1 is id in your database table
//  router.get("/get-blog/:blogid", get_db_data);

  server.utilize(router);
  server.listen("127.0.0.1:8085");
}
