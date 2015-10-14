package main

import (
  "net/http"
  "os"
)

func main() {

//  router := gin.Default()
//  router.Run(":8080")
  port := os.Getenv("PORT")
  if port == "" {
    port = "8082"
  }

  http.Handle("/", http.FileServer(http.Dir("../build/public")))
  http.ListenAndServe(":" + port, nil)
}
