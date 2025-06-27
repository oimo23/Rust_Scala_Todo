ThisBuild / scalaVersion := "2.13.8"
ThisBuild / version := "0.1.0"

lazy val root = (project in file("."))
  .enablePlugins(JavaAppPackaging)
  .settings(
    name := "todo-scala",
    libraryDependencies ++= Seq(
      "com.typesafe.akka" %% "akka-http" % "10.2.9",
      "com.typesafe.akka" %% "akka-stream" % "2.6.19",
      "com.typesafe.akka" %% "akka-actor-typed" % "2.6.19",
      "de.heikoseeberger" %% "akka-http-circe" % "1.39.2",
      "io.circe" %% "circe-core" % "0.14.1",
      "io.circe" %% "circe-generic" % "0.14.1",
      "io.circe" %% "circe-parser" % "0.14.1",
      "ch.qos.logback" % "logback-classic" % "1.2.11",
      "org.scalatest" %% "scalatest" % "3.2.12" % Test
    )
  )